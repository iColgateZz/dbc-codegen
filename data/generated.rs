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
pub trait GeneratedCanMessage<const LEN: usize>: Sized {
    fn try_from_frame(frame: &impl Frame) -> Result<Self, CanError>;
    fn encode(&self) -> [u8; LEN];
}
#[derive(Debug, Clone)]
pub enum Msg {
    DriverHeartbeat(DriverHeartbeatMsg),
}
impl Msg {
    fn try_from(frame: &impl Frame) -> Result<Self, CanError> {
        let result = match frame.id() {
            DriverHeartbeatMsg::ID => {
                Msg::DriverHeartbeat(DriverHeartbeatMsg::try_from_frame(frame)?)
            }
            _ => return Err(CanError::UnknownFrameId),
        };
        Ok(result)
    }
}
///DRIVER_HEARTBEAT
///- ID: Standard 100 (0x64)
///- Size: 1 bytes
///- Transmitter: DRIVER
#[derive(Debug, Clone)]
pub struct DriverHeartbeatMsg {
    data: [u8; 1usize],
}
impl DriverHeartbeatMsg {
    pub const ID: Id = Id::Standard(unsafe { StandardId::new_unchecked(100u16) });
    pub const LEN: usize = 1usize;
    pub fn new(temp: i8) -> Result<Self, CanError> {
        let mut msg = Self { data: [0u8; Self::LEN] };
        msg.set_temp(temp)?;
        Ok(msg)
    }
    ///temp
    ///- Min: -128
    ///- Max: 127
    ///- Unit:
    ///- Receivers: SENSOR, MOTOR
    ///- Start bit: 0
    ///- Size: 8 bits
    ///- Factor: 1
    ///- Offset: -128
    ///- Byte order: LittleEndian
    ///- Type: unsigned
    pub fn temp(&self) -> i8 {
        let raw_temp = self.data.view_bits::<Lsb0>()[0usize..8usize].load_le::<u8>();
        (raw_temp as i8) * (1i8) + (-128i8)
    }
    ///Set value of temp
    ///- Min: -128
    ///- Max: 127
    pub fn set_temp(&mut self, value: i8) -> Result<(), CanError> {
        let raw_value = ((value - (-128i8)) / (1i8)) as u8;
        let storage_value = raw_value as u8;
        self.data.view_bits_mut::<Lsb0>()[0usize..8usize].store_le(storage_value);
        Ok(())
    }
}
impl GeneratedCanMessage<{ Self::LEN }> for DriverHeartbeatMsg {
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
    fn test_driver_heartbeat_msg() {
        for seed in SEEDS {
            let mut u = Unstructured::new(seed);
            let temp_value: i8 = {
                let raw = u
                    .int_in_range(-128i8..=127i8)
                    .expect("failed to generate physical interger value");
                raw
            };
            let mut msg = DriverHeartbeatMsg::new(temp_value)
                .expect("constructor should accept generated test values");
            assert_eq!(
                msg.temp(), temp_value, "getter `{}` returned unexpected value",
                stringify!(temp),
            );
            let temp_next_value: i8 = {
                let raw = u
                    .int_in_range(-128i8..=127i8)
                    .expect("failed to generate physical interger value");
                raw
            };
            msg.set_temp(temp_next_value)
                .expect("setter should accept generated test value");
            assert_eq!(
                msg.temp(), temp_next_value, "getter `{}` returned unexpected value",
                stringify!(temp),
            );
        }
    }
}
