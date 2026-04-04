use crate::{DbcFile, codegen::Generator, empty, end_block, line, start_block};

pub struct CppGen;

impl CppGen {
    pub fn generate(file: &DbcFile) -> String {
        let mut out = Generator::new();

        line!(out, "#pragma once");
        empty!(out);

        Self::includes(&mut out);
        Self::errors(&mut out);
        Self::messages(&mut out, file);

        out.into_string()
    }

    fn includes(out: &mut Generator) {
        const INCLUDES: &[&str] = &[
            "array", "bit", "cstddef", "cstdint", "cstdio", "cstring", "expected", "span",
            "variant",
        ];

        for include in INCLUDES {
            line!(out, "#include <{}>", include);
        }
        empty!(out);
    }

    fn errors(out: &mut Generator) {
        const ERRORS: &[&str] = &["UnknownId", "InvalidLength", "InvalidData"];

        start_block!(out, "enum class CanError : uint8_t");
        for error in ERRORS {
            line!(out, "{},", error);
        }
        end_block!(out, "");
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
