use embedded_can::{Frame, Id, StandardId, ExtendedId};
use bitvec::prelude::*;
use core::ops::BitOr;
use core::convert::TryFrom;
#[derive(Debug, Clone)]
pub enum CanError {
    UnknownFrameId,
    UnknownMuxValue,
    InvalidPayloadSize,
    InvalidFrameId,
    ValueOutOfRange,
    InvalidEnumValue,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Value1Enum {
    Three,
    Two,
    One,
    Zero,
    _Other(u8),
}
impl From<u8> for Value1Enum {
    fn from(val: u8) -> Self {
        match val {
            3u8 => Value1Enum::Three,
            2u8 => Value1Enum::Two,
            1u8 => Value1Enum::One,
            0u8 => Value1Enum::Zero,
            _ => Value1Enum::_Other(val),
        }
    }
}
impl From<Value1Enum> for u8 {
    fn from(val: Value1Enum) -> Self {
        match val {
            Value1Enum::Three => 3u8,
            Value1Enum::Two => 2u8,
            Value1Enum::One => 1u8,
            Value1Enum::Zero => 0u8,
            Value1Enum::_Other(v) => v,
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Value0Enum {
    Value2,
    Value1,
    V0,
    _Other(u8),
}
impl From<u8> for Value0Enum {
    fn from(val: u8) -> Self {
        match val {
            2u8 => Value0Enum::Value2,
            1u8 => Value0Enum::Value1,
            0u8 => Value0Enum::V0,
            _ => Value0Enum::_Other(val),
        }
    }
}
impl From<Value0Enum> for u8 {
    fn from(val: Value0Enum) -> Self {
        match val {
            Value0Enum::Value2 => 2u8,
            Value0Enum::Value1 => 1u8,
            Value0Enum::V0 => 0u8,
            Value0Enum::_Other(v) => v,
        }
    }
}
pub trait GeneratedCanMessage<const LEN: usize>: Sized {
    fn try_from_frame(frame: &impl Frame) -> Result<Self, CanError>;
    fn encode(&self) -> [u8; LEN];
}
#[derive(Debug, Clone)]
pub enum Msg {
    CanMultiplexed(CanMultiplexedMsg),
    CanMessage(CanMessageMsg),
}
impl Msg {
    fn try_from(frame: &impl Frame) -> Result<Self, CanError> {
        let result = match frame.id() {
            CanMultiplexedMsg::ID => {
                Msg::CanMultiplexed(CanMultiplexedMsg::try_from_frame(frame)?)
            }
            CanMessageMsg::ID => Msg::CanMessage(CanMessageMsg::try_from_frame(frame)?),
            _ => return Err(CanError::UnknownFrameId),
        };
        Ok(result)
    }
}
#[derive(Debug, Clone)]
pub enum CanMultiplexedMsgMux {
    V0(CanMultiplexedMsgMux0),
    V1(CanMultiplexedMsgMux1),
}
#[derive(Debug, Clone, Default)]
pub struct CanMultiplexedMsgMux0 {
    data: [u8; 2usize],
}
impl CanMultiplexedMsgMux0 {
    pub fn new(value0: Value0Enum) -> Result<Self, CanError> {
        let mut msg = Self { data: [0u8; 2usize] };
        msg.set_value0(value0)?;
        Ok(msg)
    }
    ///Value0
    ///- Min: 0
    ///- Max: 0
    ///- Unit:
    ///- Receivers: Node0
    ///- Start bit: 8
    ///- Size: 8 bits
    ///- Factor: 1
    ///- Offset: 0
    ///- Byte order: LittleEndian
    ///- Type: unsigned
    pub fn value0(&self) -> Value0Enum {
        let raw_value0 = self.data.view_bits::<Lsb0>()[8usize..16usize].load_le::<u8>();
        Value0Enum::from(raw_value0 as u8)
    }
    ///Set value of Value0
    ///- Min: 0
    ///- Max: 0
    pub fn set_value0(&mut self, value: Value0Enum) -> Result<(), CanError> {
        self.data.view_bits_mut::<Lsb0>()[8usize..16usize].store_le(u8::from(value));
        Ok(())
    }
}
#[derive(Debug, Clone, Default)]
pub struct CanMultiplexedMsgMux1 {
    data: [u8; 2usize],
}
impl CanMultiplexedMsgMux1 {
    pub fn new(value1: Value1Enum) -> Result<Self, CanError> {
        let mut msg = Self { data: [0u8; 2usize] };
        msg.set_value1(value1)?;
        Ok(msg)
    }
    ///Value1
    ///- Min: 0
    ///- Max: 0
    ///- Unit:
    ///- Receivers: Node1
    ///- Start bit: 8
    ///- Size: 8 bits
    ///- Factor: 1
    ///- Offset: 0
    ///- Byte order: LittleEndian
    ///- Type: unsigned
    pub fn value1(&self) -> Value1Enum {
        let raw_value1 = self.data.view_bits::<Lsb0>()[8usize..16usize].load_le::<u8>();
        Value1Enum::from(raw_value1 as u8)
    }
    ///Set value of Value1
    ///- Min: 0
    ///- Max: 0
    pub fn set_value1(&mut self, value: Value1Enum) -> Result<(), CanError> {
        self.data.view_bits_mut::<Lsb0>()[8usize..16usize].store_le(u8::from(value));
        Ok(())
    }
}
///CANMultiplexed
///- ID: Extended 4321 (0x10E1)
///- Size: 2 bytes
///- Transmitter: Node0
///
///Multiplexed CAN-Message
#[derive(Debug, Clone)]
pub struct CanMultiplexedMsg {
    data: [u8; 2usize],
}
impl CanMultiplexedMsg {
    pub const ID: Id = Id::Extended(unsafe { ExtendedId::new_unchecked(4321u32) });
    pub const LEN: usize = 2usize;
    pub fn new(mux: CanMultiplexedMsgMux) -> Result<Self, CanError> {
        let mut msg = Self { data: [0u8; Self::LEN] };
        match mux {
            CanMultiplexedMsgMux::V0(v) => {
                msg.set_mux0(v)?;
            }
            CanMultiplexedMsgMux::V1(v) => {
                msg.set_mux1(v)?;
            }
        }
        Ok(msg)
    }
    pub fn mux(&self) -> Result<CanMultiplexedMsgMux, CanError> {
        let raw_multiplexer = self
            .data
            .view_bits::<Lsb0>()[0usize..8usize]
            .load_le::<u8>();
        match raw_multiplexer {
            0 => {
                Ok(
                    CanMultiplexedMsgMux::V0(CanMultiplexedMsgMux0 {
                        data: self.data,
                    }),
                )
            }
            1 => {
                Ok(
                    CanMultiplexedMsgMux::V1(CanMultiplexedMsgMux1 {
                        data: self.data,
                    }),
                )
            }
            _ => Err(CanError::UnknownMuxValue),
        }
    }
    pub fn set_mux0(&mut self, value: CanMultiplexedMsgMux0) -> Result<(), CanError> {
        let b0 = BitArray::<_, LocalBits>::new(self.data);
        let b1 = BitArray::<_, LocalBits>::new(value.data);
        self.data = b0.bitor(b1).into_inner();
        self.data.view_bits_mut::<Lsb0>()[0usize..8usize].store_le(0u64);
        Ok(())
    }
    pub fn set_mux1(&mut self, value: CanMultiplexedMsgMux1) -> Result<(), CanError> {
        let b0 = BitArray::<_, LocalBits>::new(self.data);
        let b1 = BitArray::<_, LocalBits>::new(value.data);
        self.data = b0.bitor(b1).into_inner();
        self.data.view_bits_mut::<Lsb0>()[0usize..8usize].store_le(1u64);
        Ok(())
    }
}
impl GeneratedCanMessage<{ Self::LEN }> for CanMultiplexedMsg {
    fn try_from_frame(frame: &impl Frame) -> Result<Self, CanError> {
        if frame.data().len() < Self::LEN {
            return Err(CanError::InvalidPayloadSize);
        }
        if frame.id() != Self::ID {
            return Err(CanError::InvalidFrameId);
        }
        let mut buf = [0u8; Self::LEN];
        buf.copy_from_slice(&frame.data()[..Self::LEN]);
        Ok(Self { data: buf })
    }
    fn encode(&self) -> [u8; Self::LEN] {
        self.data
    }
}
///CANMessage
///- ID: Standard 1234 (0x4D2)
///- Size: 8 bytes
///- Transmitter: Node0
#[derive(Debug, Clone)]
pub struct CanMessageMsg {
    data: [u8; 8usize],
}
impl CanMessageMsg {
    pub const ID: Id = Id::Standard(unsafe { StandardId::new_unchecked(1234u16) });
    pub const LEN: usize = 8usize;
    pub fn new(signal1: u8, signal0: i32) -> Result<Self, CanError> {
        let mut msg = Self { data: [0u8; Self::LEN] };
        msg.set_signal1(signal1)?;
        msg.set_signal0(signal0)?;
        Ok(msg)
    }
    ///Signal1
    ///- Min: 0
    ///- Max: 100
    ///- Unit: %
    ///- Receivers: Node1, Node2
    ///- Start bit: 32
    ///- Size: 32 bits
    ///- Factor: 100
    ///- Offset: 0
    ///- Byte order: LittleEndian
    ///- Type: unsigned
    pub fn signal1(&self) -> u8 {
        let raw_signal1 = self
            .data
            .view_bits::<Lsb0>()[32usize..64usize]
            .load_le::<u32>();
        (raw_signal1 as u8) * (100u8) + (0u8)
    }
    ///Signal0
    ///- Min: 0
    ///- Max: 0
    ///- Unit:
    ///- Receivers: Node1, Node2
    ///- Start bit: 0
    ///- Size: 32 bits
    ///- Factor: 1
    ///- Offset: 0
    ///- Byte order: LittleEndian
    ///- Type: signed
    ///
    ///First signal in this message
    pub fn signal0(&self) -> i32 {
        let raw_signal0 = self
            .data
            .view_bits::<Lsb0>()[0usize..32usize]
            .load_le::<u32>();
        (raw_signal0 as i32) * (1i32) + (0i32)
    }
    ///Set value of Signal1
    ///- Min: 0
    ///- Max: 100
    pub fn set_signal1(&mut self, value: u8) -> Result<(), CanError> {
        if value > 100u8 {
            return Err(CanError::ValueOutOfRange);
        }
        self.data
            .view_bits_mut::<Lsb0>()[32usize..64usize]
            .store_le(((value - (0u8)) / (100u8)) as u32);
        Ok(())
    }
    ///Set value of Signal0
    ///- Min: 0
    ///- Max: 0
    ///
    ///First signal in this message
    pub fn set_signal0(&mut self, value: i32) -> Result<(), CanError> {
        if value < 0i32 || value > 0i32 {
            return Err(CanError::ValueOutOfRange);
        }
        self.data
            .view_bits_mut::<Lsb0>()[0usize..32usize]
            .store_le(((value - (0i32)) / (1i32)) as u32);
        Ok(())
    }
}
impl GeneratedCanMessage<{ Self::LEN }> for CanMessageMsg {
    fn try_from_frame(frame: &impl Frame) -> Result<Self, CanError> {
        if frame.data().len() < Self::LEN {
            return Err(CanError::InvalidPayloadSize);
        }
        if frame.id() != Self::ID {
            return Err(CanError::InvalidFrameId);
        }
        let mut buf = [0u8; Self::LEN];
        buf.copy_from_slice(&frame.data()[..Self::LEN]);
        Ok(Self { data: buf })
    }
    fn encode(&self) -> [u8; Self::LEN] {
        self.data
    }
}
#[cfg(test)]
mod generated_tests {
    use super::*;
    use arbitrary::Unstructured;
    const SEEDS: &[&[u8]] = &[
        &[0u8; 128],
        &[1u8; 128],
        &[2u8; 128],
        &[3u8; 128],
        &[5u8; 128],
        &[8u8; 128],
        &[13u8; 128],
        &[21u8; 128],
        &[34u8; 128],
        &[55u8; 128],
    ];
    #[test]
    fn test_can_message_msg() {
        for seed in SEEDS {
            let mut u = Unstructured::new(seed);
            let signal1_value: u8 = {
                let raw = u
                    .int_in_range(0u8..=100u8)
                    .expect("failed to generate physical interger value");
                raw
            };
            let signal0_value: i32 = {
                let raw = u
                    .int_in_range(0i32..=0i32)
                    .expect("failed to generate physical interger value");
                raw
            };
            let mut msg = CanMessageMsg::new(signal1_value, signal0_value)
                .expect("constructor should accept generated test values");
            assert_eq!(
                msg.signal1(), signal1_value, "getter `{}` returned unexpected value",
                stringify!(signal1),
            );
            assert_eq!(
                msg.signal0(), signal0_value, "getter `{}` returned unexpected value",
                stringify!(signal0),
            );
            let signal1_next_value: u8 = {
                let raw = u
                    .int_in_range(0u8..=100u8)
                    .expect("failed to generate physical interger value");
                raw
            };
            let signal0_next_value: i32 = {
                let raw = u
                    .int_in_range(0i32..=0i32)
                    .expect("failed to generate physical interger value");
                raw
            };
            msg.set_signal1(signal1_next_value)
                .expect("setter should accept generated test value");
            msg.set_signal0(signal0_next_value)
                .expect("setter should accept generated test value");
            assert_eq!(
                msg.signal1(), signal1_next_value,
                "getter `{}` returned unexpected value", stringify!(signal1),
            );
            assert_eq!(
                msg.signal0(), signal0_next_value,
                "getter `{}` returned unexpected value", stringify!(signal0),
            );
        }
    }
}
