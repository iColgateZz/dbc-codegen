pub type Identifier = String;
pub type MessageId = u32;
pub type SignalName = String;
pub type NodeName = String;
pub type EnvVarName = String;

#[derive(Debug, Clone)]
pub struct Version(pub String);
