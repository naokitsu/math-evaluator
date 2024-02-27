use std::fmt::{Debug, Display};
use crate::error::Error;
use crate::state::State;

#[derive(Debug, Clone)]
pub(crate) enum Node {
    Leaf {
        strategy: fn(&State) -> Result<i32, Error>,
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
            Node::Leaf { strategy } => strategy(state),
            Node::Unary { child, strategy} => strategy(child, state),
            Node::Binary { left, right, strategy } => strategy(left, right, state),
        }
    }
}

