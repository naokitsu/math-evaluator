#[cfg(test)]
mod tests {
    use crate::{eval, State};

    #[test]
    fn constant() {
        let mut state = State {
            variables: std::collections::HashMap::from([("x".to_string(), 42)]),
        };

        let result = eval("42".chars(), &mut state);
        assert_eq!(result, Ok(42));
    }

    #[test]
    fn variable() {
        let mut state = State {
            variables: std::collections::HashMap::from([("x".to_string(), 42)]),
        };

        let result = eval("x".chars(), &mut state);
        assert_eq!(result, Ok(42));
    }

    #[test]
    fn unary_operator() {
        let mut state = State {
            variables: std::collections::HashMap::new(),
        };

        let result = eval("-12".chars(), &mut state);
        assert_eq!(result, Ok(-12));
        let result = eval("22".chars(), &mut state);
        assert_eq!(result, Ok(22));
    }

    #[test]
    fn binary_operator() {
        let mut state = State {
            variables: std::collections::HashMap::new(),
        };

        let result = eval("12 + 12".chars(), &mut state);
        assert_eq!(result, Ok(24));
        let result = eval("12 - 2".chars(), &mut state);
        assert_eq!(result, Ok(10));
        let result = eval("12 * 4".chars(), &mut state);
        assert_eq!(result, Ok(48));
        let result = eval("12 / 8".chars(), &mut state);
        assert_eq!(result, Ok(1));
    }

    #[test]
    fn chained_divisions() {
        let mut state = State {
            variables: std::collections::HashMap::new(),
        };

        let result = eval("24 / 8 / 2".chars(), &mut state);
        assert_eq!(result, Ok(1));
    }

    #[test]
    fn parenthesis() {
        let mut state = State {
            variables: std::collections::HashMap::new(),
        };

        let result = eval("(12 + 12) * 5 + 2 * 4 ".chars(), &mut state);
        assert_eq!(result, Ok((12 + 12) * 5 + 2 * 4));
    }

    #[test]
    fn variables() {
        let mut state = State {
            variables: std::collections::HashMap::new(),
        };

        let result = eval("x = 12".chars(), &mut state);
        assert_eq!(result, Ok(12));
        let result = eval("x + 1".chars(), &mut state);
        assert_eq!(result, Ok(13));

    }

    #[test]
    fn variables_in_parenthesis() {
        let mut state = State {
            variables: std::collections::HashMap::new(),
        };

        let result = eval("(x = 12) + (x = 7)".chars(), &mut state);
        assert_eq!(result, Ok(19));
        let result = eval("x".chars(), &mut state);
        assert_eq!(result, Ok(7));
    }
}

