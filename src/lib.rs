use crate::state::State;
use crate::token::TokenIterator;

mod error;
pub mod state;
mod node;
mod tests;
mod token;

pub fn eval(expression: impl Iterator<Item = char>, state: &State) -> i32 {
    let tokens = TokenIterator { inner: expression.peekable() };

    for token in tokens {
        println!("{:?}", token);
    }

    42
}
