use crate::events;
use crate::logger;
use crate::modules::{Context, Module};
use crate::session::Session;

use super::NoiseLevel;

pub struct ModuleReady {}

impl Default for ModuleReady {
    fn default() -> Self {
        Self::new()
    }
}

impl ModuleReady {
    pub fn new() -> Self {
        ModuleReady {}
    }
}

impl Module for ModuleReady {
    fn name(&self) -> String {
        String::from("ready")
    }

    fn noise_level(&self) -> NoiseLevel {
        NoiseLevel::None
    }

    fn description(&self) -> String {
        String::from(
            "This module is responsible to know when Project Absence is ready and will start to do the work",
        )
    }

    fn subscribers(&self) -> Vec<events::Type> {
        vec![events::Type::Ready]
    }

    fn execute(&self, _: &Session, _: Context) -> Result<(), String> {
        logger::println(
            "ready",
            "Project Absence is now ready and will start doing its magic!",
        );

        Ok(())
    }
}
