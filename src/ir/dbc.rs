use crate::ir::{BitTiming, Message, Node, Symbol, ValueTable, Version, map_into};
use can_dbc::Dbc;

#[derive(Debug, Clone)]
pub struct DbcFile {
    pub version: Version,
    pub new_symbols: Vec<Symbol>,
    pub bit_timings: Vec<BitTiming>,
    pub nodes: Vec<Node>,
    pub value_tables: Vec<ValueTable>,
    pub messages: Vec<Message>,
}
impl From<Dbc> for DbcFile {
    fn from(value: Dbc) -> Self {
        DbcFile {
            version: Version::from(value.version),
            new_symbols: map_into(value.new_symbols),
            bit_timings: map_into(value.bit_timing.unwrap_or_default()),
            nodes: map_into(value.nodes),
            value_tables: map_into(value.value_tables),
            messages: map_into(value.messages),
        }
    }
}
