use std::cmp::Ordering;
use std::ops::{Deref};
use crate::error::Error;
use crate::error::Error::CanOnlyAssignToVariable;
use crate::node::Node;
use crate::state::State;
use crate::token::{OperandsToken, OperationToken, Token, TokenIterator};


#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum Operation {
    UnaryPlus,
    BinaryPlus,
    UnaryMinus,
    BinaryMinus,
    Multiply,
    Divide,
    OpenParenthesis,
    CloseParenthesis,
    Assign,
}

impl PartialOrd for Operation {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Operation::UnaryPlus | Operation::BinaryPlus | Operation::UnaryMinus | Operation::BinaryMinus, Operation::UnaryPlus | Operation::BinaryPlus | Operation::UnaryMinus | Operation::BinaryMinus) => Some(Ordering::Equal),
            (Operation::Multiply | Operation::Divide, Operation::Multiply | Operation::Divide) => Some(Ordering::Equal),
            (Operation::UnaryPlus | Operation::BinaryPlus | Operation::UnaryMinus | Operation::BinaryMinus, Operation::Multiply | Operation::Divide) => Some(Ordering::Greater),
            (Operation::Multiply | Operation::Divide, Operation::UnaryPlus | Operation::BinaryPlus | Operation::UnaryMinus | Operation::BinaryMinus) => Some(Ordering::Less),
            (Operation::OpenParenthesis, _) => Some(Ordering::Greater),
            (Operation::CloseParenthesis, _) => Some(Ordering::Less),
            (_, Operation::OpenParenthesis) => Some(Ordering::Greater),
            (_, Operation::CloseParenthesis) => Some(Ordering::Less),
            (Operation::Assign, _) => Some(Ordering::Greater),
            (_, Operation::Assign) => Some(Ordering::Greater)
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
                    sign: '+',
                    strategy: |child, state| Ok(child.calculate(state)?),
                }
            );
        }
        Operation::UnaryMinus => {
            let prev = nodes.pop().unwrap();
            nodes.push(
                Node::Unary {
                    child: Box::new(prev),
                    sign: '-',
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
                sign: '+',
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
                sign: '-',
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
                sign: '*',
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
                sign: '/',
                strategy: |left, right, state| {
                    let l = left.calculate(state)?;
                    let r = right.calculate(state)?;
                    Ok(l / r)
                },
            });
        },
        Operation::OpenParenthesis => {
            let prev = nodes.pop().unwrap();
            nodes.push(
                Node::Parenthesis {
                    child: Box::new(prev),
                }
            );
        },
        Operation::CloseParenthesis => {

        },
        Operation::Assign => {
            let prev_top = nodes.pop().unwrap();
            let prev_bot = nodes.pop().unwrap();
            nodes.push(Node::Binary {
                left: Box::new(prev_bot),
                right: Box::new(prev_top),
                sign: '=',
                strategy: |left, right, state| {
                    let l = left;
                    let r = right.calculate(state)?;
                    if let Node::Variable { name } = l.deref() {
                        state.variables.insert(name.clone(), r);
                        Ok(r)
                    } else {
                        Err(CanOnlyAssignToVariable)
                    }
                },
            });
        }
    }
}

pub fn eval(expression: impl Iterator<Item = char>, state: &mut State) -> Result<i32, Error> {
    let tokens = TokenIterator { inner: expression.peekable() };

    let mut nodes: Vec<Node> = Vec::new();
    let mut operations: Vec<Operation> = Vec::new();

    let mut expect_operand = true;
    'outer: for token in tokens {
        let to_be_pushed;
        match (token, expect_operand) {
            (Token::Operand(OperandsToken::Constant(number)), true) => {
                nodes.push(Node::Constant {
                    value: number
                });
                to_be_pushed = None;
                expect_operand = false;
            }
            (Token::Operand(OperandsToken::Variable(name)), true) => {
                nodes.push(
                    Node::Variable {
                        name
                    }
                );
                to_be_pushed = None;
                expect_operand = false;
            }
            (Token::Operation(OperationToken::Plus), true)  => {
                to_be_pushed = Some(Operation::UnaryPlus);
                expect_operand = true;
            }
            (Token::Operation(OperationToken::Plus), false)  => {
                to_be_pushed = Some(Operation::BinaryPlus);
                expect_operand = true;
            }
            (Token::Operation(OperationToken::Minus), true) => {
                to_be_pushed = Some(Operation::UnaryMinus);
                expect_operand = true;
            },
            (Token::Operation(OperationToken::Minus), false) => {
                to_be_pushed = Some(Operation::BinaryMinus);
                expect_operand = true;
            },
            (Token::Operation(OperationToken::Multiply), false) => {
                to_be_pushed = Some(Operation::Multiply);
                expect_operand = true;
            },
            (Token::Operation(OperationToken::Divide), false) => {
                to_be_pushed = Some(Operation::Divide);
                expect_operand = true;
            },
            (Token::OpenParenthesis, true) => {
                to_be_pushed = Some(Operation::OpenParenthesis);
                expect_operand = true;
            },
            (Token::CloseParenthesis, false) => {
                to_be_pushed = Some(Operation::CloseParenthesis);
                expect_operand = false;
            },
            (Token::Operation(OperationToken::Assign), false) => {
                to_be_pushed = Some(Operation::Assign);
                expect_operand = true;
            }

            (_, _) => {
                return Err(Error::InvalidSyntax);
            }
        }

        while let (Some(&x), Some(y)) = (operations.last(), to_be_pushed) {
            if let (Operation::OpenParenthesis, Operation::CloseParenthesis) = (x, y) {
                operations.pop();
                continue 'outer;
            }
            if x <= y {
                operations.pop();
                collapse(x, &mut nodes);
            } else {
                break;
            }
        }

        match (operations.last(), to_be_pushed) {
            (Some(Operation::OpenParenthesis), Some(Operation::CloseParenthesis)) => {
                operations.pop();
                collapse(Operation::CloseParenthesis, &mut nodes);
                continue;
            }
            (Some(&x), Some(y)) => {
                while x <= y {
                    operations.pop();
                    collapse(x, &mut nodes);
                }

            },
            _ => {}
        }
        if let Some(y) = to_be_pushed {
            operations.push(y);
        }
    }

    while let Some(operation) = operations.pop() {
        collapse(operation, &mut nodes)
    }


    nodes.pop().ok_or(Error::InvalidSyntax)?.calculate(state)

}
