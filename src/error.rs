use derive_more::derive::From;
use serde_json::error::Error as SerdeError;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(From, Debug)]
pub enum Error {
    IoError(std::io::Error),
    StrError(String),
    SerdeJson(SerdeError),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "An error occurred")
    }
}

impl std::error::Error for Error {}
