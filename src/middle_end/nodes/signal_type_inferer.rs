use crate::{ir::{signal::{Signal, ValueType}, signal_extended_value_type::ExtendedValueType, signal_value_type::{EnumCoverage, PhysicalType, RawType}}, middle_end::nodes::TransformationNode};

// Determining raw_type
// if SIG_VALTYPE_ exists:
//      if type == 0: 
//          goto integer
//      if type == 1:
//          raw_type = f32
//      if type == 2:
//          raw_type = f64
// else:
//      if signed:
//          raw_type = signed_int(size)
//      else:
//          raw_type = unsigned_int(size)

// Determining logical type
// if VAL_ exists:
//      logical_type = enum
// else:
//      just some value

// Determining physical type
// if logical_type == enum:
//      map integers to enum values
// else:
//      physical_type = float if (raw_type == (f32 or f64)) or factor is float or offset is float
//                      else integer, sign depends on raw_type, factor & offset

// With enums there should be 2 types:
// 1. Where the raw type allows for N values and enum has N entries
// 2. Where the raw type allows for N values and enum has <  N entries

/// Infer singal raw and physical types
pub struct InferSignalType;

impl TransformationNode for InferSignalType {
    fn transform(&self, file: &mut crate::DbcFile) {
        for sig in &mut file.signals {
            sig.raw_type = infer_raw_type(sig);
            sig.physical_type = infer_physical_type(sig);
        }
    }
}

fn infer_raw_type(sig: &Signal) -> RawType {
    match sig.extended_type {
        ExtendedValueType::Float32  => RawType::Float32,
        ExtendedValueType::Double64 => RawType::Float64,
        ExtendedValueType::Integer => match sig.value_type {
            ValueType::Signed   => RawType::SignedInt(sig.size),
            ValueType::Unsigned => RawType::UnsignedInt(sig.size),
        },
    }
}

fn infer_physical_type(sig: &Signal) -> PhysicalType {
    if let Some(variant_count) = sig.signal_value_enum.as_ref().map(|s| s.variants.len()) {
        let possible_values: Option<u64> = 1u64.checked_shl(sig.size as u32);
        let coverage = match possible_values {
            None => EnumCoverage::Partial,
            Some(n) => {
                if variant_count as u64 == n {
                    EnumCoverage::Exhaustive
                } else {
                    EnumCoverage::Partial
                }
            }
        };
        return PhysicalType::Enum(coverage);
    }

    match &sig.raw_type {
        RawType::Float32 => PhysicalType::Float32,
        RawType::Float64 => PhysicalType::Float64,
        RawType::SignedInt(bits) => {
            if is_float_scaled(sig) {
                PhysicalType::Float64
            } else {
                PhysicalType::SignedInt(*bits)
            }
        }
        RawType::UnsignedInt(bits) => {
            if is_float_scaled(sig) {
                PhysicalType::Float64
            } else {
                PhysicalType::UnsignedInt(*bits)
            }
        }
    }
}

fn is_float_scaled(sig: &Signal) -> bool {
    sig.factor.fract() != 0.0 || sig.offset.fract() != 0.0
}