use std::cmp::Ordering;
use std::ops::{Deref};
use crate::error::Error;
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

/// Pop one/two node(s) from the node list and add put new operation node inside
fn collapse(operation: Operation, nodes: &mut Vec<Node>) -> Result<(), Error> {
    match operation {
        Operation::UnaryPlus => {
            let prev = nodes.pop().ok_or(Error::InvalidSyntax)?;
            nodes.push(
                Node::Unary {
                    child: Box::new(prev),
                    sign: '+',
                    strategy: |child, state| Ok(child.eval(state)?),
                }
            )
        }
        Operation::UnaryMinus => {
            let prev = nodes.pop().ok_or(Error::InvalidSyntax)?;
            nodes.push(
                Node::Unary {
                    child: Box::new(prev),
                    sign: '-',
                    strategy: |child, state| Ok(-child.eval(state)?),
                }
            );
        }
        Operation::BinaryPlus => {
            let prev_top = nodes.pop().ok_or(Error::InvalidSyntax)?;
            let prev_bot = nodes.pop().ok_or(Error::InvalidSyntax)?;
            nodes.push(Node::Binary {
                left: Box::new(prev_bot),
                right: Box::new(prev_top),
                sign: '+',
                strategy: |left, right, state| {
                    let l = left.eval(state)?;
                    let r = right.eval(state)?;
                    Ok(l + r)
                },
            });
        }
        Operation::BinaryMinus => {
            let prev_top = nodes.pop().ok_or(Error::InvalidSyntax)?;
            let prev_bot = nodes.pop().ok_or(Error::InvalidSyntax)?;
            nodes.push(Node::Binary {
                left: Box::new(prev_bot),
                right: Box::new(prev_top),
                sign: '-',
                strategy: |left, right, state| {
                    let l = left.eval(state)?;
                    let r = right.eval(state)?;
                    Ok(l - r)
                },
            });
        }
        Operation::Multiply => {
            let prev_top = nodes.pop().ok_or(Error::InvalidSyntax)?;
            let prev_bot = nodes.pop().ok_or(Error::InvalidSyntax)?;
            nodes.push(Node::Binary {
                left: Box::new(prev_bot),
                right: Box::new(prev_top),
                sign: '*',
                strategy: |left, right, state| {
                    let l = left.eval(state)?;
                    let r = right.eval(state)?;
                    Ok(l * r)
                },
            });
        }
        Operation::Divide => {
            let prev_top = nodes.pop().ok_or(Error::InvalidSyntax)?;
            let prev_bot = nodes.pop().ok_or(Error::InvalidSyntax)?;
            nodes.push(Node::Binary {
                left: Box::new(prev_bot),
                right: Box::new(prev_top),
                sign: '/',
                strategy: |left, right, state| {
                    let l = left.eval(state)?;
                    let r = right.eval(state)?;
                    Ok(l / r)
                },
            });
        },
        Operation::OpenParenthesis => {
            let prev = nodes.pop().ok_or(Error::InvalidSyntax)?;
            nodes.push(
                Node::Parenthesis {
                    child: Box::new(prev),
                }
            );
        },
        Operation::CloseParenthesis => {

        },
        Operation::Assign => {
            let prev_top = nodes.pop().ok_or(Error::InvalidSyntax)?;
            let prev_bot = nodes.pop().ok_or(Error::InvalidSyntax)?;
            nodes.push(Node::Binary {
                left: Box::new(prev_bot),
                right: Box::new(prev_top),
                sign: '=',
                strategy: |left, right, state| {
                    let l = left;
                    let r = right.eval(state)?; // eval right before left
                    if let Node::Variable { name } = l.deref() {
                        state.variables.insert(name.clone(), r);
                        Ok(r)
                    } else {
                        Err(Error::CanOnlyAssignToVariable)
                    }
                },
            });
        }
    }
    Ok(())
}

/// Evaluate expression from the iterator
pub fn eval(expression: impl Iterator<Item = char>, state: &mut State) -> Result<i32, Error> {
    let tokens = TokenIterator { inner: expression.peekable() };

    let mut nodes: Vec<Node> = Vec::new();
    let mut operations: Vec<Operation> = Vec::new();

    let mut expect_operand = true;
    'outer: for token in tokens {
        let to_be_pushed;
        match (token, expect_operand) {
            (Token::Operand(operand), true) => {
                nodes.push(
                    match operand {
                        OperandsToken::Constant(value) => Node::Constant { value },
                        OperandsToken::Variable(name) => Node::Variable { name }
                    }
                );
                to_be_pushed = None;
                expect_operand = false;
            }
            (Token::Operation(operation), true) => {
                match operation {
                    OperationToken::Plus => to_be_pushed = Some(Operation::UnaryPlus),
                    OperationToken::Minus => to_be_pushed = Some(Operation::UnaryMinus),
                    _ => return Err(Error::InvalidSyntax),
                }
            }

            (Token::Operation(operation), false) => {
                match operation {
                    OperationToken::Plus => to_be_pushed = Some(Operation::BinaryPlus),
                    OperationToken::Minus => to_be_pushed = Some(Operation::BinaryMinus),
                    OperationToken::Multiply => to_be_pushed = Some(Operation::Multiply),
                    OperationToken::Divide => to_be_pushed = Some(Operation::Divide),
                    OperationToken::Assign => to_be_pushed = Some(Operation::Assign),
                }
                expect_operand = true;
            }

            (Token::OpenParenthesis, true) => {
                to_be_pushed = Some(Operation::OpenParenthesis);
                expect_operand = true;
            },
            (Token::CloseParenthesis, false) => {
                to_be_pushed = Some(Operation::CloseParenthesis);
                expect_operand = false;
            },

            (_, _) => return Err(Error::InvalidSyntax),
        }

        while let (Some(&x), Some(y)) = (operations.last(), to_be_pushed) {
            if let (Operation::OpenParenthesis, Operation::CloseParenthesis) = (x, y) {
                operations.pop();
                continue 'outer;
            }
            if x <= y {
                operations.pop();
                collapse(x, &mut nodes)?;
            } else {
                break;
            }
        }

        match (operations.last(), to_be_pushed) {
            (Some(Operation::OpenParenthesis), Some(Operation::CloseParenthesis)) => {
                operations.pop();
                collapse(Operation::CloseParenthesis, &mut nodes)?;
                continue;
            }
            (Some(&x), Some(y)) => {
                while x <= y {
                    operations.pop();
                    collapse(x, &mut nodes)?;
                }

            },
            _ => {}
        }
        if let Some(y) = to_be_pushed {
            operations.push(y);
        }
    }

    while let Some(operation) = operations.pop() {
        collapse(operation, &mut nodes)?
    }


    nodes.pop().ok_or(Error::InvalidSyntax)?.eval(state)

}
