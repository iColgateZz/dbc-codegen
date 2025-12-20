use can_dbc::Node as ParsedNode;
use can_dbc::Symbol as ParsedSymbol;
use can_dbc::Transmitter as ParsedTransmitter;
use can_dbc::Version as ParsedVersion;

#[derive(Debug, Clone)]
pub struct Identifier(pub String);

#[derive(Debug, Clone)]
pub struct Symbol(pub Identifier);
impl From<ParsedSymbol> for Symbol {
    fn from(value: ParsedSymbol) -> Self {
        Symbol(Identifier(value.0))
    }
}

#[derive(Debug, Clone)]
pub struct Version(pub String);
impl From<ParsedVersion> for Version {
    fn from(value: ParsedVersion) -> Self {
        Version(value.0)
    }
}

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
