use std::fmt::{Debug, Display, Formatter};

#[derive(Debug)]
pub enum Error {
    UninitializedVariable(String),
    InvalidSyntax
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self {
            Error::UninitializedVariable(name) => {
                write!(f, "Uninitialized variable: {}", name)
            }
            Error::InvalidSyntax => {
                write!(f, "Invalid syntax")
            }
        }
    }
}

impl std::error::Error for Error {}