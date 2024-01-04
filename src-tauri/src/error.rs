use std::io;
use serde::Serializer;

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("JSON Error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("Zip Error: {0}")]
    Zip(#[from] zip::result::ZipError),

    #[error("IO Error: {0}")]
    IO(#[from] io::Error),

    #[error("{0}")]
    Message(String)
}

impl From<&str> for Error {
    fn from(value: &str) -> Self {
        Error::Message(value.to_string())
    }
}

impl From<String> for Error {
    fn from(value: String) -> Self {
        Error::Message(value)
    }
}

impl serde::Serialize for Error {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        serializer.serialize_str(self.to_string().as_ref())
    }
}