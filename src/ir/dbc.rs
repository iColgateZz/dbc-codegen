use crate::ir::{Message, Node, map_into};
use can_dbc::Dbc as ParsedDbc;

#[derive(Debug, Clone)]
pub struct DbcFile {
    pub nodes: Vec<Node>,
    pub messages: Vec<Message>,
}
impl From<ParsedDbc> for DbcFile {
    fn from(value: ParsedDbc) -> Self {
        DbcFile {
            nodes: map_into(value.nodes),
            messages: map_into(value.messages),
        }
    }
}
