use std::fmt::Display;

#[derive(PartialEq, Debug)]
pub enum Error {
    InvalidInput(String),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::InvalidInput(s) => s,
        })
    }
}

pub type Result<T> = std::result::Result<T, Error>;