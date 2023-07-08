use derive_more::Display;

use std::io::Error as IoError;
use serde_json::Error as SerdeJsonError;

#[derive(Debug, Display)]
pub enum KittyMuxError {
    SerdeJsonError(SerdeJsonError),
    IoError(IoError)
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
