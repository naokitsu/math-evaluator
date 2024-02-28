use std::collections::HashMap;

/// The state of the variables
#[derive(Clone, Debug)]
pub struct State {
    pub variables: HashMap<String, i32>,
}