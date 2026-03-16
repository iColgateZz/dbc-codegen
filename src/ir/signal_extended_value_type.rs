use can_dbc::SignalExtendedValueTypeList;
use can_dbc::SignalExtendedValueType as ParsedExtendedValueType;

#[derive(Debug, Clone)]
pub enum ExtendedValueType {
    Integer,
    Float32,
    Double64,
}

impl From<ParsedExtendedValueType> for ExtendedValueType {
    fn from(value: ParsedExtendedValueType) -> Self {
        match value {
            ParsedExtendedValueType::SignedOrUnsignedInteger => Self::Integer,
            ParsedExtendedValueType::IEEEfloat32Bit => Self::Float32,
            ParsedExtendedValueType::IEEEdouble64bit => Self::Double64,
        }
    }
}

#[derive(Debug, Clone)]
pub struct SignalExtendedValueType {
    pub extended_value_type: ExtendedValueType,
}

impl From<SignalExtendedValueTypeList> for SignalExtendedValueType {
    fn from(value: SignalExtendedValueTypeList) -> Self {
        Self {
            extended_value_type: value.signal_extended_value_type.into()
        }
    }
}