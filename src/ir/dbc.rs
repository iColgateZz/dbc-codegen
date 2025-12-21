use crate::ir::{Message, Node, map_into};
use can_dbc::Dbc;

#[derive(Debug, Clone)]
pub struct DbcFile {
    pub nodes: Vec<Node>,
    pub messages: Vec<Message>,
}
impl From<Dbc> for DbcFile {
    fn from(value: Dbc) -> Self {
        DbcFile {
            nodes: map_into(value.nodes),
            messages: map_into(value.messages),
        }
    }
}
