#[derive(Debug, Clone)]
pub struct Identifier(pub String);

#[derive(Debug, Clone)]
pub struct Symbol(pub Identifier);

#[derive(Debug, Clone)]
pub struct Version(pub String);

#[derive(Debug, Clone)]
pub struct NodeName(pub String);

#[derive(Debug, Clone)]
pub struct Node {
    pub name: NodeName,
}

#[derive(Debug, Clone)]
pub enum Transmitter {
    Node(NodeName),
    VectorXXX,
}