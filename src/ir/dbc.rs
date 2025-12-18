use crate::ir::{BitTiming, Message, Node, Symbol, ValueTable, Version};

#[derive(Debug, Clone)]
pub struct DbcFile {
    pub version: Version,
    pub new_symbols: Vec<Symbol>,
    pub bit_timings: Vec<BitTiming>,
    pub nodes: Vec<Node>,
    pub value_tables: Vec<ValueTable>,
    pub messages: Vec<Message>,
}
