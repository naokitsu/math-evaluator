use std::fmt::{Debug, Display, Formatter};

#[derive(Debug, PartialEq)]
pub enum Error {
    UninitializedVariable(String),
    InvalidSyntax,
    CanOnlyAssignToVariable,
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
            Error::CanOnlyAssignToVariable => {
                write!(f, "Can only assign to variable")
            }
        }
    }
}

impl std::error::Error for Error {}