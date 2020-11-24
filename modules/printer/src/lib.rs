use wdt_module_api::{Module, ValidationError};
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[cfg(target_os = "windows")]
mod windows;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PrinterModule {
    pub printers: Vec<PrinterConfig>
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Printer {
    pub name: String,
    #[serde(flatten)]
    pub config: PrinterConfig
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "type")]
pub enum PrinterConfig {
    #[serde(rename = "ipp")]
    IPP {
        url: String
    }
}

impl Module for PrinterModule {
    type Error = PrinterModuleError;
    type State = ();

    fn apply(&self, previous_state: Option<Self::State>) -> Result<Self::State, Self::Error> {
        Ok(())
    }

    fn validate(&self) -> Result<(), Vec<ValidationError>> {
        unimplemented!()
    }
}

#[derive(Error, Debug)]
pub enum PrinterModuleError {
}
