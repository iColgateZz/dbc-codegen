use can_dbc::Baudrate;

#[derive(Debug, Clone)]
pub struct BitTiming {
    pub baudrate: u64,
    pub brt1: u32,
    pub brt2: u32,
}
impl From<Baudrate> for BitTiming {
    fn from(value: Baudrate) -> Self {
        BitTiming {
            baudrate: value.0,
            brt1: 0,
            brt2: 0,
        }
    }
}
