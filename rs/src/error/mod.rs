use derive_more::Display;

use std::io::Error as IoError;
use serde_json::Error as SerdeJsonError;
use json::Error as JsonError;

#[derive(Debug, Display)]
pub enum KittyMuxError {
    SerdeJsonError(SerdeJsonError),
    IoError(IoError),
    JsonError(JsonError)
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
