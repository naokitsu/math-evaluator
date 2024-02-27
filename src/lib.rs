use crate::state::State;

mod error;
pub mod state;
mod node;
mod tests;

pub fn eval(expression: impl Iterator<Item = char>, state: &State) -> i32 {
    for character in expression {
        println!("{}",character);
    };
    42
}
