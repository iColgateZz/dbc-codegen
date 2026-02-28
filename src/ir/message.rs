use crate::ir::helpers::ToUpperCamelCase;
use crate::ir::map_into;
use crate::ir::{Identifier, Signal, Transmitter};
use can_dbc::Message as ParsedMessage;
use can_dbc::MessageId as ParsedMessageId;

#[derive(Debug, Clone)]
pub struct Message {
    pub id: MessageId,
    pub name: Identifier,
    pub original_name: Identifier,
    pub size: u64,
    pub transmitter: Transmitter,
    pub signals: Vec<Signal>,
}
impl From<ParsedMessage> for Message {
    fn from(value: ParsedMessage) -> Self {
        Message {
            id: MessageId::from(value.id),
            name: Identifier(value.name.to_upper_camelcase()),
            original_name: Identifier(value.name),
            size: value.size,
            transmitter: Transmitter::from(value.transmitter),
            signals: map_into(value.signals),
        }
    }
}

#[derive(Debug, Clone)]
pub enum MessageId {
    Standard(u16),
    Extended(u32),
}
impl From<ParsedMessageId> for MessageId {
    fn from(value: ParsedMessageId) -> Self {
        match value {
            ParsedMessageId::Standard(v) => MessageId::Standard(v),
            ParsedMessageId::Extended(v) => MessageId::Extended(v),
        }
    }
}
