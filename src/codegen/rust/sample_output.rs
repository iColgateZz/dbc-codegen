#[derive(Debug, Clone)]
pub struct EngineData {
    pub rpm: f32,
    pub speed: f32,
}

impl EngineData {
    pub const ID: u32 = 100;
    pub const LEN: usize = 8;

    pub fn deserialize(data: &[u8; 8]) -> Self {
        let raw_rpm = u16::from_le_bytes([data[0], data[1]]);
        let raw_speed = u16::from_le_bytes([data[2], data[3]]);

        Self {
            rpm: raw_rpm as f32 * 0.125,
            speed: raw_speed as f32 * 0.01,
        }
    }

    pub fn serialize(&self) -> [u8; 8] {
        let mut data = [0u8; 8];

        let raw_rpm = (self.rpm / 0.125) as u16;
        let raw_speed = (self.speed / 0.01) as u16;

        let rpm_bytes = raw_rpm.to_le_bytes();
        let speed_bytes = raw_speed.to_le_bytes();

        data[0] = rpm_bytes[0];
        data[1] = rpm_bytes[1];
        data[2] = speed_bytes[0];
        data[3] = speed_bytes[1];

        data
    }
}