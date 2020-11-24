#[cfg(target_os = "windows")]
mod windows;

use wdt_module_api::{Module, ValidationError};
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SoftwareModule {
    msi: Option<String>
}

impl Module for SoftwareModule {
    type Error = SoftwareModuleError;
    type State = ();

    #[cfg(target_os = "windows")]
    fn apply(&self, previous_state: Option<Self::State>) -> Result<Self::State, Self::Error> {
        if let Some(ref msi) = self.msi {
            crate::windows::install_msi(msi)?;
        }else {
            wdt_logging::warn!("No installer available");
        }
        Ok(())
    }

    #[cfg(not(target_os = "windows"))]
    fn apply(&self, previous_state: Option<Self::State>) -> Result<Self::State, Self::Error> {
        wdt_logging::warn!("No installer available");
        Ok(())
    }

    fn validate(&self) -> Result<(), Vec<ValidationError>> {
        unimplemented!()
    }
}

#[derive(Error, Debug)]
pub enum SoftwareModuleError {
    #[error("IO Error {0}")]
    Io(#[from] std::io::Error)
}
