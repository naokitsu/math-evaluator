use std::collections::HashMap;

#[derive(Clone, Debug)]
pub(crate) struct State {
    pub variables: HashMap<String, i32>,
}