use derive_more::derive::From;
use serde_json::error::Error as SerdeError;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(From, Debug)]
pub enum Error {
    Io(std::io::Error),
    Str(String),
    Serde(SerdeError),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::Io(err) => write!(f, "Io Error: {}", err),
            Error::Str(err) => write!(f, "{}", err),
            Error::Serde(err) => write!(f, "Serde JSON Error: {}", err),
        }
    }
}

impl std::error::Error for Error {}
