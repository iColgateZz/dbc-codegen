use crate::{DbcFile, codegen::Generator, empty, end_block, line, start_block};

pub struct CppGen;

impl CppGen {
    pub fn generate(file: &DbcFile) -> String {
        let mut out = Generator::new();

        line!(out, "#include <cstdint>");
        line!(out, "#include <cstddef>");
        line!(out, "#include <span>");
        line!(out, "#include <utility>");
        line!(out, "#include <variant>");
        empty!(out);

        for message in &file.messages {
            start_block!(out, "struct {}", message.name.0);
            end_block!(out, "");
            empty!(out);
        }

        out.into_string()
    }
}
