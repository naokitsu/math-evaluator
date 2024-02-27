use std::fmt::{Debug, Display};
use crate::error::Error;
use crate::state::State;

#[derive(Debug, Clone)]
pub(crate) enum Node {
    Variable {
        name: String,
    },
    Constant {
        value: i32,
    },
    Unary {
        child: Box<Node>,
        strategy: fn(child: &Box<Node>, &State) -> Result<i32, Error>,
    },
    Binary {
        left: Box<Node>,
        right: Box<Node>,
        strategy: fn(left: &Box<Node>, right: &Box<Node>, &State) -> Result<i32, Error>,
    },
}

impl Node {
    pub(crate) fn calculate(&self, state: &State) -> Result<i32, Error> {
        match self {
            Node::Variable { name } => {
                match state.variables.get(name) {
                    Some(value) => Ok(*value),
                    None => Err(Error::UninitializedVariable(name.clone())),
                }
            },
            Node::Constant { value } => Ok(*value),
            Node::Unary { child, strategy} => strategy(child, state),
            Node::Binary { left, right, strategy } => strategy(left, right, state),
        }
    }
}

