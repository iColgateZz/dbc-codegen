use crate::{DbcFile, codegen::Generator, empty, end_block, line, start_block};

pub struct CppGen;

impl CppGen {
    pub fn generate(file: &DbcFile) -> String {
        let mut out = Generator::new();

        Self::includes(&mut out);
        Self::messages(&mut out, file);

        out.into_string()
    }

    fn includes(out: &mut Generator) {
        line!(out, "#include <cstdint>");
        line!(out, "#include <cstddef>");
        line!(out, "#include <span>");
        line!(out, "#include <utility>");
        line!(out, "#include <variant>");
        empty!(out);
    }
    
    fn messages(out: &mut Generator, file: &DbcFile) {
        for message in &file.messages {
            start_block!(out, "struct {}", message.name.0);
            end_block!(out, "");
            empty!(out);
        }
    }
}
