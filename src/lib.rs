use std::cmp::Ordering;
use crate::node::Node;
use crate::state::State;
use crate::token::{OperandsToken, OperationToken, Token, TokenIterator};

mod error;
pub mod state;
mod node;
mod tests;
mod token;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum Operations {
    Plus,
    Minus,
    Multiply,
    Divide,
}

impl PartialOrd for Operations {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Operations::Plus | Operations::Minus, Operations::Plus | Operations::Minus) => Some(Ordering::Equal),
            (Operations::Multiply | Operations::Divide, Operations::Multiply | Operations::Divide) => Some(Ordering::Equal),
            (Operations::Plus | Operations::Minus, Operations::Multiply | Operations::Divide) => Some(Ordering::Greater),
            (Operations::Multiply | Operations::Divide, Operations::Plus | Operations::Minus) => Some(Ordering::Less),
        }
    }
}

pub fn eval(expression: impl Iterator<Item = char>, state: &State) -> i32 {
    let tokens = TokenIterator { inner: expression.peekable() };

    let mut nodes = Vec::new();
    let mut operations = Vec::new();

    for token in tokens {
        let mut expect_operand = true;
        match (token, expect_operand) {
            (Token::Operand(OperandsToken::Constant(number)), true) => {
                nodes.push(Node::Leaf {
                    strategy: |state| Ok(42)
                });
                expect_operand = false;
            }
            (Token::Operand(OperandsToken::Variable(name)), true) => {
                nodes.push(
                    Node::Leaf {
                        strategy: |state| Ok(*state.variables.get("TODO").unwrap_or(&0)), // TODO
                    }
                );
                expect_operand = false;
            }
            (Token::Operation(OperationToken::Plus), true)  => {
                let prev = nodes.pop().unwrap();
                nodes.push(
                    Node::Unary {
                        child: Box::new(prev),
                        strategy: |child, state| Ok(child.calculate(state)?),
                    }
                );
                expect_operand = true;
            }
            (Token::Operation(OperationToken::Plus), false)  => {
                let prev_top = nodes.pop().unwrap();
                let prev_bot = nodes.pop().unwrap();
                nodes.push(Node::Binary {
                    left: Box::new(prev_top),
                    right: Box::new(prev_bot),
                    strategy: |left, right, state| {
                        let l = left.calculate(state)?;
                        let r = right.calculate(state)?;
                        Ok(l + r)
                    },
                });
                expect_operand = true;
            }
            (Token::Operation(OperationToken::Minus), true) => {
                let prev = nodes.pop().unwrap();
                nodes.push(
                    Node::Unary {
                        child: Box::new(prev),
                        strategy: |child, state| Ok(-child.calculate(state)?)
                    }
                );
                expect_operand = true;
            },
            (Token::Operation(OperationToken::Minus), false) => {
                let prev_top = nodes.pop().unwrap();
                let prev_bot = nodes.pop().unwrap();
                nodes.push(Node::Binary {
                    left: Box::new(prev_top),
                    right: Box::new(prev_bot),
                    strategy: |left, right, state| {
                        let l = left.calculate(state)?;
                        let r = right.calculate(state)?;
                        Ok(l - r)
                    },
                });
                expect_operand = true;
            },
            (Token::Operation(OperationToken::Multiply), false) => {
                let prev_top = nodes.pop().unwrap();
                let prev_bot = nodes.pop().unwrap();
                nodes.push(Node::Binary {
                    left: Box::new(prev_top),
                    right: Box::new(prev_bot),
                    strategy: |left, right, state| {
                        let l = left.calculate(state)?;
                        let r = right.calculate(state)?;
                        Ok(l * r)
                    },
                });
                expect_operand = true;
            },
            (Token::Operation(OperationToken::Divide), false) => {
                let prev_top = nodes.pop().unwrap();
                let prev_bot = nodes.pop().unwrap();
                nodes.push(Node::Binary {
                    left: Box::new(prev_top),
                    right: Box::new(prev_bot),
                    strategy: |left, right, state| {
                        let l = left.calculate(state)?;
                        let r = right.calculate(state)?;
                        Ok(l / r)
                    },
                });
                expect_operand = true;
            },
            (Token::OpenParenthesis, true) => {
                todo!()
            },
            (Token::CloseParenthesis, false) => {
                todo!()
            },
            (x, y) => {
                println!("{:?} {:?}", x, y);
                todo!("Unexpected")
            }
        }
    }

    println!("{:?}", nodes);

    42
}
