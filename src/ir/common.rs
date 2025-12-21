use can_dbc::Node as ParsedNode;
use can_dbc::Transmitter as ParsedTransmitter;

#[derive(Debug, Clone)]
pub struct Identifier(pub String);

#[derive(Debug, Clone)]
pub struct NodeName(pub String);

#[derive(Debug, Clone)]
pub struct Node {
    pub name: NodeName,
}
impl From<ParsedNode> for Node {
    fn from(value: ParsedNode) -> Self {
        Node {
            name: (NodeName(value.0)),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Transmitter {
    Node(NodeName),
    VectorXXX,
}
impl From<ParsedTransmitter> for Transmitter {
    fn from(value: ParsedTransmitter) -> Self {
        match value {
            ParsedTransmitter::NodeName(s) => Transmitter::Node(NodeName(s)),
            ParsedTransmitter::VectorXXX => Transmitter::VectorXXX,
        }
    }
}
