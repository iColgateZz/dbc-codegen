use crate::ir::message::MessageId;
use crate::{codegen::Generator, ir::message::Message};

pub mod sample_output;

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
        RustGen { buf: String::new() }
    }

    pub fn generate(mut self, messages: &[Message]) -> String {
        for msg in messages {
            self.message(msg, 0);
        }

        self.buf
    }

    fn message(&mut self, msg: &Message, indent: usize) {
        // #[derive(...)]
        self.derive(indent);

        // pub struct Name { ... }
        self.xstruct(msg, indent);
        self.write_newline();

        // impl Name { ... }
        self.ximpl(msg, indent);
    }

    fn derive(&mut self, indent: usize) {
        self.write_line(indent, "#[derive(Debug, Clone)]");
    }

    fn xstruct(&mut self, msg: &Message, indent: usize) {
        // pub struct Name {
        self.write_line(indent, &format!("pub struct {} {{", &msg.name.0));

        for sig in &msg.signals {
            let field = &sig.name.0.0;
            self.write_line(indent + 4, &format!("pub {}: f64,", field));
        }

        // close struct block
        self.write_line(indent, "}");
    }

    fn ximpl(&mut self, msg: &Message, indent: usize) {
        // impl Name {
        self.write_line(indent, &format!("impl {} {{", &msg.name.0));

        // const ID
        self.id(msg, indent);

        // const LEN
        self.size(msg, indent);
        self.write_newline();

        self.deserialize(msg, indent + 4);
        self.write_newline();

        self.serialize(msg, indent + 4);

        // close impl block
        self.write_line(indent, "}");
    }

    fn id(&mut self, msg: &Message, indent: usize) {
        let id_line = match msg.id {
            MessageId::Standard(id) => &format!("pub const ID: u16 = {};", id),
            MessageId::Extended(id) => &format!("pub const ID: u32 = {};", id),
        };

        self.write_line(indent + 4, id_line);
    }

    fn size(&mut self, msg: &Message, indent: usize) {
        self.write_line(indent + 4, &format!("pub const LEN: usize = {};", msg.size));
    }

    fn deserialize(&mut self, msg: &Message, indent: usize) {
        self.write_line(
            indent,
            &format!("pub fn deserialize(data: &[u8; {}]) -> Self {{", msg.size),
        );

        let mut byte = 0usize;

        for sig in &msg.signals {
            let raw = format!("raw_{}", sig.name.0.0);
            self.write_line(
                indent + 4,
                &format!(
                    "let {} = u16::from_le_bytes([data[{}], data[{}]]);",
                    raw,
                    byte,
                    byte + 1
                ),
            );
            byte += 2;
        }

        self.write_newline();
        self.write_line(indent + 4, "Self {");

        for sig in &msg.signals {
            let name = &sig.name.0.0;
            let raw = format!("raw_{}", name);
            self.write_line(
                indent + 8,
                &format!("{}: {} as f32 * {},", name, raw, sig.factor),
            );
        }

        self.write_line(indent + 4, "}");
        self.write_line(indent, "}");
    }

    fn serialize(&mut self, msg: &Message, indent: usize) {
        self.write_line(
            indent,
            &format!("pub fn serialize(&self) -> [u8; {}] {{", msg.size),
        );
        self.write_line(indent + 4, &format!("let mut data = [0u8; {}];", msg.size));
        self.write_newline();

        let mut byte = 0usize;

        for sig in &msg.signals {
            let name = &sig.name.0.0;
            let raw = format!("raw_{}", name);

            self.write_line(
                indent + 4,
                &format!("let {} = (self.{} / {}) as u16;", raw, name, sig.factor),
            );

            self.write_line(
                indent + 4,
                &format!("let {}_bytes = {}.to_le_bytes();", name, raw),
            );
            self.write_line(indent + 4, &format!("data[{}] = {}_bytes[0];", byte, name));
            self.write_line(
                indent + 4,
                &format!("data[{}] = {}_bytes[1];", byte + 1, name),
            );
            self.write_newline();

            byte += 2;
        }

        self.write_line(indent + 4, "data");
        self.write_line(indent, "}");
    }
}
