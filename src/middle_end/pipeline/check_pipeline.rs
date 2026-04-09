use crate::middle_end::nodes::{CheckNode, Diagnostics};

pub struct CheckPipeline {
    nodes: Vec<Box<dyn CheckNode>>,
}

impl CheckPipeline {
    pub fn new() -> Self {
        Self { nodes: Vec::new() }
    }

    pub fn add<N>(mut self, node: N) -> Self
    where
        N: CheckNode + 'static,
    {
        self.nodes.push(Box::new(node));
        self
    }

    pub fn run(self, file: &crate::DbcFile, diagnostics: &mut Diagnostics) {
        for node in self.nodes {
            node.check(file, diagnostics);
        }
    }
}

//TODO: overlapping signals in a message
// invalid multiplexing layout
// enum value out of signal range
// signal bit overflow
// duplicate message IDs
// invalid scaling (factor = 0)
// inconsistent float/integer usage
