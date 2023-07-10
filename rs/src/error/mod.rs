use derive_more::Display;

use core::fmt;
use std::io::Error as IoError;
use serde_json::Error as SerdeJsonError;
use json::Error as JsonError;

#[derive(Debug, Display)]
pub enum KittyMuxError {
    SerdeJsonError(SerdeJsonError),
    IoError(IoError),
    JsonError(JsonError),
    MissingArgumentError(MissingArgumentError),
    InvalidWindowIdError(InvalidWindowIdError),

}


impl From<SerdeJsonError> for KittyMuxError {
    fn from(err: SerdeJsonError) -> Self {
        KittyMuxError::SerdeJsonError(err)
    }
}

impl From<IoError> for KittyMuxError {
    fn from(err: IoError) -> Self {
        KittyMuxError::IoError(err)
    }
}

impl From<JsonError> for KittyMuxError {
    fn from(err: JsonError) -> Self {
        KittyMuxError::JsonError(err)
    }
}

#[derive(Debug)]
pub struct MissingArgumentError {
    pub arugment: String,
}

impl fmt::Display for MissingArgumentError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error: missing argument: {}", self.arugment)
    }
}

impl From<MissingArgumentError> for KittyMuxError {
    fn from(err: MissingArgumentError) -> Self {
        KittyMuxError::MissingArgumentError(err)
    }
}

// Error for invalid window id
#[derive(Debug)]
pub struct InvalidWindowIdError {
    pub window_id: u32,
}

impl fmt::Display for InvalidWindowIdError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error: invalid window id: {}", self.window_id)
    }
}

impl From<InvalidWindowIdError> for KittyMuxError {
    fn from(err: InvalidWindowIdError) -> Self {
        KittyMuxError::InvalidWindowIdError(err)
    }
}
