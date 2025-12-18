use crate::ir::{Identifier, Signal, Transmitter};

#[derive(Debug, Clone)]
pub struct Message {
    pub id: MessageId,
    pub name: Identifier,
    pub size: u32,
    pub transmitter: Transmitter,
    pub signals: Vec<Signal>,
}

#[derive(Debug, Clone)]
pub enum MessageId {
    Standard(u16),
    Extended(u32),
}
