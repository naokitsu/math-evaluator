use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct State {
    pub variables: HashMap<String, i32>,
}