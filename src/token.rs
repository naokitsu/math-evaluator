use std::iter::Peekable;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Token {
    Operand(OperandsToken),
    Operation(OperationToken),
    OpenParenthesis,
    CloseParenthesis,
    Unexpected,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum OperandsToken {
    Variable(String),
    Constant(i32),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum OperationToken {
    Plus,
    Minus,
    Multiply,
    Divide,
    Assign,
}

pub(crate) struct TokenIterator<T>
    where
        T: Iterator<Item = char>
{
    pub(crate) inner: Peekable<T>,
}

impl<T> Iterator for TokenIterator<T>
    where
        T: Iterator<Item = char>
{
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.inner.next() {
                Some(c) => {
                    if c.is_ascii_whitespace() {
                        continue;
                    }
                    return match c {
                        '+' => Some(Token::Operation(OperationToken::Plus)),
                        '-' => Some(Token::Operation(OperationToken::Minus)),
                        '*' => Some(Token::Operation(OperationToken::Multiply)),
                        '/' => Some(Token::Operation(OperationToken::Divide)),
                        '(' => Some(Token::OpenParenthesis),
                        ')' => Some(Token::CloseParenthesis),
                        '=' => Some(Token::Operation(OperationToken::Assign)),
                        '0'..='9' | 'a'..='z' | 'A'..='Z' => {
                            let mut word = String::from(c);
                            let constant = match c {
                                '0'..='9' => true,
                                _ => false,
                            };
                            while let Some(c) = self.inner.peek() {
                                if c.is_ascii_alphanumeric() {
                                    word.push(*c);
                                    self.inner.next();
                                } else {
                                    break;
                                }
                            }
                            if constant {
                                match word.parse() {
                                    Ok(number) => Some(Token::Operand(OperandsToken::Constant(number))),
                                    Err(_) => Some(Token::Unexpected),
                                }
                            } else {
                                Some(Token::Operand(OperandsToken::Variable(word)))
                            }
                        },
                        _ => Some(Token::Unexpected),
                    }
                },
                None => return None,
            }
        }
    }
}