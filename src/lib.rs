use std::cmp::Ordering;
use crate::error::Error;
use crate::node::Node;
use crate::state::State;
use crate::token::{OperandsToken, OperationToken, Token, TokenIterator};

pub mod error;
pub mod state;
mod node;
mod tests;
mod token;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum Operation {
    UnaryPlus,
    BinaryPlus,
    UnaryMinus,
    BinaryMinus,
    Multiply,
    Divide,
}

impl PartialOrd for Operation {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Operation::UnaryPlus | Operation::BinaryPlus | Operation::UnaryMinus | Operation::BinaryMinus, Operation::UnaryPlus | Operation::BinaryPlus | Operation::UnaryMinus | Operation::BinaryMinus) => Some(Ordering::Equal),
            (Operation::Multiply | Operation::Divide, Operation::Multiply | Operation::Divide) => Some(Ordering::Equal),
            (Operation::UnaryPlus | Operation::BinaryPlus | Operation::UnaryMinus | Operation::BinaryMinus, Operation::Multiply | Operation::Divide) => Some(Ordering::Greater),
            (Operation::Multiply | Operation::Divide, Operation::UnaryPlus | Operation::BinaryPlus | Operation::UnaryMinus | Operation::BinaryMinus) => Some(Ordering::Less),
        }
    }
}

fn collapse(operation: Operation, nodes: &mut Vec<Node>) {
    match operation {
        Operation::UnaryPlus => {
            let prev = nodes.pop().unwrap();
            nodes.push(
                Node::Unary {
                    child: Box::new(prev),
                    strategy: |child, state| Ok(child.calculate(state)?),
                }
            );
        }
        Operation::UnaryMinus => {
            let prev = nodes.pop().unwrap();
            nodes.push(
                Node::Unary {
                    child: Box::new(prev),
                    strategy: |child, state| Ok(-child.calculate(state)?),
                }
            );
        }
        Operation::BinaryPlus => {
            let prev_top = nodes.pop().unwrap();
            let prev_bot = nodes.pop().unwrap();
            nodes.push(Node::Binary {
                left: Box::new(prev_bot),
                right: Box::new(prev_top),
                strategy: |left, right, state| {
                    let l = left.calculate(state)?;
                    let r = right.calculate(state)?;
                    Ok(l + r)
                },
            });
        }
        Operation::BinaryMinus => {
            let prev_top = nodes.pop().unwrap();
            let prev_bot = nodes.pop().unwrap();
            nodes.push(Node::Binary {
                left: Box::new(prev_bot),
                right: Box::new(prev_top),
                strategy: |left, right, state| {
                    let l = left.calculate(state)?;
                    let r = right.calculate(state)?;
                    Ok(l - r)
                },
            });
        }
        Operation::Multiply => {
            let prev_top = nodes.pop().unwrap();
            let prev_bot = nodes.pop().unwrap();
            nodes.push(Node::Binary {
                left: Box::new(prev_bot),
                right: Box::new(prev_top),
                strategy: |left, right, state| {
                    let l = left.calculate(state)?;
                    let r = right.calculate(state)?;
                    Ok(l * r)
                },
            });
        }
        Operation::Divide => {
            let prev_top = nodes.pop().unwrap();
            let prev_bot = nodes.pop().unwrap();
            nodes.push(Node::Binary {
                left: Box::new(prev_bot),
                right: Box::new(prev_top),
                strategy: |left, right, state| {
                    let l = left.calculate(state)?;
                    let r = right.calculate(state)?;
                    Ok(l / r)
                },
            });
        }
    }
}

pub fn eval(expression: impl Iterator<Item = char>, state: &State) -> Result<i32, Error> {
    let tokens = TokenIterator { inner: expression.peekable() };

    let mut nodes = Vec::new();
    let mut operations = Vec::new();

    let mut expect_operand = true;
    for token in tokens {
        match (token, expect_operand) {
            (Token::Operand(OperandsToken::Constant(number)), true) => {
                nodes.push(Node::Constant {
                    value: number
                });
                expect_operand = false;
            }
            (Token::Operand(OperandsToken::Variable(name)), true) => {
                nodes.push(
                    Node::Variable {
                        name
                    }
                );
                expect_operand = false;
            }
            (Token::Operation(OperationToken::Plus), true)  => {
                operations.push(Operation::UnaryPlus);
                expect_operand = true;
            }
            (Token::Operation(OperationToken::Plus), false)  => {
                operations.push(Operation::BinaryPlus);
                expect_operand = true;
            }
            (Token::Operation(OperationToken::Minus), true) => {
                operations.push(Operation::UnaryMinus);
                expect_operand = true;
            },
            (Token::Operation(OperationToken::Minus), false) => {
                operations.push(Operation::BinaryMinus);
                expect_operand = true;
            },
            (Token::Operation(OperationToken::Multiply), false) => {
                operations.push(Operation::Multiply);
                expect_operand = true;
            },
            (Token::Operation(OperationToken::Divide), false) => {
                operations.push(Operation::Divide);
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

        if operations.len() <= 2 {
            continue;
        }
        if let Some(x) = operations.get(operations.len() - 1) {
            if let Some(y) = operations.get(operations.len() - 2) {
                if *x > *y {
                    let operation = operations.pop().unwrap();
                    collapse(operation, &mut nodes)
                }
            }
        }
    }

    while let Some(operation) = operations.pop() {
        collapse(operation, &mut nodes)
    }

    nodes.pop().unwrap().calculate(state)

}
