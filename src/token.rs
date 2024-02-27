use std::iter::Peekable;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Token {
    Plus,
    Minus,
    Multiply,
    Divide,
    OpenParenthesis,
    CloseParenthesis,
    Variable(String),
    Constant(i32),
    Unexpected,
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
                        '+' => Some(Token::Plus),
                        '-' => Some(Token::Minus),
                        '*' => Some(Token::Multiply),
                        '/' => Some(Token::Divide),
                        '(' => Some(Token::OpenParenthesis),
                        ')' => Some(Token::CloseParenthesis),
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
                                    Ok(number) => Some(Token::Constant(number)),
                                    Err(_) => Some(Token::Unexpected),
                                }
                            } else {
                                Some(Token::Variable(word))
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