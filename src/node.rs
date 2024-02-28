use std::fmt::{Debug, Display, Formatter};
use crate::error::Error;
use crate::state::State;

/// Operation nodes for parser tree
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
    },
    Unary {
        child: Box<Node>,
        sign: char,
        strategy: fn(child: &Box<Node>, &mut State) -> Result<i32, Error>,
    },
    Binary {
        left: Box<Node>,
        right: Box<Node>,
        sign: char,
        strategy: fn(left: &Box<Node>, right: &Box<Node>, &mut State) -> Result<i32, Error>,
    },
}

impl Node {
    /// Evaluates the value of the node
    pub(crate) fn eval(&self, state: &mut State) -> Result<i32, Error> {
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
            Node::Parenthesis {child, ..} => child.eval(state),
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
            Node::Variable { name } => write!(f, "(Var: {})", name),
            Node::Constant { value } => write!(f, "(Const: {})", value),
            Node::Unary { child, sign, .. } => write!(f, "(Unary {}: {})", sign, child),
            Node::Binary { left, right, sign, .. } => write!(f, "(Binary {}: {}, {})", sign, left, right),
            Node::Parenthesis {child, ..} => write!(f, "(Nested: {})", child),
        }
    }
}
