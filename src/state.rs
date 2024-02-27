use std::collections::HashMap;

#[derive(Copy, Clone, Debug)]
pub(crate) struct State {
    variables: HashMap<String, i32>,
}