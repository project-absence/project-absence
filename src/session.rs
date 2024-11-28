use std::fs::{create_dir_all, File};
use std::io::{Error, Write};
use std::path::PathBuf;
use std::sync::{Arc, Mutex, MutexGuard};
use std::{env, thread};

use clipboard::{ClipboardContext, ClipboardProvider};
use flume::{Receiver, Sender};
use reqwest::blocking::Client;

use crate::modules::Module;
use crate::{args, config, database, debug, events, logger, modules, state};

pub struct Session {
    args: args::Args,
    config: config::Config,
    database: Arc<Mutex<database::Database>>,
    state: Arc<state::State>,
    http_client: Client,

    sender: Sender<events::Type>,
    receiver: Receiver<events::Type>,

    modules: Mutex<Vec<Arc<Box<dyn Module>>>>,
}

impl Session {
    pub fn new(
        args: args::Args,
        config: config::Config,
        sender: Sender<events::Type>,
        receiver: Receiver<events::Type>,
    ) -> Arc<Self> {
        let domain_clone = args.clone().domain;
        let is_verbose = args.verbose;
        let is_debug = args.debug;
        Arc::new(Session {
            args,
            config,
            database: Arc::new(Mutex::new(database::Database::new(
                database::node::Node::new(database::node::Type::Hostname, domain_clone),
            ))),
            state: Arc::new(state::State::new(is_verbose, is_debug)),
            http_client: Client::new(),

            sender,
            receiver,

            modules: Mutex::new(Vec::new()),
        })
    }

    pub fn get_args(&self) -> &args::Args {
        &self.args
    }

    pub fn get_config(&self) -> &config::Config {
        &self.config
    }

    pub fn get_database(&self) -> MutexGuard<database::Database> {
        self.database.lock().unwrap()
    }

    pub fn get_state(&self) -> Arc<state::State> {
        Arc::clone(&self.state)
    }

    pub fn get_http_client(&self) -> &Client {
        &self.http_client
    }

    pub fn register_module<T: Module + Send + Sync + 'static>(&self, module: T) {
        if self.get_state().is_debug_or_verbose() {
            logger::info("", format!("Registered module {}", module.name()))
        }
        let mut modules = self.modules.lock().unwrap();
        modules.push(Arc::new(Box::new(module)));
    }

    pub fn register_config_modules(&self) {
        self.register_module(modules::ready::ModuleReady::new());
        if self.config.enumerate_files.enabled {
            self.register_module(modules::enumerate_files::ModuleEnumerateFiles::new());
        }
        if self.config.enumerate_subdomains.enabled {
            self.register_module(modules::enumerate_subdomains::ModuleEnumerateSubdomains::new());
        }
        if self.config.passive_dns.enabled {
            self.register_module(modules::passive_dns::ModulePassiveDNS::new());
        }
        if self.config.port_scanner.enabled {
            self.register_module(modules::port_scanner::ModulePortScanner::new());
        }
    }

    pub fn emit(&self, event: events::Type) {
        let event_name = event.to_string();
        if let Err(e) = self.sender.send(event) {
            logger::error(
                "emit",
                format!("Failed emitting the '{}' event: {}", event_name, e),
            );
        };
    }

    pub fn run(self: Arc<Self>) -> Result<(), Error> {
        self.emit(events::Type::Ready);
        self.emit(events::Type::DiscoveredDomain(
            self.get_args().domain.clone(),
        ));

        while let Ok(event) = self.receiver.recv() {
            if event == events::Type::FinishedTask {
                self.get_state().decrement_tasks();
                if self.get_state().is_debug() {
                    logger::debug(
                        event.to_string(),
                        format!(
                            "A task has finished execution, current running tasks: {}",
                            self.get_state().active_tasks_count()
                        ),
                    );
                }
                if self.get_state().active_tasks_count() == 0 {
                    if self.get_args().clipboard {
                        let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
                        if ctx
                            .set_contents(self.get_database().get_as_pretty_json())
                            .is_ok()
                        {
                            logger::info(
                                "",
                                "Successfully copied the resulting JSON database to the clipboard",
                            )
                        }
                    }

                    if self.get_state().is_debug() {
                        debug::database::render_compact(&mut self.get_database());
                    }

                    let home_dir = env::var("HOME")
                        .or_else(|_| env::var("USERPROFILE"))
                        .unwrap_or_else(|_| String::from(""));
                    let result_path = &self.get_args().file;
                    let expanded_result_path = if result_path.starts_with("~") {
                        let mut expanded_path = result_path.clone();
                        expanded_path.replace_range(0..1, &home_dir);
                        PathBuf::from(expanded_path)
                    } else {
                        PathBuf::from(result_path)
                    };
                    if create_dir_all(expanded_result_path.parent().unwrap()).is_ok() {
                        let mut file_result = File::create(expanded_result_path.clone())?;
                        if file_result
                            .write_all(self.get_database().get_as_pretty_json().as_bytes())
                            .is_ok()
                        {
                            logger::info(
                                "",
                                format!(
                                    "Successfully wrote the resulting JSON database in '{}'",
                                    expanded_result_path.display()
                                ),
                            )
                        };
                    }

                    break;
                }
            }

            let modules = self.modules.lock().unwrap();
            for module in &*modules {
                if module.subscribers().iter().any(|sub_event| {
                    matches!(
                        (sub_event, &event),
                        (events::Type::Ready, events::Type::Ready)
                            | (
                                events::Type::DiscoveredDomain(_),
                                events::Type::DiscoveredDomain(_)
                            )
                    )
                }) {
                    thread::spawn({
                        self.get_state().increment_tasks();
                        let permit = self.get_state().get_semaphore_permit();
                        let context = modules::get_context_for_event(&event);
                        let module_clone = Arc::clone(module);
                        let session_clone = Arc::clone(&self);
                        move || {
                            if let Err(e) = module_clone.execute(&session_clone, context) {
                                logger::error(module_clone.name(), e)
                            }
                            session_clone.emit(events::Type::FinishedTask);
                            drop(permit);
                        }
                    });
                }
            }
        }

        Ok(())
    }
}
