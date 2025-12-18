use crate::ir::Identifier;

#[derive(Debug, Clone)]
pub struct ValueTable {
    pub name: Identifier,
    pub values: Vec<ValueDescription>,
}

#[derive(Debug, Clone)]
pub struct ValueDescription {
    pub value: f64,
    pub text: String,
}
