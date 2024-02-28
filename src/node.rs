use std::fmt::{Debug, Display, Formatter};
use crate::error::Error;
use crate::state::State;

#[derive(Clone)]
pub(crate) enum Node {
    Variable {
        name: String,
    },
    Constant {
        value: i32,
    },
    Parenthesis {
        child: Box<Node>,
        strategy: fn(child: &Box<Node>, &State) -> Result<i32, Error>,
    },
    Unary {
        child: Box<Node>,
        sign: char,
        strategy: fn(child: &Box<Node>, &State) -> Result<i32, Error>,
    },
    Binary {
        left: Box<Node>,
        right: Box<Node>,
        sign: char,
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
            Node::Unary { child, strategy, .. } => strategy(child, state),
            Node::Binary { left, right, strategy, .. } => strategy(left, right, state),
            Node::Parenthesis {child, ..} => child.calculate(state),

        }
    }
}

impl Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Node::Variable { name } => write!(f, "{}", name),
            Node::Constant { value } => write!(f, "{}", value),
            Node::Unary { child, sign, .. } => write!(f, "({}{})", sign, child),
            Node::Binary { left, right, sign, .. } => write!(f, "({} {} {})", left, sign, right),
            Node::Parenthesis {child, ..} => write!(f, "({})", child),
        }
    }
}

impl Debug for Node {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Node::Variable { name } => write!(f, "{}", name),
            Node::Constant { value } => write!(f, "{}", value),
            Node::Unary { child, sign, .. } => write!(f, "({}{})", sign, child),
            Node::Binary { left, right, sign, .. } => write!(f, "({} {} {})", left, sign, right),
            Node::Parenthesis {child, ..} => write!(f, "({})", child),
        }
    }
}
