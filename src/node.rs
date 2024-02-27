use std::fmt::{Display};
use crate::error::Error;
use crate::state::State;

trait Node {
    fn calculate(&self, state: State) -> Result<i32, Error>;
}

#[derive(Copy, Clone, Debug)]
struct ConstantNode {
    value: i32,
}


impl Node for ConstantNode {
    fn calculate(&self, _: State) -> Result<i32, Error> {
        Ok(self.value)
    }
}

#[derive(Copy, Clone, Debug)]
struct VariableNode {
    name: String,
}

impl Node for VariableNode {
    fn calculate(&self, state: State) -> Result<i32, Error> {
        if let Some(x) = state.get(&self.name) {
            Ok(*x)
        } else {
            Err(Error::UninitializedVariable(self.name.clone()))
        }
    }
}

#[derive(Copy, Clone, Debug)]
struct UnaryNode {
    child: Box<dyn Node>,
    operator: char,
}

impl Node for UnaryNode {
    fn calculate(&self, state: State) -> Result<i32, Error> {
        let child = self.child.calculate(state)?;
        match self.operator {
            '+' => Ok(child),
            '-' => Ok(-child),
            _ => Err(Error::InvalidSyntax)
        }
    }
}

struct BinaryNode {
    left: Box<dyn Node>,
    right: Box<dyn Node>,
    operator: char,
}

impl Node for BinaryNode {
    fn calculate(&self, state: State) -> Result<i32, Error> {
        let left = self.left.calculate(state)?;
        let right = self.right.calculate(state)?;
        match self.operator {
            '+' => Ok(left + right),
            '-' => Ok(left - right),
            '*' => Ok(left * right),
            '/' => Ok(left / right),
            _ => Err(Error::InvalidSyntax)
        }
    }
}
