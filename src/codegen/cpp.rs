use heck::{ToSnakeCase};

use crate::{
    DbcFile,
    codegen::Generator,
    empty, end_block,
    ir::{message::{Message, MessageId}, signal::Signal, signal_layout::ByteOrder, signal_value_enum::SignalValueEnum, signal_value_type::CppType},
    line, start_block,
};

pub struct CppGen;

impl CppGen {
    pub fn generate(file: &DbcFile) -> String {
        let mut out = Generator::new();

        line!(out, "#pragma once");
        empty!(out);

        Self::includes(&mut out);
        Self::errors(&mut out);
        Self::endian_read_and_write(&mut out);

        for message in &file.messages {
            Self::message(&mut out, message, file);
        }

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

    fn endian_read_fn(out: &mut Generator, name: &str, swap_if: &str) {
        line!(out, "template <typename T>");
        start_block!(
            out,
            "[[nodiscard]] constexpr T {name}(const uint8_t *d) noexcept"
        );
        line!(out, "T v{{}};");
        line!(out, "std::memcpy(&v, d, sizeof(T));");
        line!(
            out,
            "if constexpr (std::endian::native == std::endian::{swap_if}) v = std::byteswap(v);"
        );
        end_block!(out, "return v;");
        empty!(out);
    }

    fn endian_write_fn(out: &mut Generator, name: &str, swap_if: &str) {
        line!(out, "template <typename T>");
        start_block!(out, "constexpr void {name}(uint8_t *d, T v) noexcept");
        line!(
            out,
            "if constexpr (std::endian::native == std::endian::{swap_if}) v = std::byteswap(v);"
        );
        end_block!(out, "std::memcpy(d, &v, sizeof(T));");
        empty!(out);
    }

    fn extract_le_fn(out: &mut Generator) {
        line!(out, "template <typename T>");
        start_block!(
            out,
            "[[nodiscard]] constexpr T extract_le(const uint8_t* data, std::size_t start, std::size_t end) noexcept"
        );
        line!(out, "using U = std::make_unsigned_t<T>;");
        line!(out, "U result = 0;");
        line!(out, "const std::size_t len = end - start;");
        start_block!(out, "for (std::size_t i = 0; i < len; ++i)");
        line!(out, "const std::size_t bit_idx = start + i;");
        line!(
            out,
            "result |= static_cast<U>((data[bit_idx / 8] >> (bit_idx % 8)) & 0x1u) << i;"
        );
        end_block!(out, "");
        start_block!(out, "if constexpr (std::is_signed_v<T>)");
        start_block!(
            out,
            "if (len < sizeof(U) * 8 && (result & (U(1) << (len - 1))))"
        );
        line!(out, "result |= ~U(0) << len;");
        end_block!(out, "");
        end_block!(out, "");
        end_block!(out, "return static_cast<T>(result);");
        empty!(out);
    }

    fn extract_be_fn(out: &mut Generator) {
        line!(out, "template <typename T>");
        start_block!(
            out,
            "[[nodiscard]] constexpr T extract_be(const uint8_t* data, std::size_t start, std::size_t end) noexcept"
        );
        line!(out, "using U = std::make_unsigned_t<T>;");
        line!(out, "U result = 0;");
        line!(out, "const std::size_t len = end - start;");
        start_block!(out, "for (std::size_t i = 0; i < len; ++i)");
        line!(out, "const std::size_t bit_idx = start + i;");
        line!(
            out,
            "result = (result << 1) | static_cast<U>((data[bit_idx / 8] >> (7 - bit_idx % 8)) & 0x1u);"
        );
        end_block!(out, "");
        start_block!(out, "if constexpr (std::is_signed_v<T>)");
        start_block!(
            out,
            "if (len < sizeof(U) * 8 && (result & (U(1) << (len - 1))))"
        );
        line!(out, "result |= ~U(0) << len;");
        end_block!(out, "");
        end_block!(out, "");
        end_block!(out, "return static_cast<T>(result);");
        empty!(out);
    }

    fn endian_read_and_write(out: &mut Generator) {
        line!(out, "namespace detail {{");
        empty!(out);

        Self::endian_read_fn(out, "read_le", "big");
        Self::endian_read_fn(out, "read_be", "little");
        Self::endian_write_fn(out, "write_le", "big");
        Self::endian_write_fn(out, "write_be", "little");
        Self::extract_le_fn(out);
        Self::extract_be_fn(out);

        line!(out, "}} // namespace detail");
        empty!(out);
    }
    
    fn signal_value_enum(out: &mut Generator, signal: &Signal, enum_def: &SignalValueEnum) {
        let name = &signal.name.upper_camel();
        let cpp_type = &signal.physical_type.as_cpp_type();
        
        start_block!(out, "enum class {} : {}", name, cpp_type);
        for variant in &enum_def.variants {
            line!(out, "{} = {},", variant.description, variant.value)
        }
        end_block!(out, "");
        empty!(out);
        
        line!(out, "[[nodiscard]] constexpr std::expected<{}, CanError>", name);
        start_block!(out, "{}_from_raw({} v) noexcept", name.to_snake_case(), cpp_type);
        start_block!(out, "switch (v)");
        for variant in &enum_def.variants {
            line!(out, "case {}: return {}::{};", variant.value, name, variant.description);
        }
        end_block!(out, "default: return std::unexpected(CanError::InvalidData);");
        end_block!(out, "");
        empty!(out);
    }
    
    fn parse_message(out: &mut Generator, msg: &Message, signals: &Vec<&Signal>, file: &DbcFile) {
        line!(out, "[[nodiscard]] static std::expected<{}, CanError>", msg.name.upper_camel());
        start_block!(out, "parse(std::span<const uint8_t, LEN> data) noexcept");
        for signal in signals {
            let byte_order = file.signal_layouts[signal.layout.0].byte_order;
            let endian = match byte_order {
                ByteOrder::BigEndian => {"read_be"}
                ByteOrder::LittleEndian => {"read_le"}
            };
            let raw_type = signal.raw_type.as_cpp_type();
                
            if let Some(_) = &signal.signal_value_enum {
            } else {
                line!(out, "const {} raw_{} = detail::{}<{}>(&data[0]);", raw_type, signal.name.0.to_snake_case(), endian, raw_type)
            }
        }
        end_block!(out, "");
        empty!(out);
    }

    fn message(out: &mut Generator, msg: &Message, file: &DbcFile) {
        let signals: Vec<_> = msg.signal_idxs.iter().map(|idx| &file.signals[idx.0]).collect();
    
        for signal in &signals {
            if let Some(enum_def) = &signal.signal_value_enum {
                Self::signal_value_enum(out, signal, enum_def);
            }   
        }
        
        start_block!(out, "struct {}", msg.name.upper_camel());
        match msg.id {
            MessageId::Standard(id) => {
                line!(out, "static constexpr uint16_t ID = {};", id);
            }
            MessageId::Extended(id) => {
                line!(out, "static constexpr uint32_t ID = {};", id);
            }
        }
        line!(out, "static constexpr std::size_t LEN = {};", msg.size);
        empty!(out);
        
        for signal in &signals {
            if let Some(_) = &signal.signal_value_enum {
                line!(out, "{} {};", signal.name.upper_camel(), signal.name.0.to_snake_case());
            } else {
                line!(out, "{} {};", signal.physical_type.as_cpp_type(), signal.name.lower());
            }   
        }
        empty!(out);
        
        Self::parse_message(out, msg, &signals, file);
        
        end_block!(out, "");
        empty!(out);
    }
}
