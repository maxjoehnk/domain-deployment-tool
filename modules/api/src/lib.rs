use std::ops::Range;
use serde::{Serialize, de::DeserializeOwned};

pub trait Module : std::fmt::Debug {
    type Error;
    type State : Serialize + DeserializeOwned;

    fn apply(&self, previous_state: Option<Self::State>) -> Result<Self::State, Self::Error>;

    fn validate(&self) -> Result<(), Vec<ValidationError>>;
}

pub struct ValidationError {
    pub file: String,
    pub lines: Option<Range<u16>>,
    pub message: String
}
