use crate::{codegen::Generator, ir::message::Message};
use std::fmt::Write;

pub struct RustGen {
    buf: String,
}

impl Generator for RustGen {
    fn buf_mut(&mut self) -> &mut String {
        &mut self.buf
    }
}

impl RustGen {
    pub fn new() -> RustGen {
        RustGen {
            buf: String::new(),
        }
    }

    pub fn generate(mut self, messages: &[Message]) -> String {
        for msg in messages {
            self.message(msg, 0);
        }

        self.buf
    }

    fn message(&mut self, msg: &Message, indent: usize) {
        self.derive(indent);
        // ...
    }

    fn derive(&mut self, indent: usize) {
        self.write_line(indent, "#[derive(Debug, Clone)]");
    }
}
