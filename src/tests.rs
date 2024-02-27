#[cfg(test)]
mod tests {
    use crate::{eval, State};

    #[test]
    fn constant() {
        let state = State {
            variables: std::collections::HashMap::from([("x".to_string(), 42)]),
        };

        let result = eval("42".chars(), &state);
        assert_eq!(result, 42);
    }

    #[test]
    fn variable() {
        let state = State {
            variables: std::collections::HashMap::from([("x".to_string(), 42)]),
        };

        let result = eval("x".chars(), &state);
        assert_eq!(result, 42);
    }

    #[test]
    fn unary_operator() {
        let state = State {
            variables: std::collections::HashMap::new(),
        };

        let result = eval("-12".chars(), &state);
        assert_eq!(result, -12);
        let result = eval("22".chars(), &state);
        assert_eq!(result, 12);
    }

    #[test]
    fn binary_operator() {
        let state = State {
            variables: std::collections::HashMap::new(),
        };

        let result = eval("12 + 12".chars(), &state);
        assert_eq!(result, 24);
        let result = eval("12 - 2".chars(), &state);
        assert_eq!(result, 10);
        let result = eval("12 * 4".chars(), &state);
        assert_eq!(result, 48);
        let result = eval("12 / 8".chars(), &state);
        assert_eq!(result, 1);
    }
}
