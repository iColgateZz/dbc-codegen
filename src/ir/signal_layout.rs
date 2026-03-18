use can_dbc::ByteOrder as ParsedByteOrder;
use can_dbc::ValueType as ParsedValueType;
use can_dbc::Signal as ParsedSignal;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct SignalLayoutIdx(pub usize);

#[derive(Debug, Clone, Copy)]
pub struct SignalLayout {
    pub start_bit: u64,
    pub size: u64,
    pub byte_order: ByteOrder,
    pub value_type: ValueType,
    pub factor: f64,
    pub offset: f64,
    pub min: f64,
    pub max: f64,
}

impl From<&ParsedSignal> for SignalLayout {
    fn from(value: &ParsedSignal) -> Self {
        Self {
            start_bit: value.start_bit,
            size: value.size,
            byte_order: ByteOrder::from(value.byte_order),
            value_type: ValueType::from(value.value_type),
            factor: value.factor,
            offset: value.offset,
            min: value.min,
            max: value.max,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum ByteOrder {
    BigEndian,
    LittleEndian,
}
impl From<ParsedByteOrder> for ByteOrder {
    fn from(value: ParsedByteOrder) -> Self {
        match value {
            ParsedByteOrder::BigEndian => ByteOrder::BigEndian,
            ParsedByteOrder::LittleEndian => ByteOrder::LittleEndian,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum ValueType {
    Unsigned,
    Signed,
}
impl From<ParsedValueType> for ValueType {
    fn from(value: ParsedValueType) -> Self {
        match value {
            ParsedValueType::Signed => ValueType::Signed,
            ParsedValueType::Unsigned => ValueType::Unsigned,
        }
    }
}