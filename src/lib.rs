mod error;
mod state;
mod node;
mod tests;

pub fn eval(expression: impl Iterator<Item = char>) -> i32 {
    for character in expression {
        println!("{}",character);
    };
    42
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sum() {
        let result = eval("1 + 2".chars());
        assert_eq!(result, 3);
    }
}
