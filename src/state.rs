use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc, Mutex,
};

use simple_semaphore::Permit;

use crate::logger;

pub struct State {
    active_tasks: Arc<AtomicUsize>,
    semaphore: Arc<simple_semaphore::Semaphore>,
    verbose: bool,
    debug: bool,

    discovered_subdomains: Mutex<Vec<String>>,
}

impl State {
    pub fn new(verbose: bool, debug: bool) -> Self {
        State {
            active_tasks: Arc::new(AtomicUsize::new(0)),
            semaphore: simple_semaphore::Semaphore::default(),
            verbose,
            debug,

            discovered_subdomains: Mutex::new(Vec::new()),
        }
    }

    pub fn increment_tasks(&self) {
        self.active_tasks.fetch_add(1, Ordering::SeqCst);
        if self.is_debug() {
            logger::debug(
                "state",
                format!(
                    "The running task counter has incremented, current running tasks: {}",
                    self.active_tasks_count()
                ),
            );
        }
    }

    pub fn decrement_tasks(&self) {
        self.active_tasks.fetch_sub(1, Ordering::SeqCst);
        if self.is_debug() {
            logger::debug(
                "state",
                format!(
                    "The running task counter has decremented, current running tasks: {}",
                    self.active_tasks_count()
                ),
            );
        }
    }

    pub fn active_tasks_count(&self) -> usize {
        self.active_tasks.load(Ordering::SeqCst)
    }

    pub fn get_semaphore_permit(&self) -> Permit {
        self.semaphore.acquire()
    }

    pub fn is_debug(&self) -> bool {
        self.debug
    }

    pub fn is_verbose(&self) -> bool {
        self.verbose
    }

    pub fn is_debug_or_verbose(&self) -> bool {
        self.is_debug() || self.is_verbose()
    }

    pub fn discover_subdomain(&self, subdomain: String) {
        self.discovered_subdomains.lock().unwrap().push(subdomain)
    }

    pub fn has_discovered_subdomain(&self, subdomain: String) -> bool {
        self.discovered_subdomains
            .lock()
            .unwrap()
            .contains(&subdomain)
    }
}
