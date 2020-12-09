use wdt_module_api::{Module, ValidationError};
use thiserror::Error;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct WireguardModule {

}

impl Module for WireguardModule {
    type Error = WireguardModuleError;
    type State = ();

    fn apply(&self, previous_state: Option<Self::State>) -> Result<Self::State, Self::Error> {
        unimplemented!()
    }

    fn validate(&self) -> Result<(), Vec<ValidationError>> {
        unimplemented!()
    }
}

#[derive(Error, Debug)]
pub enum WireguardModuleError {

}
