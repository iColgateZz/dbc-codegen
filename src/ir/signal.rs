use crate::ir::{Identifier, NodeName};

#[derive(Debug, Clone)]
pub struct Signal {
    pub name: SignalName,
    pub multiplexer: Multiplexer,
    pub start_bit: u32,
    pub size: u32,
    pub byte_order: ByteOrder,
    pub value_type: ValueType,
    pub factor: f64,
    pub offset: f64,
    pub min: f64,
    pub max: f64,
    pub unit: String,
    pub receivers: Vec<Receiver>,
}

#[derive(Debug, Clone)]
pub struct SignalName(pub Identifier);

#[derive(Debug, Clone)]
pub enum Multiplexer {
    None,
    Multiplexor,
    Multiplexed(u32),
}

#[derive(Debug, Clone)]
pub enum ByteOrder {
    BigEndian,
    LittleEndian,
}

#[derive(Debug, Clone)]
pub enum ValueType {
    Unsigned,
    Signed,
}

#[derive(Debug, Clone)]
pub enum Receiver {
    Node(NodeName),
    VectorXXX,
}
