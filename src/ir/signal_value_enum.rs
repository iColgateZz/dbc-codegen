use crate::ir::{ValueDescription, map_into};
use crate::utils::ReprType;
use can_dbc::ValDescription as ParsedValueDescription;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct SignalValueEnumIdx(pub usize);

#[derive(Debug, Clone)]
pub struct SignalValueEnum {
    pub variants: Vec<ValueDescription>,
    pub repr_type: ReprType,
}

impl SignalValueEnum {
    pub fn from_parsed(variants: Vec<ParsedValueDescription>) -> Self {
        Self {
            variants: map_into(variants),
            repr_type: ReprType::default()
        }
    }
}