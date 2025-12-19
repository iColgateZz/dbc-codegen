use can_dbc::Node as parsedNode;
use can_dbc::Symbol as parsedSymbol;
use can_dbc::Transmitter as parsedTransmitter;
use can_dbc::Version as parsedVersion;

#[derive(Debug, Clone)]
pub struct Identifier(pub String);

#[derive(Debug, Clone)]
pub struct Symbol(pub Identifier);
impl From<parsedSymbol> for Symbol {
    fn from(value: parsedSymbol) -> Self {
        Symbol(Identifier(value.0))
    }
}

#[derive(Debug, Clone)]
pub struct Version(pub String);
impl From<parsedVersion> for Version {
    fn from(value: parsedVersion) -> Self {
        Version(value.0)
    }
}

#[derive(Debug, Clone)]
pub struct NodeName(pub String);

#[derive(Debug, Clone)]
pub struct Node {
    pub name: NodeName,
}
impl From<parsedNode> for Node {
    fn from(value: parsedNode) -> Self {
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
impl From<parsedTransmitter> for Transmitter {
    fn from(value: parsedTransmitter) -> Self {
        match value {
            parsedTransmitter::NodeName(s) => Transmitter::Node(NodeName(s)),
            parsedTransmitter::VectorXXX => Transmitter::VectorXXX,
        }
    }
}
