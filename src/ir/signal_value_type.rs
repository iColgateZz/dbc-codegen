#[derive(Debug, Clone, PartialEq)]
pub enum RawType {
    Float32,
    Float64,
    UnsignedInt(u64),
    SignedInt(u64),
}

#[derive(Debug, Clone, PartialEq)]
pub enum EnumCoverage {
    Exhaustive,
    Partial,
}

#[derive(Debug, Clone, PartialEq)]
pub enum PhysicalType {
    Float32,
    Float64,
    UnsignedInt(u64),
    SignedInt(u64),
    Enum(EnumCoverage),
}