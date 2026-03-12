use crate::middle_end::nodes::transformation_node::TransformationNode;

pub struct TransformationPipeline {
    nodes: Vec<Box<dyn TransformationNode>>,
}

impl TransformationPipeline {
    pub fn new() -> Self {
        Self { nodes: Vec::new() }
    }

    pub fn add<N>(mut self, node: N) -> Self
    where
        N: TransformationNode + 'static,
    {
        self.nodes.push(Box::new(node));
        self
    }

    pub fn run(self, file: &mut crate::DbcFile) {
        for node in self.nodes {
            node.transform(file);
        }
    }
}
