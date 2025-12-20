use crate::ir::{Identifier, map_into};
use can_dbc::ValDescription;
use can_dbc::ValueTable as ParsedValueTable;

#[derive(Debug, Clone)]
pub struct ValueTable {
    pub name: Identifier,
    pub descriptions: Vec<ValueDescription>,
}
impl From<ParsedValueTable> for ValueTable {
    fn from(value: ParsedValueTable) -> Self {
        ValueTable {
            name: Identifier(value.name),
            descriptions: map_into(value.descriptions),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ValueDescription {
    pub id: i64,
    pub description: String,
}
impl From<ValDescription> for ValueDescription {
    fn from(value: ValDescription) -> Self {
        ValueDescription {
            id: value.id,
            description: value.description,
        }
    }
}
