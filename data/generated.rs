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
pub enum VehicleModeControlEnum {
    Undefined,
    Standby,
    Charge,
    Update,
    Maintenance,
    Operational,
    Administrative,
    Battleshort,
    _Other(u8),
}
impl From<u8> for VehicleModeControlEnum {
    fn from(val: u8) -> Self {
        match val {
            0u8 => VehicleModeControlEnum::Undefined,
            1u8 => VehicleModeControlEnum::Standby,
            2u8 => VehicleModeControlEnum::Charge,
            3u8 => VehicleModeControlEnum::Update,
            4u8 => VehicleModeControlEnum::Maintenance,
            5u8 => VehicleModeControlEnum::Operational,
            6u8 => VehicleModeControlEnum::Administrative,
            7u8 => VehicleModeControlEnum::Battleshort,
            _ => VehicleModeControlEnum::_Other(val),
        }
    }
}
impl From<VehicleModeControlEnum> for u8 {
    fn from(val: VehicleModeControlEnum) -> Self {
        match val {
            VehicleModeControlEnum::Undefined => 0u8,
            VehicleModeControlEnum::Standby => 1u8,
            VehicleModeControlEnum::Charge => 2u8,
            VehicleModeControlEnum::Update => 3u8,
            VehicleModeControlEnum::Maintenance => 4u8,
            VehicleModeControlEnum::Operational => 5u8,
            VehicleModeControlEnum::Administrative => 6u8,
            VehicleModeControlEnum::Battleshort => 7u8,
            VehicleModeControlEnum::_Other(v) => v,
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SteeringModeEnum {
    Normal,
    Pivot,
    Crab,
    _Other(u8),
}
impl From<u8> for SteeringModeEnum {
    fn from(val: u8) -> Self {
        match val {
            0u8 => SteeringModeEnum::Normal,
            1u8 => SteeringModeEnum::Pivot,
            2u8 => SteeringModeEnum::Crab,
            _ => SteeringModeEnum::_Other(val),
        }
    }
}
impl From<SteeringModeEnum> for u8 {
    fn from(val: SteeringModeEnum) -> Self {
        match val {
            SteeringModeEnum::Normal => 0u8,
            SteeringModeEnum::Pivot => 1u8,
            SteeringModeEnum::Crab => 2u8,
            SteeringModeEnum::_Other(v) => v,
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DirectionEnum {
    Undefined,
    Forward,
    Reverse,
    _Other(u8),
}
impl From<u8> for DirectionEnum {
    fn from(val: u8) -> Self {
        match val {
            0u8 => DirectionEnum::Undefined,
            1u8 => DirectionEnum::Forward,
            2u8 => DirectionEnum::Reverse,
            _ => DirectionEnum::_Other(val),
        }
    }
}
impl From<DirectionEnum> for u8 {
    fn from(val: DirectionEnum) -> Self {
        match val {
            DirectionEnum::Undefined => 0u8,
            DirectionEnum::Forward => 1u8,
            DirectionEnum::Reverse => 2u8,
            DirectionEnum::_Other(v) => v,
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GearEnum {
    Undefined,
    First,
    Second,
    Third,
    AutomaticHighGear,
    Automatic,
    Neutral,
    _Other(u8),
}
impl From<u8> for GearEnum {
    fn from(val: u8) -> Self {
        match val {
            0u8 => GearEnum::Undefined,
            1u8 => GearEnum::First,
            2u8 => GearEnum::Second,
            3u8 => GearEnum::Third,
            4u8 => GearEnum::AutomaticHighGear,
            5u8 => GearEnum::Automatic,
            6u8 => GearEnum::Neutral,
            _ => GearEnum::_Other(val),
        }
    }
}
impl From<GearEnum> for u8 {
    fn from(val: GearEnum) -> Self {
        match val {
            GearEnum::Undefined => 0u8,
            GearEnum::First => 1u8,
            GearEnum::Second => 2u8,
            GearEnum::Third => 3u8,
            GearEnum::AutomaticHighGear => 4u8,
            GearEnum::Automatic => 5u8,
            GearEnum::Neutral => 6u8,
            GearEnum::_Other(v) => v,
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LoadEnum {
    Empty,
    Full,
    LoadUndefined,
    Half,
    _Other(u8),
}
impl From<u8> for LoadEnum {
    fn from(val: u8) -> Self {
        match val {
            0u8 => LoadEnum::Empty,
            2u8 => LoadEnum::Full,
            7u8 => LoadEnum::LoadUndefined,
            1u8 => LoadEnum::Half,
            _ => LoadEnum::_Other(val),
        }
    }
}
impl From<LoadEnum> for u8 {
    fn from(val: LoadEnum) -> Self {
        match val {
            LoadEnum::Empty => 0u8,
            LoadEnum::Full => 2u8,
            LoadEnum::LoadUndefined => 7u8,
            LoadEnum::Half => 1u8,
            LoadEnum::_Other(v) => v,
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TerrainEnum {
    Road,
    Track,
    Sand,
    Emergency,
    TerrainUndefined,
    _Other(u8),
}
impl From<u8> for TerrainEnum {
    fn from(val: u8) -> Self {
        match val {
            0u8 => TerrainEnum::Road,
            1u8 => TerrainEnum::Track,
            2u8 => TerrainEnum::Sand,
            3u8 => TerrainEnum::Emergency,
            7u8 => TerrainEnum::TerrainUndefined,
            _ => TerrainEnum::_Other(val),
        }
    }
}
impl From<TerrainEnum> for u8 {
    fn from(val: TerrainEnum) -> Self {
        match val {
            TerrainEnum::Road => 0u8,
            TerrainEnum::Track => 1u8,
            TerrainEnum::Sand => 2u8,
            TerrainEnum::Emergency => 3u8,
            TerrainEnum::TerrainUndefined => 7u8,
            TerrainEnum::_Other(v) => v,
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RideHeightEnum {
    Normal,
    High,
    _Other(u8),
}
impl From<u8> for RideHeightEnum {
    fn from(val: u8) -> Self {
        match val {
            0u8 => RideHeightEnum::Normal,
            1u8 => RideHeightEnum::High,
            _ => RideHeightEnum::_Other(val),
        }
    }
}
impl From<RideHeightEnum> for u8 {
    fn from(val: RideHeightEnum) -> Self {
        match val {
            RideHeightEnum::Normal => 0u8,
            RideHeightEnum::High => 1u8,
            RideHeightEnum::_Other(v) => v,
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VehicleModeStatusEnum {
    Undefined,
    Standby,
    Charge,
    Update,
    Maintenance,
    Operational,
    Administrative,
    Battleshort,
    Estop,
    Startup,
    Shutdown,
    _Other(u8),
}
impl From<u8> for VehicleModeStatusEnum {
    fn from(val: u8) -> Self {
        match val {
            0u8 => VehicleModeStatusEnum::Undefined,
            1u8 => VehicleModeStatusEnum::Standby,
            2u8 => VehicleModeStatusEnum::Charge,
            3u8 => VehicleModeStatusEnum::Update,
            4u8 => VehicleModeStatusEnum::Maintenance,
            5u8 => VehicleModeStatusEnum::Operational,
            6u8 => VehicleModeStatusEnum::Administrative,
            7u8 => VehicleModeStatusEnum::Battleshort,
            8u8 => VehicleModeStatusEnum::Estop,
            9u8 => VehicleModeStatusEnum::Startup,
            10u8 => VehicleModeStatusEnum::Shutdown,
            _ => VehicleModeStatusEnum::_Other(val),
        }
    }
}
impl From<VehicleModeStatusEnum> for u8 {
    fn from(val: VehicleModeStatusEnum) -> Self {
        match val {
            VehicleModeStatusEnum::Undefined => 0u8,
            VehicleModeStatusEnum::Standby => 1u8,
            VehicleModeStatusEnum::Charge => 2u8,
            VehicleModeStatusEnum::Update => 3u8,
            VehicleModeStatusEnum::Maintenance => 4u8,
            VehicleModeStatusEnum::Operational => 5u8,
            VehicleModeStatusEnum::Administrative => 6u8,
            VehicleModeStatusEnum::Battleshort => 7u8,
            VehicleModeStatusEnum::Estop => 8u8,
            VehicleModeStatusEnum::Startup => 9u8,
            VehicleModeStatusEnum::Shutdown => 10u8,
            VehicleModeStatusEnum::_Other(v) => v,
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VehicleModeEnum {
    Undefined,
    Standby,
    Charge,
    Update,
    Maintenance,
    Operational,
    Administrative,
    Battleshort,
    Estop,
    Startup,
    Shutdown,
    _Other(u8),
}
impl From<u8> for VehicleModeEnum {
    fn from(val: u8) -> Self {
        match val {
            0u8 => VehicleModeEnum::Undefined,
            1u8 => VehicleModeEnum::Standby,
            2u8 => VehicleModeEnum::Charge,
            3u8 => VehicleModeEnum::Update,
            4u8 => VehicleModeEnum::Maintenance,
            5u8 => VehicleModeEnum::Operational,
            6u8 => VehicleModeEnum::Administrative,
            7u8 => VehicleModeEnum::Battleshort,
            8u8 => VehicleModeEnum::Estop,
            9u8 => VehicleModeEnum::Startup,
            10u8 => VehicleModeEnum::Shutdown,
            _ => VehicleModeEnum::_Other(val),
        }
    }
}
impl From<VehicleModeEnum> for u8 {
    fn from(val: VehicleModeEnum) -> Self {
        match val {
            VehicleModeEnum::Undefined => 0u8,
            VehicleModeEnum::Standby => 1u8,
            VehicleModeEnum::Charge => 2u8,
            VehicleModeEnum::Update => 3u8,
            VehicleModeEnum::Maintenance => 4u8,
            VehicleModeEnum::Operational => 5u8,
            VehicleModeEnum::Administrative => 6u8,
            VehicleModeEnum::Battleshort => 7u8,
            VehicleModeEnum::Estop => 8u8,
            VehicleModeEnum::Startup => 9u8,
            VehicleModeEnum::Shutdown => 10u8,
            VehicleModeEnum::_Other(v) => v,
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LeftSteeringIgnitionEnum {
    NoChange,
    Off,
    On,
    _Other(u8),
}
impl From<u8> for LeftSteeringIgnitionEnum {
    fn from(val: u8) -> Self {
        match val {
            0u8 => LeftSteeringIgnitionEnum::NoChange,
            1u8 => LeftSteeringIgnitionEnum::Off,
            2u8 => LeftSteeringIgnitionEnum::On,
            _ => LeftSteeringIgnitionEnum::_Other(val),
        }
    }
}
impl From<LeftSteeringIgnitionEnum> for u8 {
    fn from(val: LeftSteeringIgnitionEnum) -> Self {
        match val {
            LeftSteeringIgnitionEnum::NoChange => 0u8,
            LeftSteeringIgnitionEnum::Off => 1u8,
            LeftSteeringIgnitionEnum::On => 2u8,
            LeftSteeringIgnitionEnum::_Other(v) => v,
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RightSteeringIgnitionEnum {
    NoChange,
    Off,
    On,
    _Other(u8),
}
impl From<u8> for RightSteeringIgnitionEnum {
    fn from(val: u8) -> Self {
        match val {
            0u8 => RightSteeringIgnitionEnum::NoChange,
            1u8 => RightSteeringIgnitionEnum::Off,
            2u8 => RightSteeringIgnitionEnum::On,
            _ => RightSteeringIgnitionEnum::_Other(val),
        }
    }
}
impl From<RightSteeringIgnitionEnum> for u8 {
    fn from(val: RightSteeringIgnitionEnum) -> Self {
        match val {
            RightSteeringIgnitionEnum::NoChange => 0u8,
            RightSteeringIgnitionEnum::Off => 1u8,
            RightSteeringIgnitionEnum::On => 2u8,
            RightSteeringIgnitionEnum::_Other(v) => v,
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EaxleIgnitionEnum {
    NoChange,
    Off,
    On,
    _Other(u8),
}
impl From<u8> for EaxleIgnitionEnum {
    fn from(val: u8) -> Self {
        match val {
            0u8 => EaxleIgnitionEnum::NoChange,
            1u8 => EaxleIgnitionEnum::Off,
            2u8 => EaxleIgnitionEnum::On,
            _ => EaxleIgnitionEnum::_Other(val),
        }
    }
}
impl From<EaxleIgnitionEnum> for u8 {
    fn from(val: EaxleIgnitionEnum) -> Self {
        match val {
            EaxleIgnitionEnum::NoChange => 0u8,
            EaxleIgnitionEnum::Off => 1u8,
            EaxleIgnitionEnum::On => 2u8,
            EaxleIgnitionEnum::_Other(v) => v,
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MuxLinearControlEnum {
    ThrottleBrake,
    Torque,
    Speed,
    _Other(u8),
}
impl From<u8> for MuxLinearControlEnum {
    fn from(val: u8) -> Self {
        match val {
            0u8 => MuxLinearControlEnum::ThrottleBrake,
            1u8 => MuxLinearControlEnum::Torque,
            2u8 => MuxLinearControlEnum::Speed,
            _ => MuxLinearControlEnum::_Other(val),
        }
    }
}
impl From<MuxLinearControlEnum> for u8 {
    fn from(val: MuxLinearControlEnum) -> Self {
        match val {
            MuxLinearControlEnum::ThrottleBrake => 0u8,
            MuxLinearControlEnum::Torque => 1u8,
            MuxLinearControlEnum::Speed => 2u8,
            MuxLinearControlEnum::_Other(v) => v,
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MuxAngularControlEnum {
    SteerAngle,
    Curvature,
    YawRate,
    _Other(u8),
}
impl From<u8> for MuxAngularControlEnum {
    fn from(val: u8) -> Self {
        match val {
            0u8 => MuxAngularControlEnum::SteerAngle,
            1u8 => MuxAngularControlEnum::Curvature,
            2u8 => MuxAngularControlEnum::YawRate,
            _ => MuxAngularControlEnum::_Other(val),
        }
    }
}
impl From<MuxAngularControlEnum> for u8 {
    fn from(val: MuxAngularControlEnum) -> Self {
        match val {
            MuxAngularControlEnum::SteerAngle => 0u8,
            MuxAngularControlEnum::Curvature => 1u8,
            MuxAngularControlEnum::YawRate => 2u8,
            MuxAngularControlEnum::_Other(v) => v,
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HvStateControlEnum {
    Off,
    Main,
    Baz,
    Foo,
    _Other(u8),
}
impl From<u8> for HvStateControlEnum {
    fn from(val: u8) -> Self {
        match val {
            0u8 => HvStateControlEnum::Off,
            1u8 => HvStateControlEnum::Main,
            2u8 => HvStateControlEnum::Baz,
            3u8 => HvStateControlEnum::Foo,
            _ => HvStateControlEnum::_Other(val),
        }
    }
}
impl From<HvStateControlEnum> for u8 {
    fn from(val: HvStateControlEnum) -> Self {
        match val {
            HvStateControlEnum::Off => 0u8,
            HvStateControlEnum::Main => 1u8,
            HvStateControlEnum::Baz => 2u8,
            HvStateControlEnum::Foo => 3u8,
            HvStateControlEnum::_Other(v) => v,
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ContactorToSendBootEnum {
    FirstContactor,
    SecondContactor,
    ThirdContactor,
    _Other(u8),
}
impl From<u8> for ContactorToSendBootEnum {
    fn from(val: u8) -> Self {
        match val {
            0u8 => ContactorToSendBootEnum::FirstContactor,
            1u8 => ContactorToSendBootEnum::SecondContactor,
            2u8 => ContactorToSendBootEnum::ThirdContactor,
            _ => ContactorToSendBootEnum::_Other(val),
        }
    }
}
impl From<ContactorToSendBootEnum> for u8 {
    fn from(val: ContactorToSendBootEnum) -> Self {
        match val {
            ContactorToSendBootEnum::FirstContactor => 0u8,
            ContactorToSendBootEnum::SecondContactor => 1u8,
            ContactorToSendBootEnum::ThirdContactor => 2u8,
            ContactorToSendBootEnum::_Other(v) => v,
        }
    }
}
pub trait GeneratedCanMessage<const LEN: usize>: Sized {
    fn try_from_frame(frame: &impl Frame) -> Result<Self, CanError>;
    fn encode(&self) -> [u8; LEN];
}
#[derive(Debug, Clone)]
pub enum Msg {
    GeneralControl(GeneralControlMsg),
    PowerLimits(PowerLimitsMsg),
    QuuxStatus(QuuxStatusMsg),
    FooStatus1(FooStatus1Msg),
    FooStatus2(FooStatus2Msg),
    BazStatus(BazStatusMsg),
    BazTemperatures1(BazTemperatures1Msg),
    BazTemperatures2(BazTemperatures2Msg),
    BarControlAndStatus(BarControlAndStatusMsg),
    FooMaintenanceControl(FooMaintenanceControlMsg),
    ZeroAll(ZeroAllMsg),
    QuuxTimeSyncRequest(QuuxTimeSyncRequestMsg),
    FooTimeSyncRequest(FooTimeSyncRequestMsg),
    BarTimeSyncRequest(BarTimeSyncRequestMsg),
    BazTimeSyncRequest(BazTimeSyncRequestMsg),
    QuuxTimeSyncResponse(QuuxTimeSyncResponseMsg),
    FooTimeSyncResponse(FooTimeSyncResponseMsg),
    BarTimeSyncResponse(BarTimeSyncResponseMsg),
    BazTimeSyncResponse(BazTimeSyncResponseMsg),
    BazAck(BazAckMsg),
    BcuAck(BcuAckMsg),
    FooAck(FooAckMsg),
    QuuxAck(QuuxAckMsg),
    FooLinearControl(FooLinearControlMsg),
    FooAngularControl(FooAngularControlMsg),
    BarMaintenanceControl(BarMaintenanceControlMsg),
    BazMaintenanceControl(BazMaintenanceControlMsg),
    QuuxMaintenanceControl1(QuuxMaintenanceControl1Msg),
    ContactorUpdateResponse(ContactorUpdateResponseMsg),
    ContactorUpdateRequest(ContactorUpdateRequestMsg),
    FooContactorBootRequest(FooContactorBootRequestMsg),
    BarContactorBootRequest(BarContactorBootRequestMsg),
    QuuxMaintenanceControl2(QuuxMaintenanceControl2Msg),
    QuuxMaintenanceControl3(QuuxMaintenanceControl3Msg),
}
impl Msg {
    fn try_from(frame: &impl Frame) -> Result<Self, CanError> {
        let result = match frame.id() {
            GeneralControlMsg::ID => {
                Msg::GeneralControl(GeneralControlMsg::try_from_frame(frame)?)
            }
            PowerLimitsMsg::ID => {
                Msg::PowerLimits(PowerLimitsMsg::try_from_frame(frame)?)
            }
            QuuxStatusMsg::ID => Msg::QuuxStatus(QuuxStatusMsg::try_from_frame(frame)?),
            FooStatus1Msg::ID => Msg::FooStatus1(FooStatus1Msg::try_from_frame(frame)?),
            FooStatus2Msg::ID => Msg::FooStatus2(FooStatus2Msg::try_from_frame(frame)?),
            BazStatusMsg::ID => Msg::BazStatus(BazStatusMsg::try_from_frame(frame)?),
            BazTemperatures1Msg::ID => {
                Msg::BazTemperatures1(BazTemperatures1Msg::try_from_frame(frame)?)
            }
            BazTemperatures2Msg::ID => {
                Msg::BazTemperatures2(BazTemperatures2Msg::try_from_frame(frame)?)
            }
            BarControlAndStatusMsg::ID => {
                Msg::BarControlAndStatus(BarControlAndStatusMsg::try_from_frame(frame)?)
            }
            FooMaintenanceControlMsg::ID => {
                Msg::FooMaintenanceControl(
                    FooMaintenanceControlMsg::try_from_frame(frame)?,
                )
            }
            ZeroAllMsg::ID => Msg::ZeroAll(ZeroAllMsg::try_from_frame(frame)?),
            QuuxTimeSyncRequestMsg::ID => {
                Msg::QuuxTimeSyncRequest(QuuxTimeSyncRequestMsg::try_from_frame(frame)?)
            }
            FooTimeSyncRequestMsg::ID => {
                Msg::FooTimeSyncRequest(FooTimeSyncRequestMsg::try_from_frame(frame)?)
            }
            BarTimeSyncRequestMsg::ID => {
                Msg::BarTimeSyncRequest(BarTimeSyncRequestMsg::try_from_frame(frame)?)
            }
            BazTimeSyncRequestMsg::ID => {
                Msg::BazTimeSyncRequest(BazTimeSyncRequestMsg::try_from_frame(frame)?)
            }
            QuuxTimeSyncResponseMsg::ID => {
                Msg::QuuxTimeSyncResponse(
                    QuuxTimeSyncResponseMsg::try_from_frame(frame)?,
                )
            }
            FooTimeSyncResponseMsg::ID => {
                Msg::FooTimeSyncResponse(FooTimeSyncResponseMsg::try_from_frame(frame)?)
            }
            BarTimeSyncResponseMsg::ID => {
                Msg::BarTimeSyncResponse(BarTimeSyncResponseMsg::try_from_frame(frame)?)
            }
            BazTimeSyncResponseMsg::ID => {
                Msg::BazTimeSyncResponse(BazTimeSyncResponseMsg::try_from_frame(frame)?)
            }
            BazAckMsg::ID => Msg::BazAck(BazAckMsg::try_from_frame(frame)?),
            BcuAckMsg::ID => Msg::BcuAck(BcuAckMsg::try_from_frame(frame)?),
            FooAckMsg::ID => Msg::FooAck(FooAckMsg::try_from_frame(frame)?),
            QuuxAckMsg::ID => Msg::QuuxAck(QuuxAckMsg::try_from_frame(frame)?),
            FooLinearControlMsg::ID => {
                Msg::FooLinearControl(FooLinearControlMsg::try_from_frame(frame)?)
            }
            FooAngularControlMsg::ID => {
                Msg::FooAngularControl(FooAngularControlMsg::try_from_frame(frame)?)
            }
            BarMaintenanceControlMsg::ID => {
                Msg::BarMaintenanceControl(
                    BarMaintenanceControlMsg::try_from_frame(frame)?,
                )
            }
            BazMaintenanceControlMsg::ID => {
                Msg::BazMaintenanceControl(
                    BazMaintenanceControlMsg::try_from_frame(frame)?,
                )
            }
            QuuxMaintenanceControl1Msg::ID => {
                Msg::QuuxMaintenanceControl1(
                    QuuxMaintenanceControl1Msg::try_from_frame(frame)?,
                )
            }
            ContactorUpdateResponseMsg::ID => {
                Msg::ContactorUpdateResponse(
                    ContactorUpdateResponseMsg::try_from_frame(frame)?,
                )
            }
            ContactorUpdateRequestMsg::ID => {
                Msg::ContactorUpdateRequest(
                    ContactorUpdateRequestMsg::try_from_frame(frame)?,
                )
            }
            FooContactorBootRequestMsg::ID => {
                Msg::FooContactorBootRequest(
                    FooContactorBootRequestMsg::try_from_frame(frame)?,
                )
            }
            BarContactorBootRequestMsg::ID => {
                Msg::BarContactorBootRequest(
                    BarContactorBootRequestMsg::try_from_frame(frame)?,
                )
            }
            QuuxMaintenanceControl2Msg::ID => {
                Msg::QuuxMaintenanceControl2(
                    QuuxMaintenanceControl2Msg::try_from_frame(frame)?,
                )
            }
            QuuxMaintenanceControl3Msg::ID => {
                Msg::QuuxMaintenanceControl3(
                    QuuxMaintenanceControl3Msg::try_from_frame(frame)?,
                )
            }
            _ => return Err(CanError::UnknownFrameId),
        };
        Ok(result)
    }
}
///GeneralControl
///- ID: Standard 272 (0x110)
///- Size: 8 bytes
///- Transmitter: Bcu
#[derive(Debug, Clone)]
pub struct GeneralControlMsg {
    data: [u8; 8usize],
}
impl GeneralControlMsg {
    pub const ID: Id = Id::Standard(unsafe { StandardId::new_unchecked(272u16) });
    pub const LEN: usize = 8usize;
    pub fn new(
        vehicle_mode_control: VehicleModeControlEnum,
        payload_hv_control: bool,
        dgs_control: bool,
        clear_faults: bool,
        faults_from_last_n_minutes: u8,
        diesel_preheater_control: bool,
        high_charge_control: bool,
        boost_control: bool,
        park_brake: bool,
        foo_enable: bool,
        steering_mode: SteeringModeEnum,
        differential_lock: bool,
        direction: DirectionEnum,
        gear: GearEnum,
        load: LoadEnum,
        terrain: TerrainEnum,
        ride_height: RideHeightEnum,
        vehicle_mode_status: VehicleModeStatusEnum,
        differential_lock_longitude: bool,
        park_brake_override_acknowledged: bool,
    ) -> Result<Self, CanError> {
        let mut msg = Self { data: [0u8; Self::LEN] };
        msg.set_vehicle_mode_control(vehicle_mode_control)?;
        msg.set_payload_hv_control(payload_hv_control)?;
        msg.set_dgs_control(dgs_control)?;
        msg.set_clear_faults(clear_faults)?;
        msg.set_faults_from_last_n_minutes(faults_from_last_n_minutes)?;
        msg.set_diesel_preheater_control(diesel_preheater_control)?;
        msg.set_high_charge_control(high_charge_control)?;
        msg.set_boost_control(boost_control)?;
        msg.set_park_brake(park_brake)?;
        msg.set_foo_enable(foo_enable)?;
        msg.set_steering_mode(steering_mode)?;
        msg.set_differential_lock(differential_lock)?;
        msg.set_direction(direction)?;
        msg.set_gear(gear)?;
        msg.set_load(load)?;
        msg.set_terrain(terrain)?;
        msg.set_ride_height(ride_height)?;
        msg.set_vehicle_mode_status(vehicle_mode_status)?;
        msg.set_differential_lock_longitude(differential_lock_longitude)?;
        msg.set_park_brake_override_acknowledged(park_brake_override_acknowledged)?;
        Ok(msg)
    }
    ///vehicleModeControl
    ///- Min: 0
    ///- Max: 15
    ///- Unit:
    ///- Receivers: VectorXXX
    ///- Start bit: 0
    ///- Size: 4 bits
    ///- Factor: 1
    ///- Offset: 0
    ///- Byte order: LittleEndian
    ///- Type: unsigned
    ///
    ///Command to all of the TTCs for mode control.
    pub fn vehicle_mode_control(&self) -> VehicleModeControlEnum {
        let raw_vehicle_mode_control = self
            .data
            .view_bits::<Lsb0>()[0usize..4usize]
            .load_le::<u8>();
        VehicleModeControlEnum::from(raw_vehicle_mode_control as u8)
    }
    ///payloadHvControl
    ///- Min: 0
    ///- Max: 1
    ///- Unit:
    ///- Receivers: VectorXXX
    ///- Start bit: 9
    ///- Size: 1 bits
    ///- Factor: 1
    ///- Offset: 0
    ///- Byte order: LittleEndian
    ///- Type: unsigned
    ///
    ///Command to Bar ECU to switch payload HV on and off.
    pub fn payload_hv_control(&self) -> bool {
        let raw_payload_hv_control = self
            .data
            .view_bits::<Lsb0>()[9usize..10usize]
            .load_le::<u8>();
        raw_payload_hv_control == 1
    }
    ///dgsControl
    ///- Min: 0
    ///- Max: 1
    ///- Unit:
    ///- Receivers: VectorXXX
    ///- Start bit: 10
    ///- Size: 1 bits
    ///- Factor: 1
    ///- Offset: 0
    ///- Byte order: LittleEndian
    ///- Type: unsigned
    ///
    ///Command to Baz ECU to switch diesel generator on and off.
    pub fn dgs_control(&self) -> bool {
        let raw_dgs_control = self
            .data
            .view_bits::<Lsb0>()[10usize..11usize]
            .load_le::<u8>();
        raw_dgs_control == 1
    }
    ///clearFaults
    ///- Min: 0
    ///- Max: 1
    ///- Unit:
    ///- Receivers: VectorXXX
    ///- Start bit: 12
    ///- Size: 1 bits
    ///- Factor: 1
    ///- Offset: 0
    ///- Byte order: LittleEndian
    ///- Type: unsigned
    ///
    ///Command to all ECUs to clear fault state.
    pub fn clear_faults(&self) -> bool {
        let raw_clear_faults = self
            .data
            .view_bits::<Lsb0>()[12usize..13usize]
            .load_le::<u8>();
        raw_clear_faults == 1
    }
    ///faultsFromLastNMinutes
    ///- Min: 0
    ///- Max: 255
    ///- Unit: min
    ///- Receivers: VectorXXX
    ///- Start bit: 16
    ///- Size: 8 bits
    ///- Factor: 1
    ///- Offset: 0
    ///- Byte order: LittleEndian
    ///- Type: unsigned
    ///
    ///Request ECUs to send out faults over UDP. 0 - No request.
    pub fn faults_from_last_n_minutes(&self) -> u8 {
        let raw_faults_from_last_n_minutes = self
            .data
            .view_bits::<Lsb0>()[16usize..24usize]
            .load_le::<u8>();
        (raw_faults_from_last_n_minutes) * (1u8) + (0u8)
    }
    ///dieselPreheaterControl
    ///- Min: 0
    ///- Max: 1
    ///- Unit:
    ///- Receivers: VectorXXX
    ///- Start bit: 11
    ///- Size: 1 bits
    ///- Factor: 1
    ///- Offset: 0
    ///- Byte order: LittleEndian
    ///- Type: unsigned
    ///
    ///Command to Baz ECU to switch diesel prehater on and off.
    pub fn diesel_preheater_control(&self) -> bool {
        let raw_diesel_preheater_control = self
            .data
            .view_bits::<Lsb0>()[11usize..12usize]
            .load_le::<u8>();
        raw_diesel_preheater_control == 1
    }
    ///highChargeControl
    ///- Min: 0
    ///- Max: 1
    ///- Unit:
    ///- Receivers: VectorXXX
    ///- Start bit: 14
    ///- Size: 1 bits
    ///- Factor: 1
    ///- Offset: 0
    ///- Byte order: LittleEndian
    ///- Type: unsigned
    ///
    ///Set the power management to higher charge mode, which allows HV battery to be fully charged.
    pub fn high_charge_control(&self) -> bool {
        let raw_high_charge_control = self
            .data
            .view_bits::<Lsb0>()[14usize..15usize]
            .load_le::<u8>();
        raw_high_charge_control == 1
    }
    ///boostControl
    ///- Min: 0
    ///- Max: 1
    ///- Unit:
    ///- Receivers: VectorXXX
    ///- Start bit: 15
    ///- Size: 1 bits
    ///- Factor: 1
    ///- Offset: 0
    ///- Byte order: LittleEndian
    ///- Type: unsigned
    ///
    ///Set the power management Boost flag, which allows battery power to be used.
    pub fn boost_control(&self) -> bool {
        let raw_boost_control = self
            .data
            .view_bits::<Lsb0>()[15usize..16usize]
            .load_le::<u8>();
        raw_boost_control == 1
    }
    ///parkBrake
    ///- Min: 0
    ///- Max: 1
    ///- Unit:
    ///- Receivers: VectorXXX
    ///- Start bit: 8
    ///- Size: 1 bits
    ///- Factor: 1
    ///- Offset: 0
    ///- Byte order: LittleEndian
    ///- Type: unsigned
    pub fn park_brake(&self) -> bool {
        let raw_park_brake = self
            .data
            .view_bits::<Lsb0>()[8usize..9usize]
            .load_le::<u8>();
        raw_park_brake == 1
    }
    ///FooEnable
    ///- Min: 0
    ///- Max: 1
    ///- Unit:
    ///- Receivers: VectorXXX
    ///- Start bit: 40
    ///- Size: 1 bits
    ///- Factor: 1
    ///- Offset: 0
    ///- Byte order: LittleEndian
    ///- Type: unsigned
    ///
    ///Deadman
    pub fn foo_enable(&self) -> bool {
        let raw_foo_enable = self
            .data
            .view_bits::<Lsb0>()[40usize..41usize]
            .load_le::<u8>();
        raw_foo_enable == 1
    }
    ///steeringMode
    ///- Min: 0
    ///- Max: 7
    ///- Unit:
    ///- Receivers: VectorXXX
    ///- Start bit: 29
    ///- Size: 3 bits
    ///- Factor: 1
    ///- Offset: 0
    ///- Byte order: LittleEndian
    ///- Type: unsigned
    pub fn steering_mode(&self) -> SteeringModeEnum {
        let raw_steering_mode = self
            .data
            .view_bits::<Lsb0>()[29usize..32usize]
            .load_le::<u8>();
        SteeringModeEnum::from(raw_steering_mode as u8)
    }
    ///differentialLock
    ///- Min: 0
    ///- Max: 1
    ///- Unit:
    ///- Receivers: VectorXXX
    ///- Start bit: 41
    ///- Size: 1 bits
    ///- Factor: 1
    ///- Offset: 0
    ///- Byte order: LittleEndian
    ///- Type: unsigned
    pub fn differential_lock(&self) -> bool {
        let raw_differential_lock = self
            .data
            .view_bits::<Lsb0>()[41usize..42usize]
            .load_le::<u8>();
        raw_differential_lock == 1
    }
    ///direction
    ///- Min: 0
    ///- Max: 3
    ///- Unit:
    ///- Receivers: VectorXXX
    ///- Start bit: 27
    ///- Size: 2 bits
    ///- Factor: 1
    ///- Offset: 0
    ///- Byte order: LittleEndian
    ///- Type: unsigned
    pub fn direction(&self) -> DirectionEnum {
        let raw_direction = self
            .data
            .view_bits::<Lsb0>()[27usize..29usize]
            .load_le::<u8>();
        DirectionEnum::from(raw_direction as u8)
    }
    ///gear
    ///- Min: 0
    ///- Max: 7
    ///- Unit:
    ///- Receivers: VectorXXX
    ///- Start bit: 24
    ///- Size: 3 bits
    ///- Factor: 1
    ///- Offset: 0
    ///- Byte order: LittleEndian
    ///- Type: unsigned
    pub fn gear(&self) -> GearEnum {
        let raw_gear = self.data.view_bits::<Lsb0>()[24usize..27usize].load_le::<u8>();
        GearEnum::from(raw_gear as u8)
    }
    ///load
    ///- Min: 0
    ///- Max: 7
    ///- Unit:
    ///- Receivers: VectorXXX
    ///- Start bit: 35
    ///- Size: 3 bits
    ///- Factor: 1
    ///- Offset: 0
    ///- Byte order: LittleEndian
    ///- Type: unsigned
    pub fn load(&self) -> LoadEnum {
        let raw_load = self.data.view_bits::<Lsb0>()[35usize..38usize].load_le::<u8>();
        LoadEnum::from(raw_load as u8)
    }
    ///terrain
    ///- Min: 0
    ///- Max: 7
    ///- Unit:
    ///- Receivers: VectorXXX
    ///- Start bit: 32
    ///- Size: 3 bits
    ///- Factor: 1
    ///- Offset: 0
    ///- Byte order: LittleEndian
    ///- Type: unsigned
    pub fn terrain(&self) -> TerrainEnum {
        let raw_terrain = self
            .data
            .view_bits::<Lsb0>()[32usize..35usize]
            .load_le::<u8>();
        TerrainEnum::from(raw_terrain as u8)
    }
    ///rideHeight
    ///- Min: 0
    ///- Max: 1
    ///- Unit:
    ///- Receivers: VectorXXX
    ///- Start bit: 38
    ///- Size: 1 bits
    ///- Factor: 1
    ///- Offset: 0
    ///- Byte order: LittleEndian
    ///- Type: unsigned
    pub fn ride_height(&self) -> RideHeightEnum {
        let raw_ride_height = self
            .data
            .view_bits::<Lsb0>()[38usize..39usize]
            .load_le::<u8>();
        RideHeightEnum::from(raw_ride_height as u8)
    }
    ///vehicleModeStatus
    ///- Min: 0
    ///- Max: 15
    ///- Unit:
    ///- Receivers: VectorXXX
    ///- Start bit: 4
    ///- Size: 4 bits
    ///- Factor: 1
    ///- Offset: 0
    ///- Byte order: LittleEndian
    ///- Type: unsigned
    pub fn vehicle_mode_status(&self) -> VehicleModeStatusEnum {
        let raw_vehicle_mode_status = self
            .data
            .view_bits::<Lsb0>()[4usize..8usize]
            .load_le::<u8>();
        VehicleModeStatusEnum::from(raw_vehicle_mode_status as u8)
    }
    ///differentialLockLongitude
    ///- Min: 0
    ///- Max: 1
    ///- Unit:
    ///- Receivers: VectorXXX
    ///- Start bit: 42
    ///- Size: 1 bits
    ///- Factor: 1
    ///- Offset: 0
    ///- Byte order: LittleEndian
    ///- Type: unsigned
    pub fn differential_lock_longitude(&self) -> bool {
        let raw_differential_lock_longitude = self
            .data
            .view_bits::<Lsb0>()[42usize..43usize]
            .load_le::<u8>();
        raw_differential_lock_longitude == 1
    }
    ///parkBrakeOverrideAcknowledged
    ///- Min: 0
    ///- Max: 1
    ///- Unit:
    ///- Receivers: VectorXXX
    ///- Start bit: 43
    ///- Size: 1 bits
    ///- Factor: 1
    ///- Offset: 0
    ///- Byte order: LittleEndian
    ///- Type: unsigned
    ///
    ///Park brake override acknowledge from sidepanel.
    pub fn park_brake_override_acknowledged(&self) -> bool {
        let raw_park_brake_override_acknowledged = self
            .data
            .view_bits::<Lsb0>()[43usize..44usize]
            .load_le::<u8>();
        raw_park_brake_override_acknowledged == 1
    }
    ///Set value of vehicleModeControl
    ///- Min: 0
    ///- Max: 15
    ///
    ///Command to all of the TTCs for mode control.
    pub fn set_vehicle_mode_control(
        &mut self,
        value: VehicleModeControlEnum,
    ) -> Result<(), CanError> {
        self.data.view_bits_mut::<Lsb0>()[0usize..4usize].store_le(u8::from(value));
        Ok(())
    }
    ///Set value of payloadHvControl
    ///- Min: 0
    ///- Max: 1
    ///
    ///Command to Bar ECU to switch payload HV on and off.
    pub fn set_payload_hv_control(&mut self, value: bool) -> Result<(), CanError> {
        self.data.view_bits_mut::<Lsb0>()[9usize..10usize].store_le(u8::from(value));
        Ok(())
    }
    ///Set value of dgsControl
    ///- Min: 0
    ///- Max: 1
    ///
    ///Command to Baz ECU to switch diesel generator on and off.
    pub fn set_dgs_control(&mut self, value: bool) -> Result<(), CanError> {
        self.data.view_bits_mut::<Lsb0>()[10usize..11usize].store_le(u8::from(value));
        Ok(())
    }
    ///Set value of clearFaults
    ///- Min: 0
    ///- Max: 1
    ///
    ///Command to all ECUs to clear fault state.
    pub fn set_clear_faults(&mut self, value: bool) -> Result<(), CanError> {
        self.data.view_bits_mut::<Lsb0>()[12usize..13usize].store_le(u8::from(value));
        Ok(())
    }
    ///Set value of faultsFromLastNMinutes
    ///- Min: 0
    ///- Max: 255
    ///
    ///Request ECUs to send out faults over UDP. 0 - No request.
    pub fn set_faults_from_last_n_minutes(&mut self, value: u8) -> Result<(), CanError> {
        self.data
            .view_bits_mut::<Lsb0>()[16usize..24usize]
            .store_le((value - (0u8)) / (1u8));
        Ok(())
    }
    ///Set value of dieselPreheaterControl
    ///- Min: 0
    ///- Max: 1
    ///
    ///Command to Baz ECU to switch diesel prehater on and off.
    pub fn set_diesel_preheater_control(&mut self, value: bool) -> Result<(), CanError> {
        self.data.view_bits_mut::<Lsb0>()[11usize..12usize].store_le(u8::from(value));
        Ok(())
    }
    ///Set value of highChargeControl
    ///- Min: 0
    ///- Max: 1
    ///
    ///Set the power management to higher charge mode, which allows HV battery to be fully charged.
    pub fn set_high_charge_control(&mut self, value: bool) -> Result<(), CanError> {
        self.data.view_bits_mut::<Lsb0>()[14usize..15usize].store_le(u8::from(value));
        Ok(())
    }
    ///Set value of boostControl
    ///- Min: 0
    ///- Max: 1
    ///
    ///Set the power management Boost flag, which allows battery power to be used.
    pub fn set_boost_control(&mut self, value: bool) -> Result<(), CanError> {
        self.data.view_bits_mut::<Lsb0>()[15usize..16usize].store_le(u8::from(value));
        Ok(())
    }
    ///Set value of parkBrake
    ///- Min: 0
    ///- Max: 1
    pub fn set_park_brake(&mut self, value: bool) -> Result<(), CanError> {
        self.data.view_bits_mut::<Lsb0>()[8usize..9usize].store_le(u8::from(value));
        Ok(())
    }
    ///Set value of FooEnable
    ///- Min: 0
    ///- Max: 1
    ///
    ///Deadman
    pub fn set_foo_enable(&mut self, value: bool) -> Result<(), CanError> {
        self.data.view_bits_mut::<Lsb0>()[40usize..41usize].store_le(u8::from(value));
        Ok(())
    }
    ///Set value of steeringMode
    ///- Min: 0
    ///- Max: 7
    pub fn set_steering_mode(
        &mut self,
        value: SteeringModeEnum,
    ) -> Result<(), CanError> {
        self.data.view_bits_mut::<Lsb0>()[29usize..32usize].store_le(u8::from(value));
        Ok(())
    }
    ///Set value of differentialLock
    ///- Min: 0
    ///- Max: 1
    pub fn set_differential_lock(&mut self, value: bool) -> Result<(), CanError> {
        self.data.view_bits_mut::<Lsb0>()[41usize..42usize].store_le(u8::from(value));
        Ok(())
    }
    ///Set value of direction
    ///- Min: 0
    ///- Max: 3
    pub fn set_direction(&mut self, value: DirectionEnum) -> Result<(), CanError> {
        self.data.view_bits_mut::<Lsb0>()[27usize..29usize].store_le(u8::from(value));
        Ok(())
    }
    ///Set value of gear
    ///- Min: 0
    ///- Max: 7
    pub fn set_gear(&mut self, value: GearEnum) -> Result<(), CanError> {
        self.data.view_bits_mut::<Lsb0>()[24usize..27usize].store_le(u8::from(value));
        Ok(())
    }
    ///Set value of load
    ///- Min: 0
    ///- Max: 7
    pub fn set_load(&mut self, value: LoadEnum) -> Result<(), CanError> {
        self.data.view_bits_mut::<Lsb0>()[35usize..38usize].store_le(u8::from(value));
        Ok(())
    }
    ///Set value of terrain
    ///- Min: 0
    ///- Max: 7
    pub fn set_terrain(&mut self, value: TerrainEnum) -> Result<(), CanError> {
        self.data.view_bits_mut::<Lsb0>()[32usize..35usize].store_le(u8::from(value));
        Ok(())
    }
    ///Set value of rideHeight
    ///- Min: 0
    ///- Max: 1
    pub fn set_ride_height(&mut self, value: RideHeightEnum) -> Result<(), CanError> {
        self.data.view_bits_mut::<Lsb0>()[38usize..39usize].store_le(u8::from(value));
        Ok(())
    }
    ///Set value of vehicleModeStatus
    ///- Min: 0
    ///- Max: 15
    pub fn set_vehicle_mode_status(
        &mut self,
        value: VehicleModeStatusEnum,
    ) -> Result<(), CanError> {
        self.data.view_bits_mut::<Lsb0>()[4usize..8usize].store_le(u8::from(value));
        Ok(())
    }
    ///Set value of differentialLockLongitude
    ///- Min: 0
    ///- Max: 1
    pub fn set_differential_lock_longitude(
        &mut self,
        value: bool,
    ) -> Result<(), CanError> {
        self.data.view_bits_mut::<Lsb0>()[42usize..43usize].store_le(u8::from(value));
        Ok(())
    }
    ///Set value of parkBrakeOverrideAcknowledged
    ///- Min: 0
    ///- Max: 1
    ///
    ///Park brake override acknowledge from sidepanel.
    pub fn set_park_brake_override_acknowledged(
        &mut self,
        value: bool,
    ) -> Result<(), CanError> {
        self.data.view_bits_mut::<Lsb0>()[43usize..44usize].store_le(u8::from(value));
        Ok(())
    }
}
impl GeneratedCanMessage<{ Self::LEN }> for GeneralControlMsg {
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
///PowerLimits
///- ID: Standard 304 (0x130)
///- Size: 8 bytes
///- Transmitter: PowerManagementEcu
#[derive(Debug, Clone)]
pub struct PowerLimitsMsg {
    data: [u8; 8usize],
}
impl PowerLimitsMsg {
    pub const ID: Id = Id::Standard(unsafe { StandardId::new_unchecked(304u16) });
    pub const LEN: usize = 8usize;
    pub fn new(
        foo_available_power: f32,
        foo_available_regen: f32,
        quux_available_power: f32,
    ) -> Result<Self, CanError> {
        let mut msg = Self { data: [0u8; Self::LEN] };
        msg.set_foo_available_power(foo_available_power)?;
        msg.set_foo_available_regen(foo_available_regen)?;
        msg.set_quux_available_power(quux_available_power)?;
        Ok(msg)
    }
    ///FooAvailablePower
    ///- Min: 0
    ///- Max: 6553.5
    ///- Unit: kW
    ///- Receivers: VectorXXX
    ///- Start bit: 0
    ///- Size: 16 bits
    ///- Factor: 0.1
    ///- Offset: 0
    ///- Byte order: LittleEndian
    ///- Type: unsigned
    pub fn foo_available_power(&self) -> f32 {
        let raw_foo_available_power = self
            .data
            .view_bits::<Lsb0>()[0usize..16usize]
            .load_le::<u16>();
        (raw_foo_available_power as f32) * (0.1f32) + (0f32)
    }
    ///FooAvailableRegen
    ///- Min: 0
    ///- Max: 6553.5
    ///- Unit: kW
    ///- Receivers: VectorXXX
    ///- Start bit: 16
    ///- Size: 16 bits
    ///- Factor: 0.1
    ///- Offset: 0
    ///- Byte order: LittleEndian
    ///- Type: unsigned
    pub fn foo_available_regen(&self) -> f32 {
        let raw_foo_available_regen = self
            .data
            .view_bits::<Lsb0>()[16usize..32usize]
            .load_le::<u16>();
        (raw_foo_available_regen as f32) * (0.1f32) + (0f32)
    }
    ///quuxAvailablePower
    ///- Min: 0
    ///- Max: 6553.5
    ///- Unit: kW
    ///- Receivers: VectorXXX
    ///- Start bit: 32
    ///- Size: 16 bits
    ///- Factor: 0.1
    ///- Offset: 0
    ///- Byte order: LittleEndian
    ///- Type: unsigned
    pub fn quux_available_power(&self) -> f32 {
        let raw_quux_available_power = self
            .data
            .view_bits::<Lsb0>()[32usize..48usize]
            .load_le::<u16>();
        (raw_quux_available_power as f32) * (0.1f32) + (0f32)
    }
    ///Set value of FooAvailablePower
    ///- Min: 0
    ///- Max: 6553.5
    pub fn set_foo_available_power(&mut self, value: f32) -> Result<(), CanError> {
        if value < 0f32 || value > 6553.5f32 {
            return Err(CanError::ValueOutOfRange);
        }
        self.data
            .view_bits_mut::<Lsb0>()[0usize..16usize]
            .store_le(((value - (0f32)) / (0.1f32)) as u16);
        Ok(())
    }
    ///Set value of FooAvailableRegen
    ///- Min: 0
    ///- Max: 6553.5
    pub fn set_foo_available_regen(&mut self, value: f32) -> Result<(), CanError> {
        if value < 0f32 || value > 6553.5f32 {
            return Err(CanError::ValueOutOfRange);
        }
        self.data
            .view_bits_mut::<Lsb0>()[16usize..32usize]
            .store_le(((value - (0f32)) / (0.1f32)) as u16);
        Ok(())
    }
    ///Set value of quuxAvailablePower
    ///- Min: 0
    ///- Max: 6553.5
    pub fn set_quux_available_power(&mut self, value: f32) -> Result<(), CanError> {
        if value < 0f32 || value > 6553.5f32 {
            return Err(CanError::ValueOutOfRange);
        }
        self.data
            .view_bits_mut::<Lsb0>()[32usize..48usize]
            .store_le(((value - (0f32)) / (0.1f32)) as u16);
        Ok(())
    }
}
impl GeneratedCanMessage<{ Self::LEN }> for PowerLimitsMsg {
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
///quuxStatus
///- ID: Standard 320 (0x140)
///- Size: 8 bytes
///- Transmitter: quuxEcu
#[derive(Debug, Clone)]
pub struct QuuxStatusMsg {
    data: [u8; 8usize],
}
impl QuuxStatusMsg {
    pub const ID: Id = Id::Standard(unsafe { StandardId::new_unchecked(320u16) });
    pub const LEN: usize = 8usize;
    pub fn new(
        quux_total_current: u8,
        vehicle_mode: VehicleModeEnum,
    ) -> Result<Self, CanError> {
        let mut msg = Self { data: [0u8; Self::LEN] };
        msg.set_quux_total_current(quux_total_current)?;
        msg.set_vehicle_mode(vehicle_mode)?;
        Ok(msg)
    }
    ///quuxTotalCurrent
    ///- Min: 0
    ///- Max: 0
    ///- Unit: A
    ///- Receivers: VectorXXX
    ///- Start bit: 40
    ///- Size: 8 bits
    ///- Factor: 1
    ///- Offset: 0
    ///- Byte order: LittleEndian
    ///- Type: unsigned
    pub fn quux_total_current(&self) -> u8 {
        let raw_quux_total_current = self
            .data
            .view_bits::<Lsb0>()[40usize..48usize]
            .load_le::<u8>();
        (raw_quux_total_current) * (1u8) + (0u8)
    }
    ///vehicleMode
    ///- Min: 0
    ///- Max: 0
    ///- Unit:
    ///- Receivers: VectorXXX
    ///- Start bit: 0
    ///- Size: 4 bits
    ///- Factor: 1
    ///- Offset: 0
    ///- Byte order: LittleEndian
    ///- Type: unsigned
    pub fn vehicle_mode(&self) -> VehicleModeEnum {
        let raw_vehicle_mode = self
            .data
            .view_bits::<Lsb0>()[0usize..4usize]
            .load_le::<u8>();
        VehicleModeEnum::from(raw_vehicle_mode as u8)
    }
    ///Set value of quuxTotalCurrent
    ///- Min: 0
    ///- Max: 0
    pub fn set_quux_total_current(&mut self, value: u8) -> Result<(), CanError> {
        if value > 0u8 {
            return Err(CanError::ValueOutOfRange);
        }
        self.data
            .view_bits_mut::<Lsb0>()[40usize..48usize]
            .store_le((value - (0u8)) / (1u8));
        Ok(())
    }
    ///Set value of vehicleMode
    ///- Min: 0
    ///- Max: 0
    pub fn set_vehicle_mode(&mut self, value: VehicleModeEnum) -> Result<(), CanError> {
        self.data.view_bits_mut::<Lsb0>()[0usize..4usize].store_le(u8::from(value));
        Ok(())
    }
}
impl GeneratedCanMessage<{ Self::LEN }> for QuuxStatusMsg {
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
///FooStatus1
///- ID: Standard 336 (0x150)
///- Size: 8 bytes
///- Transmitter: FooEcu
#[derive(Debug, Clone)]
pub struct FooStatus1Msg {
    data: [u8; 8usize],
}
impl FooStatus1Msg {
    pub const ID: Id = Id::Standard(unsafe { StandardId::new_unchecked(336u16) });
    pub const LEN: usize = 8usize;
    pub fn new(
        foo_total_current: f32,
        vehicle_speed: f32,
        roll: i8,
        pitch: i8,
    ) -> Result<Self, CanError> {
        let mut msg = Self { data: [0u8; Self::LEN] };
        msg.set_foo_total_current(foo_total_current)?;
        msg.set_vehicle_speed(vehicle_speed)?;
        msg.set_roll(roll)?;
        msg.set_pitch(pitch)?;
        Ok(msg)
    }
    ///FooTotalCurrent
    ///- Min: -3276
    ///- Max: 3276
    ///- Unit: A
    ///- Receivers: VectorXXX
    ///- Start bit: 0
    ///- Size: 16 bits
    ///- Factor: 0.1
    ///- Offset: 0
    ///- Byte order: LittleEndian
    ///- Type: signed
    pub fn foo_total_current(&self) -> f32 {
        let raw_foo_total_current = self
            .data
            .view_bits::<Lsb0>()[0usize..16usize]
            .load_le::<u16>();
        (raw_foo_total_current as f32) * (0.1f32) + (0f32)
    }
    ///vehicleSpeed
    ///- Min: 0
    ///- Max: 35
    ///- Unit: m/s
    ///- Receivers: VectorXXX
    ///- Start bit: 24
    ///- Size: 8 bits
    ///- Factor: 0.14
    ///- Offset: 0
    ///- Byte order: LittleEndian
    ///- Type: unsigned
    pub fn vehicle_speed(&self) -> f32 {
        let raw_vehicle_speed = self
            .data
            .view_bits::<Lsb0>()[24usize..32usize]
            .load_le::<u8>();
        (raw_vehicle_speed as f32) * (0.14f32) + (0f32)
    }
    ///roll
    ///- Min: -128
    ///- Max: 127
    ///- Unit: deg
    ///- Receivers: VectorXXX
    ///- Start bit: 32
    ///- Size: 8 bits
    ///- Factor: 1
    ///- Offset: 0
    ///- Byte order: LittleEndian
    ///- Type: signed
    pub fn roll(&self) -> i8 {
        let raw_roll = self.data.view_bits::<Lsb0>()[32usize..40usize].load_le::<u8>();
        (raw_roll as i8) * (1i8) + (0i8)
    }
    ///pitch
    ///- Min: -128
    ///- Max: 127
    ///- Unit: deg
    ///- Receivers: VectorXXX
    ///- Start bit: 40
    ///- Size: 8 bits
    ///- Factor: 1
    ///- Offset: 0
    ///- Byte order: LittleEndian
    ///- Type: signed
    pub fn pitch(&self) -> i8 {
        let raw_pitch = self.data.view_bits::<Lsb0>()[40usize..48usize].load_le::<u8>();
        (raw_pitch as i8) * (1i8) + (0i8)
    }
    ///Set value of FooTotalCurrent
    ///- Min: -3276
    ///- Max: 3276
    pub fn set_foo_total_current(&mut self, value: f32) -> Result<(), CanError> {
        if value < -3276f32 || value > 3276f32 {
            return Err(CanError::ValueOutOfRange);
        }
        self.data
            .view_bits_mut::<Lsb0>()[0usize..16usize]
            .store_le(((value - (0f32)) / (0.1f32)) as u16);
        Ok(())
    }
    ///Set value of vehicleSpeed
    ///- Min: 0
    ///- Max: 35
    pub fn set_vehicle_speed(&mut self, value: f32) -> Result<(), CanError> {
        if value < 0f32 || value > 35f32 {
            return Err(CanError::ValueOutOfRange);
        }
        self.data
            .view_bits_mut::<Lsb0>()[24usize..32usize]
            .store_le(((value - (0f32)) / (0.14f32)) as u8);
        Ok(())
    }
    ///Set value of roll
    ///- Min: -128
    ///- Max: 127
    pub fn set_roll(&mut self, value: i8) -> Result<(), CanError> {
        self.data
            .view_bits_mut::<Lsb0>()[32usize..40usize]
            .store_le((value - (0i8)) / (1i8));
        Ok(())
    }
    ///Set value of pitch
    ///- Min: -128
    ///- Max: 127
    pub fn set_pitch(&mut self, value: i8) -> Result<(), CanError> {
        self.data
            .view_bits_mut::<Lsb0>()[40usize..48usize]
            .store_le((value - (0i8)) / (1i8));
        Ok(())
    }
}
impl GeneratedCanMessage<{ Self::LEN }> for FooStatus1Msg {
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
///FooStatus2
///- ID: Standard 337 (0x151)
///- Size: 8 bytes
///- Transmitter: FooEcu
#[derive(Debug, Clone)]
pub struct FooStatus2Msg {
    data: [u8; 8usize],
}
impl FooStatus2Msg {
    pub const ID: Id = Id::Standard(unsafe { StandardId::new_unchecked(337u16) });
    pub const LEN: usize = 8usize;
    pub fn new(
        max_inverter_temp: u8,
        max_motor_temp: u8,
        compressor_temp: u8,
        vehicle_mode: VehicleModeEnum,
        park_brake_override_detected: bool,
        foo_sub_contactors_status: bool,
        fooline_enabled: bool,
        compressor_safed: bool,
    ) -> Result<Self, CanError> {
        let mut msg = Self { data: [0u8; Self::LEN] };
        msg.set_max_inverter_temp(max_inverter_temp)?;
        msg.set_max_motor_temp(max_motor_temp)?;
        msg.set_compressor_temp(compressor_temp)?;
        msg.set_vehicle_mode(vehicle_mode)?;
        msg.set_park_brake_override_detected(park_brake_override_detected)?;
        msg.set_foo_sub_contactors_status(foo_sub_contactors_status)?;
        msg.set_fooline_enabled(fooline_enabled)?;
        msg.set_compressor_safed(compressor_safed)?;
        Ok(msg)
    }
    ///maxInverterTemp
    ///- Min: -40
    ///- Max: 215
    ///- Unit: degC
    ///- Receivers: VectorXXX
    ///- Start bit: 8
    ///- Size: 8 bits
    ///- Factor: 1
    ///- Offset: -40
    ///- Byte order: LittleEndian
    ///- Type: unsigned
    pub fn max_inverter_temp(&self) -> u8 {
        let raw_max_inverter_temp = self
            .data
            .view_bits::<Lsb0>()[8usize..16usize]
            .load_le::<u8>();
        (raw_max_inverter_temp) * (1u8) + (216u8)
    }
    ///maxMotorTemp
    ///- Min: -40
    ///- Max: 215
    ///- Unit: degC
    ///- Receivers: VectorXXX
    ///- Start bit: 16
    ///- Size: 8 bits
    ///- Factor: 1
    ///- Offset: -40
    ///- Byte order: LittleEndian
    ///- Type: unsigned
    pub fn max_motor_temp(&self) -> u8 {
        let raw_max_motor_temp = self
            .data
            .view_bits::<Lsb0>()[16usize..24usize]
            .load_le::<u8>();
        (raw_max_motor_temp) * (1u8) + (216u8)
    }
    ///compressorTemp
    ///- Min: -40
    ///- Max: 215
    ///- Unit: degC
    ///- Receivers: VectorXXX
    ///- Start bit: 24
    ///- Size: 8 bits
    ///- Factor: 1
    ///- Offset: -40
    ///- Byte order: LittleEndian
    ///- Type: unsigned
    pub fn compressor_temp(&self) -> u8 {
        let raw_compressor_temp = self
            .data
            .view_bits::<Lsb0>()[24usize..32usize]
            .load_le::<u8>();
        (raw_compressor_temp) * (1u8) + (216u8)
    }
    ///vehicleMode
    ///- Min: 0
    ///- Max: 0
    ///- Unit:
    ///- Receivers: VectorXXX
    ///- Start bit: 0
    ///- Size: 4 bits
    ///- Factor: 1
    ///- Offset: 0
    ///- Byte order: LittleEndian
    ///- Type: unsigned
    pub fn vehicle_mode(&self) -> VehicleModeEnum {
        let raw_vehicle_mode = self
            .data
            .view_bits::<Lsb0>()[0usize..4usize]
            .load_le::<u8>();
        VehicleModeEnum::from(raw_vehicle_mode as u8)
    }
    ///parkBrakeOverrideDetected
    ///- Min: 0
    ///- Max: 1
    ///- Unit:
    ///- Receivers: VectorXXX
    ///- Start bit: 5
    ///- Size: 1 bits
    ///- Factor: 1
    ///- Offset: 0
    ///- Byte order: LittleEndian
    ///- Type: unsigned
    pub fn park_brake_override_detected(&self) -> bool {
        let raw_park_brake_override_detected = self
            .data
            .view_bits::<Lsb0>()[5usize..6usize]
            .load_le::<u8>();
        raw_park_brake_override_detected == 1
    }
    ///FooSubContactorsStatus
    ///- Min: 0
    ///- Max: 1
    ///- Unit:
    ///- Receivers: VectorXXX
    ///- Start bit: 4
    ///- Size: 1 bits
    ///- Factor: 1
    ///- Offset: 0
    ///- Byte order: LittleEndian
    ///- Type: unsigned
    pub fn foo_sub_contactors_status(&self) -> bool {
        let raw_foo_sub_contactors_status = self
            .data
            .view_bits::<Lsb0>()[4usize..5usize]
            .load_le::<u8>();
        raw_foo_sub_contactors_status == 1
    }
    ///FoolineEnabled
    ///- Min: 0
    ///- Max: 0
    ///- Unit:
    ///- Receivers: VectorXXX
    ///- Start bit: 6
    ///- Size: 1 bits
    ///- Factor: 1
    ///- Offset: 0
    ///- Byte order: LittleEndian
    ///- Type: unsigned
    ///
    ///Flag to indicate whether Fooline is in Fooline_Enabled substate while in Operational mode.
    pub fn fooline_enabled(&self) -> bool {
        let raw_fooline_enabled = self
            .data
            .view_bits::<Lsb0>()[6usize..7usize]
            .load_le::<u8>();
        raw_fooline_enabled == 1
    }
    ///compressorSafed
    ///- Min: 0
    ///- Max: 0
    ///- Unit:
    ///- Receivers: VectorXXX
    ///- Start bit: 7
    ///- Size: 1 bits
    ///- Factor: 1
    ///- Offset: 0
    ///- Byte order: LittleEndian
    ///- Type: unsigned
    ///
    ///Flag to indicate whether the air compressor has been completely shut off.
    pub fn compressor_safed(&self) -> bool {
        let raw_compressor_safed = self
            .data
            .view_bits::<Lsb0>()[7usize..8usize]
            .load_le::<u8>();
        raw_compressor_safed == 1
    }
    ///Set value of maxInverterTemp
    ///- Min: -40
    ///- Max: 215
    pub fn set_max_inverter_temp(&mut self, value: u8) -> Result<(), CanError> {
        if value > 215u8 {
            return Err(CanError::ValueOutOfRange);
        }
        self.data
            .view_bits_mut::<Lsb0>()[8usize..16usize]
            .store_le((value - (216u8)) / (1u8));
        Ok(())
    }
    ///Set value of maxMotorTemp
    ///- Min: -40
    ///- Max: 215
    pub fn set_max_motor_temp(&mut self, value: u8) -> Result<(), CanError> {
        if value > 215u8 {
            return Err(CanError::ValueOutOfRange);
        }
        self.data
            .view_bits_mut::<Lsb0>()[16usize..24usize]
            .store_le((value - (216u8)) / (1u8));
        Ok(())
    }
    ///Set value of compressorTemp
    ///- Min: -40
    ///- Max: 215
    pub fn set_compressor_temp(&mut self, value: u8) -> Result<(), CanError> {
        if value > 215u8 {
            return Err(CanError::ValueOutOfRange);
        }
        self.data
            .view_bits_mut::<Lsb0>()[24usize..32usize]
            .store_le((value - (216u8)) / (1u8));
        Ok(())
    }
    ///Set value of vehicleMode
    ///- Min: 0
    ///- Max: 0
    pub fn set_vehicle_mode(&mut self, value: VehicleModeEnum) -> Result<(), CanError> {
        self.data.view_bits_mut::<Lsb0>()[0usize..4usize].store_le(u8::from(value));
        Ok(())
    }
    ///Set value of parkBrakeOverrideDetected
    ///- Min: 0
    ///- Max: 1
    pub fn set_park_brake_override_detected(
        &mut self,
        value: bool,
    ) -> Result<(), CanError> {
        self.data.view_bits_mut::<Lsb0>()[5usize..6usize].store_le(u8::from(value));
        Ok(())
    }
    ///Set value of FooSubContactorsStatus
    ///- Min: 0
    ///- Max: 1
    pub fn set_foo_sub_contactors_status(
        &mut self,
        value: bool,
    ) -> Result<(), CanError> {
        self.data.view_bits_mut::<Lsb0>()[4usize..5usize].store_le(u8::from(value));
        Ok(())
    }
    ///Set value of FoolineEnabled
    ///- Min: 0
    ///- Max: 0
    ///
    ///Flag to indicate whether Fooline is in Fooline_Enabled substate while in Operational mode.
    pub fn set_fooline_enabled(&mut self, value: bool) -> Result<(), CanError> {
        self.data.view_bits_mut::<Lsb0>()[6usize..7usize].store_le(u8::from(value));
        Ok(())
    }
    ///Set value of compressorSafed
    ///- Min: 0
    ///- Max: 0
    ///
    ///Flag to indicate whether the air compressor has been completely shut off.
    pub fn set_compressor_safed(&mut self, value: bool) -> Result<(), CanError> {
        self.data.view_bits_mut::<Lsb0>()[7usize..8usize].store_le(u8::from(value));
        Ok(())
    }
}
impl GeneratedCanMessage<{ Self::LEN }> for FooStatus2Msg {
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
///BazStatus
///- ID: Standard 352 (0x160)
///- Size: 8 bytes
///- Transmitter: PowerManagementEcu
#[derive(Debug, Clone)]
pub struct BazStatusMsg {
    data: [u8; 8usize],
}
impl BazStatusMsg {
    pub const ID: Id = Id::Standard(unsafe { StandardId::new_unchecked(352u16) });
    pub const LEN: usize = 8usize;
    pub fn new(
        hv_battery_contactor_status: bool,
        hv_battery_voltage: f32,
        obc_type2_connected: bool,
        dgs_enabled: bool,
        vehicle_mode: VehicleModeEnum,
        hv_battery_ready: bool,
        disable_ext_precharge_req: bool,
    ) -> Result<Self, CanError> {
        let mut msg = Self { data: [0u8; Self::LEN] };
        msg.set_hv_battery_contactor_status(hv_battery_contactor_status)?;
        msg.set_hv_battery_voltage(hv_battery_voltage)?;
        msg.set_obc_type2_connected(obc_type2_connected)?;
        msg.set_dgs_enabled(dgs_enabled)?;
        msg.set_vehicle_mode(vehicle_mode)?;
        msg.set_hv_battery_ready(hv_battery_ready)?;
        msg.set_disable_ext_precharge_req(disable_ext_precharge_req)?;
        Ok(msg)
    }
    ///hvBatteryContactorStatus
    ///- Min: 0
    ///- Max: 1
    ///- Unit:
    ///- Receivers: VectorXXX
    ///- Start bit: 8
    ///- Size: 1 bits
    ///- Factor: 1
    ///- Offset: 0
    ///- Byte order: LittleEndian
    ///- Type: unsigned
    ///
    ///HV Battery ready to provide power
    pub fn hv_battery_contactor_status(&self) -> bool {
        let raw_hv_battery_contactor_status = self
            .data
            .view_bits::<Lsb0>()[8usize..9usize]
            .load_le::<u8>();
        raw_hv_battery_contactor_status == 1
    }
    ///hvBatteryVoltage
    ///- Min: 0
    ///- Max: 0
    ///- Unit: V
    ///- Receivers: VectorXXX
    ///- Start bit: 40
    ///- Size: 16 bits
    ///- Factor: 0.02
    ///- Offset: 0
    ///- Byte order: LittleEndian
    ///- Type: unsigned
    pub fn hv_battery_voltage(&self) -> f32 {
        let raw_hv_battery_voltage = self
            .data
            .view_bits::<Lsb0>()[40usize..56usize]
            .load_le::<u16>();
        (raw_hv_battery_voltage as f32) * (0.02f32) + (0f32)
    }
    ///obcType2Connected
    ///- Min: 0
    ///- Max: 0
    ///- Unit:
    ///- Receivers: VectorXXX
    ///- Start bit: 9
    ///- Size: 1 bits
    ///- Factor: 1
    ///- Offset: 0
    ///- Byte order: LittleEndian
    ///- Type: unsigned
    pub fn obc_type2_connected(&self) -> bool {
        let raw_obc_type2_connected = self
            .data
            .view_bits::<Lsb0>()[9usize..10usize]
            .load_le::<u8>();
        raw_obc_type2_connected == 1
    }
    ///dgsEnabled
    ///- Min: 0
    ///- Max: 1
    ///- Unit:
    ///- Receivers: VectorXXX
    ///- Start bit: 10
    ///- Size: 1 bits
    ///- Factor: 1
    ///- Offset: 0
    ///- Byte order: LittleEndian
    ///- Type: unsigned
    pub fn dgs_enabled(&self) -> bool {
        let raw_dgs_enabled = self
            .data
            .view_bits::<Lsb0>()[10usize..11usize]
            .load_le::<u8>();
        raw_dgs_enabled == 1
    }
    ///vehicleMode
    ///- Min: 0
    ///- Max: 0
    ///- Unit:
    ///- Receivers: VectorXXX
    ///- Start bit: 0
    ///- Size: 4 bits
    ///- Factor: 1
    ///- Offset: 0
    ///- Byte order: LittleEndian
    ///- Type: unsigned
    pub fn vehicle_mode(&self) -> VehicleModeEnum {
        let raw_vehicle_mode = self
            .data
            .view_bits::<Lsb0>()[0usize..4usize]
            .load_le::<u8>();
        VehicleModeEnum::from(raw_vehicle_mode as u8)
    }
    ///hvBatteryReady
    ///- Min: 0
    ///- Max: 1
    ///- Unit:
    ///- Receivers: VectorXXX
    ///- Start bit: 11
    ///- Size: 1 bits
    ///- Factor: 1
    ///- Offset: 0
    ///- Byte order: LittleEndian
    ///- Type: unsigned
    ///
    ///HV Battery ready to be enabled
    pub fn hv_battery_ready(&self) -> bool {
        let raw_hv_battery_ready = self
            .data
            .view_bits::<Lsb0>()[11usize..12usize]
            .load_le::<u8>();
        raw_hv_battery_ready == 1
    }
    ///disableExtPrechargeReq
    ///- Min: 0
    ///- Max: 1
    ///- Unit:
    ///- Receivers: VectorXXX
    ///- Start bit: 12
    ///- Size: 1 bits
    ///- Factor: 1
    ///- Offset: 0
    ///- Byte order: LittleEndian
    ///- Type: unsigned
    ///
    ///Indicator request to stop external precharge sequence.
    pub fn disable_ext_precharge_req(&self) -> bool {
        let raw_disable_ext_precharge_req = self
            .data
            .view_bits::<Lsb0>()[12usize..13usize]
            .load_le::<u8>();
        raw_disable_ext_precharge_req == 1
    }
    ///Set value of hvBatteryContactorStatus
    ///- Min: 0
    ///- Max: 1
    ///
    ///HV Battery ready to provide power
    pub fn set_hv_battery_contactor_status(
        &mut self,
        value: bool,
    ) -> Result<(), CanError> {
        self.data.view_bits_mut::<Lsb0>()[8usize..9usize].store_le(u8::from(value));
        Ok(())
    }
    ///Set value of hvBatteryVoltage
    ///- Min: 0
    ///- Max: 0
    pub fn set_hv_battery_voltage(&mut self, value: f32) -> Result<(), CanError> {
        if value < 0f32 || value > 0f32 {
            return Err(CanError::ValueOutOfRange);
        }
        self.data
            .view_bits_mut::<Lsb0>()[40usize..56usize]
            .store_le(((value - (0f32)) / (0.02f32)) as u16);
        Ok(())
    }
    ///Set value of obcType2Connected
    ///- Min: 0
    ///- Max: 0
    pub fn set_obc_type2_connected(&mut self, value: bool) -> Result<(), CanError> {
        self.data.view_bits_mut::<Lsb0>()[9usize..10usize].store_le(u8::from(value));
        Ok(())
    }
    ///Set value of dgsEnabled
    ///- Min: 0
    ///- Max: 1
    pub fn set_dgs_enabled(&mut self, value: bool) -> Result<(), CanError> {
        self.data.view_bits_mut::<Lsb0>()[10usize..11usize].store_le(u8::from(value));
        Ok(())
    }
    ///Set value of vehicleMode
    ///- Min: 0
    ///- Max: 0
    pub fn set_vehicle_mode(&mut self, value: VehicleModeEnum) -> Result<(), CanError> {
        self.data.view_bits_mut::<Lsb0>()[0usize..4usize].store_le(u8::from(value));
        Ok(())
    }
    ///Set value of hvBatteryReady
    ///- Min: 0
    ///- Max: 1
    ///
    ///HV Battery ready to be enabled
    pub fn set_hv_battery_ready(&mut self, value: bool) -> Result<(), CanError> {
        self.data.view_bits_mut::<Lsb0>()[11usize..12usize].store_le(u8::from(value));
        Ok(())
    }
    ///Set value of disableExtPrechargeReq
    ///- Min: 0
    ///- Max: 1
    ///
    ///Indicator request to stop external precharge sequence.
    pub fn set_disable_ext_precharge_req(
        &mut self,
        value: bool,
    ) -> Result<(), CanError> {
        self.data.view_bits_mut::<Lsb0>()[12usize..13usize].store_le(u8::from(value));
        Ok(())
    }
}
impl GeneratedCanMessage<{ Self::LEN }> for BazStatusMsg {
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
///BazTemperatures1
///- ID: Standard 353 (0x161)
///- Size: 8 bytes
///- Transmitter: PowerManagementEcu
#[derive(Debug, Clone)]
pub struct BazTemperatures1Msg {
    data: [u8; 8usize],
}
impl BazTemperatures1Msg {
    pub const ID: Id = Id::Standard(unsafe { StandardId::new_unchecked(353u16) });
    pub const LEN: usize = 8usize;
    pub fn new(
        hv_battery_max_temp: u8,
        hv_min_battery_temp: u8,
        obc_buck_boost_temp: u8,
        obc_chassis_temp: u8,
        obc_inverter_temp: u8,
        bc_max_junction_temp: u8,
        obc_ambient_temp: u8,
        gen_ambient_temp: u8,
    ) -> Result<Self, CanError> {
        let mut msg = Self { data: [0u8; Self::LEN] };
        msg.set_hv_battery_max_temp(hv_battery_max_temp)?;
        msg.set_hv_min_battery_temp(hv_min_battery_temp)?;
        msg.set_obc_buck_boost_temp(obc_buck_boost_temp)?;
        msg.set_obc_chassis_temp(obc_chassis_temp)?;
        msg.set_obc_inverter_temp(obc_inverter_temp)?;
        msg.set_bc_max_junction_temp(bc_max_junction_temp)?;
        msg.set_obc_ambient_temp(obc_ambient_temp)?;
        msg.set_gen_ambient_temp(gen_ambient_temp)?;
        Ok(msg)
    }
    ///hvBatteryMaxTemp
    ///- Min: -50
    ///- Max: 205
    ///- Unit: degC
    ///- Receivers: VectorXXX
    ///- Start bit: 0
    ///- Size: 8 bits
    ///- Factor: 1
    ///- Offset: -50
    ///- Byte order: LittleEndian
    ///- Type: unsigned
    ///
    pub fn hv_battery_max_temp(&self) -> u8 {
        let raw_hv_battery_max_temp = self
            .data
            .view_bits::<Lsb0>()[0usize..8usize]
            .load_le::<u8>();
        (raw_hv_battery_max_temp) * (1u8) + (206u8)
    }
    ///hvMinBatteryTemp
    ///- Min: -50
    ///- Max: 205
    ///- Unit: degC
    ///- Receivers: VectorXXX
    ///- Start bit: 8
    ///- Size: 8 bits
    ///- Factor: 1
    ///- Offset: -50
    ///- Byte order: LittleEndian
    ///- Type: unsigned
    pub fn hv_min_battery_temp(&self) -> u8 {
        let raw_hv_min_battery_temp = self
            .data
            .view_bits::<Lsb0>()[8usize..16usize]
            .load_le::<u8>();
        (raw_hv_min_battery_temp) * (1u8) + (206u8)
    }
    ///obcBuckBoostTemp
    ///- Min: -50
    ///- Max: 205
    ///- Unit: degC
    ///- Receivers: VectorXXX
    ///- Start bit: 16
    ///- Size: 8 bits
    ///- Factor: 1
    ///- Offset: -50
    ///- Byte order: LittleEndian
    ///- Type: unsigned
    pub fn obc_buck_boost_temp(&self) -> u8 {
        let raw_obc_buck_boost_temp = self
            .data
            .view_bits::<Lsb0>()[16usize..24usize]
            .load_le::<u8>();
        (raw_obc_buck_boost_temp) * (1u8) + (206u8)
    }
    ///obcChassisTemp
    ///- Min: -50
    ///- Max: 205
    ///- Unit: degC
    ///- Receivers: VectorXXX
    ///- Start bit: 24
    ///- Size: 8 bits
    ///- Factor: 1
    ///- Offset: -50
    ///- Byte order: LittleEndian
    ///- Type: unsigned
    pub fn obc_chassis_temp(&self) -> u8 {
        let raw_obc_chassis_temp = self
            .data
            .view_bits::<Lsb0>()[24usize..32usize]
            .load_le::<u8>();
        (raw_obc_chassis_temp) * (1u8) + (206u8)
    }
    ///obcInverterTemp
    ///- Min: -50
    ///- Max: 205
    ///- Unit: degC
    ///- Receivers: VectorXXX
    ///- Start bit: 32
    ///- Size: 8 bits
    ///- Factor: 1
    ///- Offset: -50
    ///- Byte order: LittleEndian
    ///- Type: unsigned
    pub fn obc_inverter_temp(&self) -> u8 {
        let raw_obc_inverter_temp = self
            .data
            .view_bits::<Lsb0>()[32usize..40usize]
            .load_le::<u8>();
        (raw_obc_inverter_temp) * (1u8) + (206u8)
    }
    ///bcMaxJunctionTemp
    ///- Min: -40
    ///- Max: 210
    ///- Unit: degC
    ///- Receivers: VectorXXX
    ///- Start bit: 40
    ///- Size: 8 bits
    ///- Factor: 1
    ///- Offset: -40
    ///- Byte order: LittleEndian
    ///- Type: unsigned
    pub fn bc_max_junction_temp(&self) -> u8 {
        let raw_bc_max_junction_temp = self
            .data
            .view_bits::<Lsb0>()[40usize..48usize]
            .load_le::<u8>();
        (raw_bc_max_junction_temp) * (1u8) + (216u8)
    }
    ///obcAmbientTemp
    ///- Min: -50
    ///- Max: 205
    ///- Unit: degC
    ///- Receivers: VectorXXX
    ///- Start bit: 48
    ///- Size: 8 bits
    ///- Factor: 1
    ///- Offset: -50
    ///- Byte order: LittleEndian
    ///- Type: unsigned
    pub fn obc_ambient_temp(&self) -> u8 {
        let raw_obc_ambient_temp = self
            .data
            .view_bits::<Lsb0>()[48usize..56usize]
            .load_le::<u8>();
        (raw_obc_ambient_temp) * (1u8) + (206u8)
    }
    ///genAmbientTemp
    ///- Min: -40
    ///- Max: 215
    ///- Unit: degC
    ///- Receivers: VectorXXX
    ///- Start bit: 56
    ///- Size: 8 bits
    ///- Factor: 1
    ///- Offset: -40
    ///- Byte order: LittleEndian
    ///- Type: unsigned
    pub fn gen_ambient_temp(&self) -> u8 {
        let raw_gen_ambient_temp = self
            .data
            .view_bits::<Lsb0>()[56usize..64usize]
            .load_le::<u8>();
        (raw_gen_ambient_temp) * (1u8) + (216u8)
    }
    ///Set value of hvBatteryMaxTemp
    ///- Min: -50
    ///- Max: 205
    ///
    pub fn set_hv_battery_max_temp(&mut self, value: u8) -> Result<(), CanError> {
        if value > 205u8 {
            return Err(CanError::ValueOutOfRange);
        }
        self.data
            .view_bits_mut::<Lsb0>()[0usize..8usize]
            .store_le((value - (206u8)) / (1u8));
        Ok(())
    }
    ///Set value of hvMinBatteryTemp
    ///- Min: -50
    ///- Max: 205
    pub fn set_hv_min_battery_temp(&mut self, value: u8) -> Result<(), CanError> {
        if value > 205u8 {
            return Err(CanError::ValueOutOfRange);
        }
        self.data
            .view_bits_mut::<Lsb0>()[8usize..16usize]
            .store_le((value - (206u8)) / (1u8));
        Ok(())
    }
    ///Set value of obcBuckBoostTemp
    ///- Min: -50
    ///- Max: 205
    pub fn set_obc_buck_boost_temp(&mut self, value: u8) -> Result<(), CanError> {
        if value > 205u8 {
            return Err(CanError::ValueOutOfRange);
        }
        self.data
            .view_bits_mut::<Lsb0>()[16usize..24usize]
            .store_le((value - (206u8)) / (1u8));
        Ok(())
    }
    ///Set value of obcChassisTemp
    ///- Min: -50
    ///- Max: 205
    pub fn set_obc_chassis_temp(&mut self, value: u8) -> Result<(), CanError> {
        if value > 205u8 {
            return Err(CanError::ValueOutOfRange);
        }
        self.data
            .view_bits_mut::<Lsb0>()[24usize..32usize]
            .store_le((value - (206u8)) / (1u8));
        Ok(())
    }
    ///Set value of obcInverterTemp
    ///- Min: -50
    ///- Max: 205
    pub fn set_obc_inverter_temp(&mut self, value: u8) -> Result<(), CanError> {
        if value > 205u8 {
            return Err(CanError::ValueOutOfRange);
        }
        self.data
            .view_bits_mut::<Lsb0>()[32usize..40usize]
            .store_le((value - (206u8)) / (1u8));
        Ok(())
    }
    ///Set value of bcMaxJunctionTemp
    ///- Min: -40
    ///- Max: 210
    pub fn set_bc_max_junction_temp(&mut self, value: u8) -> Result<(), CanError> {
        if value > 210u8 {
            return Err(CanError::ValueOutOfRange);
        }
        self.data
            .view_bits_mut::<Lsb0>()[40usize..48usize]
            .store_le((value - (216u8)) / (1u8));
        Ok(())
    }
    ///Set value of obcAmbientTemp
    ///- Min: -50
    ///- Max: 205
    pub fn set_obc_ambient_temp(&mut self, value: u8) -> Result<(), CanError> {
        if value > 205u8 {
            return Err(CanError::ValueOutOfRange);
        }
        self.data
            .view_bits_mut::<Lsb0>()[48usize..56usize]
            .store_le((value - (206u8)) / (1u8));
        Ok(())
    }
    ///Set value of genAmbientTemp
    ///- Min: -40
    ///- Max: 215
    pub fn set_gen_ambient_temp(&mut self, value: u8) -> Result<(), CanError> {
        if value > 215u8 {
            return Err(CanError::ValueOutOfRange);
        }
        self.data
            .view_bits_mut::<Lsb0>()[56usize..64usize]
            .store_le((value - (216u8)) / (1u8));
        Ok(())
    }
}
impl GeneratedCanMessage<{ Self::LEN }> for BazTemperatures1Msg {
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
///BazTemperatures2
///- ID: Standard 354 (0x162)
///- Size: 8 bytes
///- Transmitter: PowerManagementEcu
#[derive(Debug, Clone)]
pub struct BazTemperatures2Msg {
    data: [u8; 8usize],
}
impl BazTemperatures2Msg {
    pub const ID: Id = Id::Standard(unsafe { StandardId::new_unchecked(354u16) });
    pub const LEN: usize = 8usize;
    pub fn new(
        engine_coolant_temp: u8,
        engine_fuel_temp: u8,
        intake_manifold_temp: u8,
        gen_motor_temp: u8,
        gen_emi_temp: u8,
        hv_battery_avg_temp: u8,
    ) -> Result<Self, CanError> {
        let mut msg = Self { data: [0u8; Self::LEN] };
        msg.set_engine_coolant_temp(engine_coolant_temp)?;
        msg.set_engine_fuel_temp(engine_fuel_temp)?;
        msg.set_intake_manifold_temp(intake_manifold_temp)?;
        msg.set_gen_motor_temp(gen_motor_temp)?;
        msg.set_gen_emi_temp(gen_emi_temp)?;
        msg.set_hv_battery_avg_temp(hv_battery_avg_temp)?;
        Ok(msg)
    }
    ///engineCoolantTemp
    ///- Min: -40
    ///- Max: 210
    ///- Unit: degC
    ///- Receivers: VectorXXX
    ///- Start bit: 0
    ///- Size: 8 bits
    ///- Factor: 1
    ///- Offset: -40
    ///- Byte order: LittleEndian
    ///- Type: unsigned
    pub fn engine_coolant_temp(&self) -> u8 {
        let raw_engine_coolant_temp = self
            .data
            .view_bits::<Lsb0>()[0usize..8usize]
            .load_le::<u8>();
        (raw_engine_coolant_temp) * (1u8) + (216u8)
    }
    ///engineFuelTemp
    ///- Min: -40
    ///- Max: 210
    ///- Unit: degC
    ///- Receivers: VectorXXX
    ///- Start bit: 8
    ///- Size: 8 bits
    ///- Factor: 1
    ///- Offset: -40
    ///- Byte order: LittleEndian
    ///- Type: unsigned
    pub fn engine_fuel_temp(&self) -> u8 {
        let raw_engine_fuel_temp = self
            .data
            .view_bits::<Lsb0>()[8usize..16usize]
            .load_le::<u8>();
        (raw_engine_fuel_temp) * (1u8) + (216u8)
    }
    ///intakeManifoldTemp
    ///- Min: -40
    ///- Max: 210
    ///- Unit: degC
    ///- Receivers: VectorXXX
    ///- Start bit: 16
    ///- Size: 8 bits
    ///- Factor: 1
    ///- Offset: -40
    ///- Byte order: LittleEndian
    ///- Type: unsigned
    pub fn intake_manifold_temp(&self) -> u8 {
        let raw_intake_manifold_temp = self
            .data
            .view_bits::<Lsb0>()[16usize..24usize]
            .load_le::<u8>();
        (raw_intake_manifold_temp) * (1u8) + (216u8)
    }
    ///genMotorTemp
    ///- Min: -40
    ///- Max: 210
    ///- Unit: degC
    ///- Receivers: VectorXXX
    ///- Start bit: 32
    ///- Size: 8 bits
    ///- Factor: 1
    ///- Offset: -40
    ///- Byte order: LittleEndian
    ///- Type: unsigned
    pub fn gen_motor_temp(&self) -> u8 {
        let raw_gen_motor_temp = self
            .data
            .view_bits::<Lsb0>()[32usize..40usize]
            .load_le::<u8>();
        (raw_gen_motor_temp) * (1u8) + (216u8)
    }
    ///genEmiTemp
    ///- Min: -40
    ///- Max: 210
    ///- Unit: degC
    ///- Receivers: VectorXXX
    ///- Start bit: 40
    ///- Size: 8 bits
    ///- Factor: 1
    ///- Offset: -40
    ///- Byte order: LittleEndian
    ///- Type: unsigned
    ///
    ///ElectroMagnetic Interference filter temperature
    pub fn gen_emi_temp(&self) -> u8 {
        let raw_gen_emi_temp = self
            .data
            .view_bits::<Lsb0>()[40usize..48usize]
            .load_le::<u8>();
        (raw_gen_emi_temp) * (1u8) + (216u8)
    }
    ///hvBatteryAvgTemp
    ///- Min: -50
    ///- Max: 205
    ///- Unit: degC
    ///- Receivers: VectorXXX
    ///- Start bit: 48
    ///- Size: 8 bits
    ///- Factor: 1
    ///- Offset: -50
    ///- Byte order: LittleEndian
    ///- Type: unsigned
    pub fn hv_battery_avg_temp(&self) -> u8 {
        let raw_hv_battery_avg_temp = self
            .data
            .view_bits::<Lsb0>()[48usize..56usize]
            .load_le::<u8>();
        (raw_hv_battery_avg_temp) * (1u8) + (206u8)
    }
    ///Set value of engineCoolantTemp
    ///- Min: -40
    ///- Max: 210
    pub fn set_engine_coolant_temp(&mut self, value: u8) -> Result<(), CanError> {
        if value > 210u8 {
            return Err(CanError::ValueOutOfRange);
        }
        self.data
            .view_bits_mut::<Lsb0>()[0usize..8usize]
            .store_le((value - (216u8)) / (1u8));
        Ok(())
    }
    ///Set value of engineFuelTemp
    ///- Min: -40
    ///- Max: 210
    pub fn set_engine_fuel_temp(&mut self, value: u8) -> Result<(), CanError> {
        if value > 210u8 {
            return Err(CanError::ValueOutOfRange);
        }
        self.data
            .view_bits_mut::<Lsb0>()[8usize..16usize]
            .store_le((value - (216u8)) / (1u8));
        Ok(())
    }
    ///Set value of intakeManifoldTemp
    ///- Min: -40
    ///- Max: 210
    pub fn set_intake_manifold_temp(&mut self, value: u8) -> Result<(), CanError> {
        if value > 210u8 {
            return Err(CanError::ValueOutOfRange);
        }
        self.data
            .view_bits_mut::<Lsb0>()[16usize..24usize]
            .store_le((value - (216u8)) / (1u8));
        Ok(())
    }
    ///Set value of genMotorTemp
    ///- Min: -40
    ///- Max: 210
    pub fn set_gen_motor_temp(&mut self, value: u8) -> Result<(), CanError> {
        if value > 210u8 {
            return Err(CanError::ValueOutOfRange);
        }
        self.data
            .view_bits_mut::<Lsb0>()[32usize..40usize]
            .store_le((value - (216u8)) / (1u8));
        Ok(())
    }
    ///Set value of genEmiTemp
    ///- Min: -40
    ///- Max: 210
    ///
    ///ElectroMagnetic Interference filter temperature
    pub fn set_gen_emi_temp(&mut self, value: u8) -> Result<(), CanError> {
        if value > 210u8 {
            return Err(CanError::ValueOutOfRange);
        }
        self.data
            .view_bits_mut::<Lsb0>()[40usize..48usize]
            .store_le((value - (216u8)) / (1u8));
        Ok(())
    }
    ///Set value of hvBatteryAvgTemp
    ///- Min: -50
    ///- Max: 205
    pub fn set_hv_battery_avg_temp(&mut self, value: u8) -> Result<(), CanError> {
        if value > 205u8 {
            return Err(CanError::ValueOutOfRange);
        }
        self.data
            .view_bits_mut::<Lsb0>()[48usize..56usize]
            .store_le((value - (206u8)) / (1u8));
        Ok(())
    }
}
impl GeneratedCanMessage<{ Self::LEN }> for BazTemperatures2Msg {
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
///BarControlAndStatus
///- ID: Standard 288 (0x120)
///- Size: 8 bytes
///- Transmitter: BarEcu
#[derive(Debug, Clone)]
pub struct BarControlAndStatusMsg {
    data: [u8; 8usize],
}
impl BarControlAndStatusMsg {
    pub const ID: Id = Id::Standard(unsafe { StandardId::new_unchecked(288u16) });
    pub const LEN: usize = 8usize;
    pub fn new(
        hv_battery_contactor_control: bool,
        foo_sub_contactors_control: bool,
        bc_discharge_control: bool,
        hv_off_control: bool,
        payload_hv_status: bool,
        vehicle_mode: VehicleModeEnum,
        lv_on_status: bool,
    ) -> Result<Self, CanError> {
        let mut msg = Self { data: [0u8; Self::LEN] };
        msg.set_hv_battery_contactor_control(hv_battery_contactor_control)?;
        msg.set_foo_sub_contactors_control(foo_sub_contactors_control)?;
        msg.set_bc_discharge_control(bc_discharge_control)?;
        msg.set_hv_off_control(hv_off_control)?;
        msg.set_payload_hv_status(payload_hv_status)?;
        msg.set_vehicle_mode(vehicle_mode)?;
        msg.set_lv_on_status(lv_on_status)?;
        Ok(msg)
    }
    ///hvBatteryContactorControl
    ///- Min: 0
    ///- Max: 1
    ///- Unit:
    ///- Receivers: VectorXXX
    ///- Start bit: 8
    ///- Size: 1 bits
    ///- Factor: 1
    ///- Offset: 0
    ///- Byte order: LittleEndian
    ///- Type: unsigned
    ///
    ///Control signal to Baz ECU to change the HV battery pack contactor state.
    pub fn hv_battery_contactor_control(&self) -> bool {
        let raw_hv_battery_contactor_control = self
            .data
            .view_bits::<Lsb0>()[8usize..9usize]
            .load_le::<u8>();
        raw_hv_battery_contactor_control == 1
    }
    ///FooSubContactorsControl
    ///- Min: 0
    ///- Max: 1
    ///- Unit:
    ///- Receivers: VectorXXX
    ///- Start bit: 9
    ///- Size: 1 bits
    ///- Factor: 1
    ///- Offset: 0
    ///- Byte order: LittleEndian
    ///- Type: unsigned
    ///
    ///Control signal to Foo ECU to change contactor state.
    pub fn foo_sub_contactors_control(&self) -> bool {
        let raw_foo_sub_contactors_control = self
            .data
            .view_bits::<Lsb0>()[9usize..10usize]
            .load_le::<u8>();
        raw_foo_sub_contactors_control == 1
    }
    ///bcDischargeControl
    ///- Min: 0
    ///- Max: 1
    ///- Unit:
    ///- Receivers: VectorXXX
    ///- Start bit: 11
    ///- Size: 1 bits
    ///- Factor: 1
    ///- Offset: 0
    ///- Byte order: LittleEndian
    ///- Type: unsigned
    ///
    ///Control signal to Baz ECU to discharge the HV bus.
    pub fn bc_discharge_control(&self) -> bool {
        let raw_bc_discharge_control = self
            .data
            .view_bits::<Lsb0>()[11usize..12usize]
            .load_le::<u8>();
        raw_bc_discharge_control == 1
    }
    ///hvOffControl
    ///- Min: 0
    ///- Max: 1
    ///- Unit:
    ///- Receivers: VectorXXX
    ///- Start bit: 13
    ///- Size: 1 bits
    ///- Factor: 1
    ///- Offset: 0
    ///- Byte order: LittleEndian
    ///- Type: unsigned
    ///
    ///Control signal to indicate TTC ECUs to start HV shutdown process.
    pub fn hv_off_control(&self) -> bool {
        let raw_hv_off_control = self
            .data
            .view_bits::<Lsb0>()[13usize..14usize]
            .load_le::<u8>();
        raw_hv_off_control == 1
    }
    ///payloadHvStatus
    ///- Min: 0
    ///- Max: 1
    ///- Unit:
    ///- Receivers: VectorXXX
    ///- Start bit: 12
    ///- Size: 1 bits
    ///- Factor: 1
    ///- Offset: 0
    ///- Byte order: LittleEndian
    ///- Type: unsigned
    ///
    ///Status signal to BCU to indicate current state of the payload HV.
    pub fn payload_hv_status(&self) -> bool {
        let raw_payload_hv_status = self
            .data
            .view_bits::<Lsb0>()[12usize..13usize]
            .load_le::<u8>();
        raw_payload_hv_status == 1
    }
    ///vehicleMode
    ///- Min: 0
    ///- Max: 0
    ///- Unit:
    ///- Receivers: VectorXXX
    ///- Start bit: 0
    ///- Size: 4 bits
    ///- Factor: 1
    ///- Offset: 0
    ///- Byte order: LittleEndian
    ///- Type: unsigned
    pub fn vehicle_mode(&self) -> VehicleModeEnum {
        let raw_vehicle_mode = self
            .data
            .view_bits::<Lsb0>()[0usize..4usize]
            .load_le::<u8>();
        VehicleModeEnum::from(raw_vehicle_mode as u8)
    }
    ///lvOnStatus
    ///- Min: 0
    ///- Max: 1
    ///- Unit: -
    ///- Receivers: VectorXXX
    ///- Start bit: 10
    ///- Size: 1 bits
    ///- Factor: 1
    ///- Offset: 0
    ///- Byte order: LittleEndian
    ///- Type: unsigned
    ///
    ///Indicates whether LV bus has voltage and is the main contactor closed
    pub fn lv_on_status(&self) -> bool {
        let raw_lv_on_status = self
            .data
            .view_bits::<Lsb0>()[10usize..11usize]
            .load_le::<u8>();
        raw_lv_on_status == 1
    }
    ///Set value of hvBatteryContactorControl
    ///- Min: 0
    ///- Max: 1
    ///
    ///Control signal to Baz ECU to change the HV battery pack contactor state.
    pub fn set_hv_battery_contactor_control(
        &mut self,
        value: bool,
    ) -> Result<(), CanError> {
        self.data.view_bits_mut::<Lsb0>()[8usize..9usize].store_le(u8::from(value));
        Ok(())
    }
    ///Set value of FooSubContactorsControl
    ///- Min: 0
    ///- Max: 1
    ///
    ///Control signal to Foo ECU to change contactor state.
    pub fn set_foo_sub_contactors_control(
        &mut self,
        value: bool,
    ) -> Result<(), CanError> {
        self.data.view_bits_mut::<Lsb0>()[9usize..10usize].store_le(u8::from(value));
        Ok(())
    }
    ///Set value of bcDischargeControl
    ///- Min: 0
    ///- Max: 1
    ///
    ///Control signal to Baz ECU to discharge the HV bus.
    pub fn set_bc_discharge_control(&mut self, value: bool) -> Result<(), CanError> {
        self.data.view_bits_mut::<Lsb0>()[11usize..12usize].store_le(u8::from(value));
        Ok(())
    }
    ///Set value of hvOffControl
    ///- Min: 0
    ///- Max: 1
    ///
    ///Control signal to indicate TTC ECUs to start HV shutdown process.
    pub fn set_hv_off_control(&mut self, value: bool) -> Result<(), CanError> {
        self.data.view_bits_mut::<Lsb0>()[13usize..14usize].store_le(u8::from(value));
        Ok(())
    }
    ///Set value of payloadHvStatus
    ///- Min: 0
    ///- Max: 1
    ///
    ///Status signal to BCU to indicate current state of the payload HV.
    pub fn set_payload_hv_status(&mut self, value: bool) -> Result<(), CanError> {
        self.data.view_bits_mut::<Lsb0>()[12usize..13usize].store_le(u8::from(value));
        Ok(())
    }
    ///Set value of vehicleMode
    ///- Min: 0
    ///- Max: 0
    pub fn set_vehicle_mode(&mut self, value: VehicleModeEnum) -> Result<(), CanError> {
        self.data.view_bits_mut::<Lsb0>()[0usize..4usize].store_le(u8::from(value));
        Ok(())
    }
    ///Set value of lvOnStatus
    ///- Min: 0
    ///- Max: 1
    ///
    ///Indicates whether LV bus has voltage and is the main contactor closed
    pub fn set_lv_on_status(&mut self, value: bool) -> Result<(), CanError> {
        self.data.view_bits_mut::<Lsb0>()[10usize..11usize].store_le(u8::from(value));
        Ok(())
    }
}
impl GeneratedCanMessage<{ Self::LEN }> for BarControlAndStatusMsg {
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
///FooMaintenanceControl
///- ID: Standard 1840 (0x730)
///- Size: 8 bytes
///- Transmitter: Bcu
///
///Foo pump and brake control in service mode.
#[derive(Debug, Clone)]
pub struct FooMaintenanceControlMsg {
    data: [u8; 8usize],
}
impl FooMaintenanceControlMsg {
    pub const ID: Id = Id::Standard(unsafe { StandardId::new_unchecked(1840u16) });
    pub const LEN: usize = 8usize;
    pub fn new(
        brake_pump_control: bool,
        service_brake_axle1_prop_control: u8,
        service_brake_axle2_prop_control: u8,
        service_brake_axle3_prop_control: u8,
        service_brake_axle4_prop_control: u8,
        park_tank_outflow_valve_control: bool,
        axle1_tank_outflow_valve_control: bool,
        axle2_tank_outflow_valve_control: bool,
        axle3_tank_outflow_valve_control: bool,
        axle4_tank_outflow_valve_control: bool,
        left_steering_ignition: LeftSteeringIgnitionEnum,
        right_steering_ignition: RightSteeringIgnitionEnum,
        eaxle_ignition: EaxleIgnitionEnum,
        pneumatics_enabled: bool,
    ) -> Result<Self, CanError> {
        let mut msg = Self { data: [0u8; Self::LEN] };
        msg.set_brake_pump_control(brake_pump_control)?;
        msg.set_service_brake_axle1_prop_control(service_brake_axle1_prop_control)?;
        msg.set_service_brake_axle2_prop_control(service_brake_axle2_prop_control)?;
        msg.set_service_brake_axle3_prop_control(service_brake_axle3_prop_control)?;
        msg.set_service_brake_axle4_prop_control(service_brake_axle4_prop_control)?;
        msg.set_park_tank_outflow_valve_control(park_tank_outflow_valve_control)?;
        msg.set_axle1_tank_outflow_valve_control(axle1_tank_outflow_valve_control)?;
        msg.set_axle2_tank_outflow_valve_control(axle2_tank_outflow_valve_control)?;
        msg.set_axle3_tank_outflow_valve_control(axle3_tank_outflow_valve_control)?;
        msg.set_axle4_tank_outflow_valve_control(axle4_tank_outflow_valve_control)?;
        msg.set_left_steering_ignition(left_steering_ignition)?;
        msg.set_right_steering_ignition(right_steering_ignition)?;
        msg.set_eaxle_ignition(eaxle_ignition)?;
        msg.set_pneumatics_enabled(pneumatics_enabled)?;
        Ok(msg)
    }
    ///brakePumpControl
    ///- Min: 0
    ///- Max: 1
    ///- Unit:
    ///- Receivers: VectorXXX
    ///- Start bit: 0
    ///- Size: 1 bits
    ///- Factor: 1
    ///- Offset: 0
    ///- Byte order: LittleEndian
    ///- Type: unsigned
    pub fn brake_pump_control(&self) -> bool {
        let raw_brake_pump_control = self
            .data
            .view_bits::<Lsb0>()[0usize..1usize]
            .load_le::<u8>();
        raw_brake_pump_control == 1
    }
    ///serviceBrakeAxle1PropControl
    ///- Min: 0
    ///- Max: 100
    ///- Unit: %
    ///- Receivers: VectorXXX
    ///- Start bit: 32
    ///- Size: 8 bits
    ///- Factor: 1
    ///- Offset: 0
    ///- Byte order: LittleEndian
    ///- Type: unsigned
    pub fn service_brake_axle1_prop_control(&self) -> u8 {
        let raw_service_brake_axle1_prop_control = self
            .data
            .view_bits::<Lsb0>()[32usize..40usize]
            .load_le::<u8>();
        (raw_service_brake_axle1_prop_control) * (1u8) + (0u8)
    }
    ///serviceBrakeAxle2PropControl
    ///- Min: 0
    ///- Max: 100
    ///- Unit: %
    ///- Receivers: VectorXXX
    ///- Start bit: 40
    ///- Size: 8 bits
    ///- Factor: 1
    ///- Offset: 0
    ///- Byte order: LittleEndian
    ///- Type: unsigned
    pub fn service_brake_axle2_prop_control(&self) -> u8 {
        let raw_service_brake_axle2_prop_control = self
            .data
            .view_bits::<Lsb0>()[40usize..48usize]
            .load_le::<u8>();
        (raw_service_brake_axle2_prop_control) * (1u8) + (0u8)
    }
    ///serviceBrakeAxle3PropControl
    ///- Min: 0
    ///- Max: 100
    ///- Unit: %
    ///- Receivers: VectorXXX
    ///- Start bit: 48
    ///- Size: 8 bits
    ///- Factor: 1
    ///- Offset: 0
    ///- Byte order: LittleEndian
    ///- Type: unsigned
    pub fn service_brake_axle3_prop_control(&self) -> u8 {
        let raw_service_brake_axle3_prop_control = self
            .data
            .view_bits::<Lsb0>()[48usize..56usize]
            .load_le::<u8>();
        (raw_service_brake_axle3_prop_control) * (1u8) + (0u8)
    }
    ///serviceBrakeAxle4PropControl
    ///- Min: 0
    ///- Max: 100
    ///- Unit: %
    ///- Receivers: VectorXXX
    ///- Start bit: 56
    ///- Size: 8 bits
    ///- Factor: 1
    ///- Offset: 0
    ///- Byte order: LittleEndian
    ///- Type: unsigned
    pub fn service_brake_axle4_prop_control(&self) -> u8 {
        let raw_service_brake_axle4_prop_control = self
            .data
            .view_bits::<Lsb0>()[56usize..64usize]
            .load_le::<u8>();
        (raw_service_brake_axle4_prop_control) * (1u8) + (0u8)
    }
    ///parkTankOutflowValveControl
    ///- Min: 0
    ///- Max: 1
    ///- Unit:
    ///- Receivers: VectorXXX
    ///- Start bit: 1
    ///- Size: 1 bits
    ///- Factor: 1
    ///- Offset: 0
    ///- Byte order: LittleEndian
    ///- Type: unsigned
    pub fn park_tank_outflow_valve_control(&self) -> bool {
        let raw_park_tank_outflow_valve_control = self
            .data
            .view_bits::<Lsb0>()[1usize..2usize]
            .load_le::<u8>();
        raw_park_tank_outflow_valve_control == 1
    }
    ///axle1TankOutflowValveControl
    ///- Min: 0
    ///- Max: 1
    ///- Unit:
    ///- Receivers: VectorXXX
    ///- Start bit: 2
    ///- Size: 1 bits
    ///- Factor: 1
    ///- Offset: 0
    ///- Byte order: LittleEndian
    ///- Type: unsigned
    pub fn axle1_tank_outflow_valve_control(&self) -> bool {
        let raw_axle1_tank_outflow_valve_control = self
            .data
            .view_bits::<Lsb0>()[2usize..3usize]
            .load_le::<u8>();
        raw_axle1_tank_outflow_valve_control == 1
    }
    ///axle2TankOutflowValveControl
    ///- Min: 0
    ///- Max: 1
    ///- Unit:
    ///- Receivers: VectorXXX
    ///- Start bit: 3
    ///- Size: 1 bits
    ///- Factor: 1
    ///- Offset: 0
    ///- Byte order: LittleEndian
    ///- Type: unsigned
    pub fn axle2_tank_outflow_valve_control(&self) -> bool {
        let raw_axle2_tank_outflow_valve_control = self
            .data
            .view_bits::<Lsb0>()[3usize..4usize]
            .load_le::<u8>();
        raw_axle2_tank_outflow_valve_control == 1
    }
    ///axle3TankOutflowValveControl
    ///- Min: 0
    ///- Max: 1
    ///- Unit:
    ///- Receivers: VectorXXX
    ///- Start bit: 4
    ///- Size: 1 bits
    ///- Factor: 1
    ///- Offset: 0
    ///- Byte order: LittleEndian
    ///- Type: unsigned
    pub fn axle3_tank_outflow_valve_control(&self) -> bool {
        let raw_axle3_tank_outflow_valve_control = self
            .data
            .view_bits::<Lsb0>()[4usize..5usize]
            .load_le::<u8>();
        raw_axle3_tank_outflow_valve_control == 1
    }
    ///axle4TankOutflowValveControl
    ///- Min: 0
    ///- Max: 1
    ///- Unit:
    ///- Receivers: VectorXXX
    ///- Start bit: 5
    ///- Size: 1 bits
    ///- Factor: 1
    ///- Offset: 0
    ///- Byte order: LittleEndian
    ///- Type: unsigned
    pub fn axle4_tank_outflow_valve_control(&self) -> bool {
        let raw_axle4_tank_outflow_valve_control = self
            .data
            .view_bits::<Lsb0>()[5usize..6usize]
            .load_le::<u8>();
        raw_axle4_tank_outflow_valve_control == 1
    }
    ///leftSteeringIgnition
    ///- Min: 0
    ///- Max: 0
    ///- Unit:
    ///- Receivers: VectorXXX
    ///- Start bit: 8
    ///- Size: 2 bits
    ///- Factor: 1
    ///- Offset: 0
    ///- Byte order: LittleEndian
    ///- Type: unsigned
    pub fn left_steering_ignition(&self) -> LeftSteeringIgnitionEnum {
        let raw_left_steering_ignition = self
            .data
            .view_bits::<Lsb0>()[8usize..10usize]
            .load_le::<u8>();
        LeftSteeringIgnitionEnum::from(raw_left_steering_ignition as u8)
    }
    ///rightSteeringIgnition
    ///- Min: 0
    ///- Max: 0
    ///- Unit:
    ///- Receivers: VectorXXX
    ///- Start bit: 10
    ///- Size: 2 bits
    ///- Factor: 1
    ///- Offset: 0
    ///- Byte order: LittleEndian
    ///- Type: unsigned
    pub fn right_steering_ignition(&self) -> RightSteeringIgnitionEnum {
        let raw_right_steering_ignition = self
            .data
            .view_bits::<Lsb0>()[10usize..12usize]
            .load_le::<u8>();
        RightSteeringIgnitionEnum::from(raw_right_steering_ignition as u8)
    }
    ///eaxleIgnition
    ///- Min: 0
    ///- Max: 2
    ///- Unit:
    ///- Receivers: VectorXXX
    ///- Start bit: 12
    ///- Size: 2 bits
    ///- Factor: 1
    ///- Offset: 0
    ///- Byte order: LittleEndian
    ///- Type: unsigned
    pub fn eaxle_ignition(&self) -> EaxleIgnitionEnum {
        let raw_eaxle_ignition = self
            .data
            .view_bits::<Lsb0>()[12usize..14usize]
            .load_le::<u8>();
        EaxleIgnitionEnum::from(raw_eaxle_ignition as u8)
    }
    ///pneumaticsEnabled
    ///- Min: 0
    ///- Max: 0
    ///- Unit:
    ///- Receivers: VectorXXX
    ///- Start bit: 6
    ///- Size: 1 bits
    ///- Factor: 1
    ///- Offset: 0
    ///- Byte order: LittleEndian
    ///- Type: unsigned
    ///
    ///Flag to allow operator to enable/disable the pneumatic system in Maintenance mode.
    pub fn pneumatics_enabled(&self) -> bool {
        let raw_pneumatics_enabled = self
            .data
            .view_bits::<Lsb0>()[6usize..7usize]
            .load_le::<u8>();
        raw_pneumatics_enabled == 1
    }
    ///Set value of brakePumpControl
    ///- Min: 0
    ///- Max: 1
    pub fn set_brake_pump_control(&mut self, value: bool) -> Result<(), CanError> {
        self.data.view_bits_mut::<Lsb0>()[0usize..1usize].store_le(u8::from(value));
        Ok(())
    }
    ///Set value of serviceBrakeAxle1PropControl
    ///- Min: 0
    ///- Max: 100
    pub fn set_service_brake_axle1_prop_control(
        &mut self,
        value: u8,
    ) -> Result<(), CanError> {
        if value > 100u8 {
            return Err(CanError::ValueOutOfRange);
        }
        self.data
            .view_bits_mut::<Lsb0>()[32usize..40usize]
            .store_le((value - (0u8)) / (1u8));
        Ok(())
    }
    ///Set value of serviceBrakeAxle2PropControl
    ///- Min: 0
    ///- Max: 100
    pub fn set_service_brake_axle2_prop_control(
        &mut self,
        value: u8,
    ) -> Result<(), CanError> {
        if value > 100u8 {
            return Err(CanError::ValueOutOfRange);
        }
        self.data
            .view_bits_mut::<Lsb0>()[40usize..48usize]
            .store_le((value - (0u8)) / (1u8));
        Ok(())
    }
    ///Set value of serviceBrakeAxle3PropControl
    ///- Min: 0
    ///- Max: 100
    pub fn set_service_brake_axle3_prop_control(
        &mut self,
        value: u8,
    ) -> Result<(), CanError> {
        if value > 100u8 {
            return Err(CanError::ValueOutOfRange);
        }
        self.data
            .view_bits_mut::<Lsb0>()[48usize..56usize]
            .store_le((value - (0u8)) / (1u8));
        Ok(())
    }
    ///Set value of serviceBrakeAxle4PropControl
    ///- Min: 0
    ///- Max: 100
    pub fn set_service_brake_axle4_prop_control(
        &mut self,
        value: u8,
    ) -> Result<(), CanError> {
        if value > 100u8 {
            return Err(CanError::ValueOutOfRange);
        }
        self.data
            .view_bits_mut::<Lsb0>()[56usize..64usize]
            .store_le((value - (0u8)) / (1u8));
        Ok(())
    }
    ///Set value of parkTankOutflowValveControl
    ///- Min: 0
    ///- Max: 1
    pub fn set_park_tank_outflow_valve_control(
        &mut self,
        value: bool,
    ) -> Result<(), CanError> {
        self.data.view_bits_mut::<Lsb0>()[1usize..2usize].store_le(u8::from(value));
        Ok(())
    }
    ///Set value of axle1TankOutflowValveControl
    ///- Min: 0
    ///- Max: 1
    pub fn set_axle1_tank_outflow_valve_control(
        &mut self,
        value: bool,
    ) -> Result<(), CanError> {
        self.data.view_bits_mut::<Lsb0>()[2usize..3usize].store_le(u8::from(value));
        Ok(())
    }
    ///Set value of axle2TankOutflowValveControl
    ///- Min: 0
    ///- Max: 1
    pub fn set_axle2_tank_outflow_valve_control(
        &mut self,
        value: bool,
    ) -> Result<(), CanError> {
        self.data.view_bits_mut::<Lsb0>()[3usize..4usize].store_le(u8::from(value));
        Ok(())
    }
    ///Set value of axle3TankOutflowValveControl
    ///- Min: 0
    ///- Max: 1
    pub fn set_axle3_tank_outflow_valve_control(
        &mut self,
        value: bool,
    ) -> Result<(), CanError> {
        self.data.view_bits_mut::<Lsb0>()[4usize..5usize].store_le(u8::from(value));
        Ok(())
    }
    ///Set value of axle4TankOutflowValveControl
    ///- Min: 0
    ///- Max: 1
    pub fn set_axle4_tank_outflow_valve_control(
        &mut self,
        value: bool,
    ) -> Result<(), CanError> {
        self.data.view_bits_mut::<Lsb0>()[5usize..6usize].store_le(u8::from(value));
        Ok(())
    }
    ///Set value of leftSteeringIgnition
    ///- Min: 0
    ///- Max: 0
    pub fn set_left_steering_ignition(
        &mut self,
        value: LeftSteeringIgnitionEnum,
    ) -> Result<(), CanError> {
        self.data.view_bits_mut::<Lsb0>()[8usize..10usize].store_le(u8::from(value));
        Ok(())
    }
    ///Set value of rightSteeringIgnition
    ///- Min: 0
    ///- Max: 0
    pub fn set_right_steering_ignition(
        &mut self,
        value: RightSteeringIgnitionEnum,
    ) -> Result<(), CanError> {
        self.data.view_bits_mut::<Lsb0>()[10usize..12usize].store_le(u8::from(value));
        Ok(())
    }
    ///Set value of eaxleIgnition
    ///- Min: 0
    ///- Max: 2
    pub fn set_eaxle_ignition(
        &mut self,
        value: EaxleIgnitionEnum,
    ) -> Result<(), CanError> {
        self.data.view_bits_mut::<Lsb0>()[12usize..14usize].store_le(u8::from(value));
        Ok(())
    }
    ///Set value of pneumaticsEnabled
    ///- Min: 0
    ///- Max: 0
    ///
    ///Flag to allow operator to enable/disable the pneumatic system in Maintenance mode.
    pub fn set_pneumatics_enabled(&mut self, value: bool) -> Result<(), CanError> {
        self.data.view_bits_mut::<Lsb0>()[6usize..7usize].store_le(u8::from(value));
        Ok(())
    }
}
impl GeneratedCanMessage<{ Self::LEN }> for FooMaintenanceControlMsg {
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
///ZeroAll
///- ID: Standard 85 (0x55)
///- Size: 8 bytes
///- Transmitter: Bcu
///
///Initiate zero all to all of the ECUs, the ECUs should flush firmware running on the device.
#[derive(Debug, Clone)]
pub struct ZeroAllMsg {
    data: [u8; 8usize],
}
impl ZeroAllMsg {
    pub const ID: Id = Id::Standard(unsafe { StandardId::new_unchecked(85u16) });
    pub const LEN: usize = 8usize;
    pub fn new() -> Result<Self, CanError> {
        let mut msg = Self { data: [0u8; Self::LEN] };
        Ok(msg)
    }
}
impl GeneratedCanMessage<{ Self::LEN }> for ZeroAllMsg {
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
///quuxTimeSyncRequest
///- ID: Standard 256 (0x100)
///- Size: 4 bytes
///- Transmitter: Bcu
#[derive(Debug, Clone)]
pub struct QuuxTimeSyncRequestMsg {
    data: [u8; 4usize],
}
impl QuuxTimeSyncRequestMsg {
    pub const ID: Id = Id::Standard(unsafe { StandardId::new_unchecked(256u16) });
    pub const LEN: usize = 4usize;
    pub fn new(timestamp: u32) -> Result<Self, CanError> {
        let mut msg = Self { data: [0u8; Self::LEN] };
        msg.set_timestamp(timestamp)?;
        Ok(msg)
    }
    ///timestamp
    ///- Min: 0
    ///- Max: 4294967295
    ///- Unit: ms
    ///- Receivers: quuxEcu
    ///- Start bit: 0
    ///- Size: 32 bits
    ///- Factor: 1
    ///- Offset: 0
    ///- Byte order: LittleEndian
    ///- Type: unsigned
    ///
    ///BCU timestamp of relative time
    pub fn timestamp(&self) -> u32 {
        let raw_timestamp = self
            .data
            .view_bits::<Lsb0>()[0usize..32usize]
            .load_le::<u32>();
        (raw_timestamp) * (1u32) + (0u32)
    }
    ///Set value of timestamp
    ///- Min: 0
    ///- Max: 4294967295
    ///
    ///BCU timestamp of relative time
    pub fn set_timestamp(&mut self, value: u32) -> Result<(), CanError> {
        self.data
            .view_bits_mut::<Lsb0>()[0usize..32usize]
            .store_le((value - (0u32)) / (1u32));
        Ok(())
    }
}
impl GeneratedCanMessage<{ Self::LEN }> for QuuxTimeSyncRequestMsg {
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
///FooTimeSyncRequest
///- ID: Standard 257 (0x101)
///- Size: 4 bytes
///- Transmitter: Bcu
#[derive(Debug, Clone)]
pub struct FooTimeSyncRequestMsg {
    data: [u8; 4usize],
}
impl FooTimeSyncRequestMsg {
    pub const ID: Id = Id::Standard(unsafe { StandardId::new_unchecked(257u16) });
    pub const LEN: usize = 4usize;
    pub fn new(timestamp: u32) -> Result<Self, CanError> {
        let mut msg = Self { data: [0u8; Self::LEN] };
        msg.set_timestamp(timestamp)?;
        Ok(msg)
    }
    ///timestamp
    ///- Min: 0
    ///- Max: 4294967295
    ///- Unit: ms
    ///- Receivers: FooEcu
    ///- Start bit: 0
    ///- Size: 32 bits
    ///- Factor: 1
    ///- Offset: 0
    ///- Byte order: LittleEndian
    ///- Type: unsigned
    ///
    ///BCU timestamp of relative time
    pub fn timestamp(&self) -> u32 {
        let raw_timestamp = self
            .data
            .view_bits::<Lsb0>()[0usize..32usize]
            .load_le::<u32>();
        (raw_timestamp) * (1u32) + (0u32)
    }
    ///Set value of timestamp
    ///- Min: 0
    ///- Max: 4294967295
    ///
    ///BCU timestamp of relative time
    pub fn set_timestamp(&mut self, value: u32) -> Result<(), CanError> {
        self.data
            .view_bits_mut::<Lsb0>()[0usize..32usize]
            .store_le((value - (0u32)) / (1u32));
        Ok(())
    }
}
impl GeneratedCanMessage<{ Self::LEN }> for FooTimeSyncRequestMsg {
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
///BarTimeSyncRequest
///- ID: Standard 258 (0x102)
///- Size: 4 bytes
///- Transmitter: Bcu
#[derive(Debug, Clone)]
pub struct BarTimeSyncRequestMsg {
    data: [u8; 4usize],
}
impl BarTimeSyncRequestMsg {
    pub const ID: Id = Id::Standard(unsafe { StandardId::new_unchecked(258u16) });
    pub const LEN: usize = 4usize;
    pub fn new(timestamp: u32) -> Result<Self, CanError> {
        let mut msg = Self { data: [0u8; Self::LEN] };
        msg.set_timestamp(timestamp)?;
        Ok(msg)
    }
    ///timestamp
    ///- Min: 0
    ///- Max: 4294967295
    ///- Unit: ms
    ///- Receivers: BarEcu
    ///- Start bit: 0
    ///- Size: 32 bits
    ///- Factor: 1
    ///- Offset: 0
    ///- Byte order: LittleEndian
    ///- Type: unsigned
    ///
    ///BCU timestamp in relative time
    pub fn timestamp(&self) -> u32 {
        let raw_timestamp = self
            .data
            .view_bits::<Lsb0>()[0usize..32usize]
            .load_le::<u32>();
        (raw_timestamp) * (1u32) + (0u32)
    }
    ///Set value of timestamp
    ///- Min: 0
    ///- Max: 4294967295
    ///
    ///BCU timestamp in relative time
    pub fn set_timestamp(&mut self, value: u32) -> Result<(), CanError> {
        self.data
            .view_bits_mut::<Lsb0>()[0usize..32usize]
            .store_le((value - (0u32)) / (1u32));
        Ok(())
    }
}
impl GeneratedCanMessage<{ Self::LEN }> for BarTimeSyncRequestMsg {
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
///BazTimeSyncRequest
///- ID: Standard 259 (0x103)
///- Size: 4 bytes
///- Transmitter: Bcu
#[derive(Debug, Clone)]
pub struct BazTimeSyncRequestMsg {
    data: [u8; 4usize],
}
impl BazTimeSyncRequestMsg {
    pub const ID: Id = Id::Standard(unsafe { StandardId::new_unchecked(259u16) });
    pub const LEN: usize = 4usize;
    pub fn new(timestamp: u32) -> Result<Self, CanError> {
        let mut msg = Self { data: [0u8; Self::LEN] };
        msg.set_timestamp(timestamp)?;
        Ok(msg)
    }
    ///timestamp
    ///- Min: 0
    ///- Max: 4294967295
    ///- Unit: ms
    ///- Receivers: PowerManagementEcu
    ///- Start bit: 0
    ///- Size: 32 bits
    ///- Factor: 1
    ///- Offset: 0
    ///- Byte order: LittleEndian
    ///- Type: unsigned
    ///
    ///BCU timestamp in relative time
    pub fn timestamp(&self) -> u32 {
        let raw_timestamp = self
            .data
            .view_bits::<Lsb0>()[0usize..32usize]
            .load_le::<u32>();
        (raw_timestamp) * (1u32) + (0u32)
    }
    ///Set value of timestamp
    ///- Min: 0
    ///- Max: 4294967295
    ///
    ///BCU timestamp in relative time
    pub fn set_timestamp(&mut self, value: u32) -> Result<(), CanError> {
        self.data
            .view_bits_mut::<Lsb0>()[0usize..32usize]
            .store_le((value - (0u32)) / (1u32));
        Ok(())
    }
}
impl GeneratedCanMessage<{ Self::LEN }> for BazTimeSyncRequestMsg {
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
///quuxTimeSyncResponse
///- ID: Standard 260 (0x104)
///- Size: 8 bytes
///- Transmitter: quuxEcu
#[derive(Debug, Clone)]
pub struct QuuxTimeSyncResponseMsg {
    data: [u8; 8usize],
}
impl QuuxTimeSyncResponseMsg {
    pub const ID: Id = Id::Standard(unsafe { StandardId::new_unchecked(260u16) });
    pub const LEN: usize = 8usize;
    pub fn new(time_diff: i32, timestamp: u32) -> Result<Self, CanError> {
        let mut msg = Self { data: [0u8; Self::LEN] };
        msg.set_time_diff(time_diff)?;
        msg.set_timestamp(timestamp)?;
        Ok(msg)
    }
    ///timeDiff
    ///- Min: -2147483648
    ///- Max: 2147483647
    ///- Unit: ms
    ///- Receivers: Bcu
    ///- Start bit: 0
    ///- Size: 32 bits
    ///- Factor: 1
    ///- Offset: 0
    ///- Byte order: LittleEndian
    ///- Type: signed
    ///
    ///Delay + offset
    pub fn time_diff(&self) -> i32 {
        let raw_time_diff = self
            .data
            .view_bits::<Lsb0>()[0usize..32usize]
            .load_le::<u32>();
        (raw_time_diff as i32) * (1i32) + (0i32)
    }
    ///timestamp
    ///- Min: 0
    ///- Max: 4294967295
    ///- Unit: ms
    ///- Receivers: Bcu
    ///- Start bit: 32
    ///- Size: 32 bits
    ///- Factor: 1
    ///- Offset: 0
    ///- Byte order: LittleEndian
    ///- Type: unsigned
    ///
    ///quux ECU timestamp
    pub fn timestamp(&self) -> u32 {
        let raw_timestamp = self
            .data
            .view_bits::<Lsb0>()[32usize..64usize]
            .load_le::<u32>();
        (raw_timestamp) * (1u32) + (0u32)
    }
    ///Set value of timeDiff
    ///- Min: -2147483648
    ///- Max: 2147483647
    ///
    ///Delay + offset
    pub fn set_time_diff(&mut self, value: i32) -> Result<(), CanError> {
        self.data
            .view_bits_mut::<Lsb0>()[0usize..32usize]
            .store_le((value - (0i32)) / (1i32));
        Ok(())
    }
    ///Set value of timestamp
    ///- Min: 0
    ///- Max: 4294967295
    ///
    ///quux ECU timestamp
    pub fn set_timestamp(&mut self, value: u32) -> Result<(), CanError> {
        self.data
            .view_bits_mut::<Lsb0>()[32usize..64usize]
            .store_le((value - (0u32)) / (1u32));
        Ok(())
    }
}
impl GeneratedCanMessage<{ Self::LEN }> for QuuxTimeSyncResponseMsg {
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
///FooTimeSyncResponse
///- ID: Standard 261 (0x105)
///- Size: 8 bytes
///- Transmitter: FooEcu
#[derive(Debug, Clone)]
pub struct FooTimeSyncResponseMsg {
    data: [u8; 8usize],
}
impl FooTimeSyncResponseMsg {
    pub const ID: Id = Id::Standard(unsafe { StandardId::new_unchecked(261u16) });
    pub const LEN: usize = 8usize;
    pub fn new(time_diff: i32, timestamp: u32) -> Result<Self, CanError> {
        let mut msg = Self { data: [0u8; Self::LEN] };
        msg.set_time_diff(time_diff)?;
        msg.set_timestamp(timestamp)?;
        Ok(msg)
    }
    ///timeDiff
    ///- Min: -2147483648
    ///- Max: 2147483647
    ///- Unit: ms
    ///- Receivers: Bcu
    ///- Start bit: 0
    ///- Size: 32 bits
    ///- Factor: 1
    ///- Offset: 0
    ///- Byte order: LittleEndian
    ///- Type: signed
    ///
    ///Delay + offset
    pub fn time_diff(&self) -> i32 {
        let raw_time_diff = self
            .data
            .view_bits::<Lsb0>()[0usize..32usize]
            .load_le::<u32>();
        (raw_time_diff as i32) * (1i32) + (0i32)
    }
    ///timestamp
    ///- Min: 0
    ///- Max: 4294967295
    ///- Unit: ms
    ///- Receivers: Bcu
    ///- Start bit: 32
    ///- Size: 32 bits
    ///- Factor: 1
    ///- Offset: 0
    ///- Byte order: LittleEndian
    ///- Type: unsigned
    ///
    ///Foo ECU timestamp
    pub fn timestamp(&self) -> u32 {
        let raw_timestamp = self
            .data
            .view_bits::<Lsb0>()[32usize..64usize]
            .load_le::<u32>();
        (raw_timestamp) * (1u32) + (0u32)
    }
    ///Set value of timeDiff
    ///- Min: -2147483648
    ///- Max: 2147483647
    ///
    ///Delay + offset
    pub fn set_time_diff(&mut self, value: i32) -> Result<(), CanError> {
        self.data
            .view_bits_mut::<Lsb0>()[0usize..32usize]
            .store_le((value - (0i32)) / (1i32));
        Ok(())
    }
    ///Set value of timestamp
    ///- Min: 0
    ///- Max: 4294967295
    ///
    ///Foo ECU timestamp
    pub fn set_timestamp(&mut self, value: u32) -> Result<(), CanError> {
        self.data
            .view_bits_mut::<Lsb0>()[32usize..64usize]
            .store_le((value - (0u32)) / (1u32));
        Ok(())
    }
}
impl GeneratedCanMessage<{ Self::LEN }> for FooTimeSyncResponseMsg {
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
///BarTimeSyncResponse
///- ID: Standard 262 (0x106)
///- Size: 8 bytes
///- Transmitter: BarEcu
#[derive(Debug, Clone)]
pub struct BarTimeSyncResponseMsg {
    data: [u8; 8usize],
}
impl BarTimeSyncResponseMsg {
    pub const ID: Id = Id::Standard(unsafe { StandardId::new_unchecked(262u16) });
    pub const LEN: usize = 8usize;
    pub fn new(time_diff: i32, timestamp: u32) -> Result<Self, CanError> {
        let mut msg = Self { data: [0u8; Self::LEN] };
        msg.set_time_diff(time_diff)?;
        msg.set_timestamp(timestamp)?;
        Ok(msg)
    }
    ///timeDiff
    ///- Min: -2147483648
    ///- Max: 2147483647
    ///- Unit: ms
    ///- Receivers: Bcu
    ///- Start bit: 0
    ///- Size: 32 bits
    ///- Factor: 1
    ///- Offset: 0
    ///- Byte order: LittleEndian
    ///- Type: signed
    ///
    ///Delay + offset
    pub fn time_diff(&self) -> i32 {
        let raw_time_diff = self
            .data
            .view_bits::<Lsb0>()[0usize..32usize]
            .load_le::<u32>();
        (raw_time_diff as i32) * (1i32) + (0i32)
    }
    ///timestamp
    ///- Min: 0
    ///- Max: 4294967295
    ///- Unit: ms
    ///- Receivers: Bcu
    ///- Start bit: 32
    ///- Size: 32 bits
    ///- Factor: 1
    ///- Offset: 0
    ///- Byte order: LittleEndian
    ///- Type: unsigned
    ///
    ///Bar ECU timestamp
    pub fn timestamp(&self) -> u32 {
        let raw_timestamp = self
            .data
            .view_bits::<Lsb0>()[32usize..64usize]
            .load_le::<u32>();
        (raw_timestamp) * (1u32) + (0u32)
    }
    ///Set value of timeDiff
    ///- Min: -2147483648
    ///- Max: 2147483647
    ///
    ///Delay + offset
    pub fn set_time_diff(&mut self, value: i32) -> Result<(), CanError> {
        self.data
            .view_bits_mut::<Lsb0>()[0usize..32usize]
            .store_le((value - (0i32)) / (1i32));
        Ok(())
    }
    ///Set value of timestamp
    ///- Min: 0
    ///- Max: 4294967295
    ///
    ///Bar ECU timestamp
    pub fn set_timestamp(&mut self, value: u32) -> Result<(), CanError> {
        self.data
            .view_bits_mut::<Lsb0>()[32usize..64usize]
            .store_le((value - (0u32)) / (1u32));
        Ok(())
    }
}
impl GeneratedCanMessage<{ Self::LEN }> for BarTimeSyncResponseMsg {
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
///BazTimeSyncResponse
///- ID: Standard 263 (0x107)
///- Size: 8 bytes
///- Transmitter: PowerManagementEcu
#[derive(Debug, Clone)]
pub struct BazTimeSyncResponseMsg {
    data: [u8; 8usize],
}
impl BazTimeSyncResponseMsg {
    pub const ID: Id = Id::Standard(unsafe { StandardId::new_unchecked(263u16) });
    pub const LEN: usize = 8usize;
    pub fn new(time_diff: i32, timestamp: u32) -> Result<Self, CanError> {
        let mut msg = Self { data: [0u8; Self::LEN] };
        msg.set_time_diff(time_diff)?;
        msg.set_timestamp(timestamp)?;
        Ok(msg)
    }
    ///timeDiff
    ///- Min: -2147483648
    ///- Max: 2147483647
    ///- Unit: ms
    ///- Receivers: Bcu
    ///- Start bit: 0
    ///- Size: 32 bits
    ///- Factor: 1
    ///- Offset: 0
    ///- Byte order: LittleEndian
    ///- Type: signed
    ///
    ///Delay + offset
    pub fn time_diff(&self) -> i32 {
        let raw_time_diff = self
            .data
            .view_bits::<Lsb0>()[0usize..32usize]
            .load_le::<u32>();
        (raw_time_diff as i32) * (1i32) + (0i32)
    }
    ///timestamp
    ///- Min: 0
    ///- Max: 4294967295
    ///- Unit: ms
    ///- Receivers: Bcu
    ///- Start bit: 32
    ///- Size: 32 bits
    ///- Factor: 1
    ///- Offset: 0
    ///- Byte order: LittleEndian
    ///- Type: unsigned
    ///
    ///Baz ECU timestamp
    pub fn timestamp(&self) -> u32 {
        let raw_timestamp = self
            .data
            .view_bits::<Lsb0>()[32usize..64usize]
            .load_le::<u32>();
        (raw_timestamp) * (1u32) + (0u32)
    }
    ///Set value of timeDiff
    ///- Min: -2147483648
    ///- Max: 2147483647
    ///
    ///Delay + offset
    pub fn set_time_diff(&mut self, value: i32) -> Result<(), CanError> {
        self.data
            .view_bits_mut::<Lsb0>()[0usize..32usize]
            .store_le((value - (0i32)) / (1i32));
        Ok(())
    }
    ///Set value of timestamp
    ///- Min: 0
    ///- Max: 4294967295
    ///
    ///Baz ECU timestamp
    pub fn set_timestamp(&mut self, value: u32) -> Result<(), CanError> {
        self.data
            .view_bits_mut::<Lsb0>()[32usize..64usize]
            .store_le((value - (0u32)) / (1u32));
        Ok(())
    }
}
impl GeneratedCanMessage<{ Self::LEN }> for BazTimeSyncResponseMsg {
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
///BazAck
///- ID: Standard 355 (0x163)
///- Size: 1 bytes
///- Transmitter: PowerManagementEcu
///
///Subsystem specific confirmation message after LV or HV shutdown request.
#[derive(Debug, Clone)]
pub struct BazAckMsg {
    data: [u8; 1usize],
}
impl BazAckMsg {
    pub const ID: Id = Id::Standard(unsafe { StandardId::new_unchecked(355u16) });
    pub const LEN: usize = 1usize;
    pub fn new(
        hv_safe_to_disable: bool,
        lv_safe_to_disable: bool,
    ) -> Result<Self, CanError> {
        let mut msg = Self { data: [0u8; Self::LEN] };
        msg.set_hv_safe_to_disable(hv_safe_to_disable)?;
        msg.set_lv_safe_to_disable(lv_safe_to_disable)?;
        Ok(msg)
    }
    ///hvSafeToDisable
    ///- Min: 0
    ///- Max: 1
    ///- Unit: -
    ///- Receivers: VectorXXX
    ///- Start bit: 0
    ///- Size: 1 bits
    ///- Factor: 1
    ///- Offset: 0
    ///- Byte order: LittleEndian
    ///- Type: unsigned
    pub fn hv_safe_to_disable(&self) -> bool {
        let raw_hv_safe_to_disable = self
            .data
            .view_bits::<Lsb0>()[0usize..1usize]
            .load_le::<u8>();
        raw_hv_safe_to_disable == 1
    }
    ///lvSafeToDisable
    ///- Min: 0
    ///- Max: 1
    ///- Unit: -
    ///- Receivers: VectorXXX
    ///- Start bit: 1
    ///- Size: 1 bits
    ///- Factor: 1
    ///- Offset: 0
    ///- Byte order: LittleEndian
    ///- Type: unsigned
    pub fn lv_safe_to_disable(&self) -> bool {
        let raw_lv_safe_to_disable = self
            .data
            .view_bits::<Lsb0>()[1usize..2usize]
            .load_le::<u8>();
        raw_lv_safe_to_disable == 1
    }
    ///Set value of hvSafeToDisable
    ///- Min: 0
    ///- Max: 1
    pub fn set_hv_safe_to_disable(&mut self, value: bool) -> Result<(), CanError> {
        self.data.view_bits_mut::<Lsb0>()[0usize..1usize].store_le(u8::from(value));
        Ok(())
    }
    ///Set value of lvSafeToDisable
    ///- Min: 0
    ///- Max: 1
    pub fn set_lv_safe_to_disable(&mut self, value: bool) -> Result<(), CanError> {
        self.data.view_bits_mut::<Lsb0>()[1usize..2usize].store_le(u8::from(value));
        Ok(())
    }
}
impl GeneratedCanMessage<{ Self::LEN }> for BazAckMsg {
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
///BcuAck
///- ID: Standard 112 (0x70)
///- Size: 1 bytes
///- Transmitter: Bcu
///
///Subsystem specific confirmation message after LV or HV shutdown request.
#[derive(Debug, Clone)]
pub struct BcuAckMsg {
    data: [u8; 1usize],
}
impl BcuAckMsg {
    pub const ID: Id = Id::Standard(unsafe { StandardId::new_unchecked(112u16) });
    pub const LEN: usize = 1usize;
    pub fn new(lv_safe_to_disable: bool) -> Result<Self, CanError> {
        let mut msg = Self { data: [0u8; Self::LEN] };
        msg.set_lv_safe_to_disable(lv_safe_to_disable)?;
        Ok(msg)
    }
    ///lvSafeToDisable
    ///- Min: 0
    ///- Max: 1
    ///- Unit: -
    ///- Receivers: VectorXXX
    ///- Start bit: 1
    ///- Size: 1 bits
    ///- Factor: 1
    ///- Offset: 0
    ///- Byte order: LittleEndian
    ///- Type: unsigned
    pub fn lv_safe_to_disable(&self) -> bool {
        let raw_lv_safe_to_disable = self
            .data
            .view_bits::<Lsb0>()[1usize..2usize]
            .load_le::<u8>();
        raw_lv_safe_to_disable == 1
    }
    ///Set value of lvSafeToDisable
    ///- Min: 0
    ///- Max: 1
    pub fn set_lv_safe_to_disable(&mut self, value: bool) -> Result<(), CanError> {
        self.data.view_bits_mut::<Lsb0>()[1usize..2usize].store_le(u8::from(value));
        Ok(())
    }
}
impl GeneratedCanMessage<{ Self::LEN }> for BcuAckMsg {
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
///FooAck
///- ID: Standard 338 (0x152)
///- Size: 1 bytes
///- Transmitter: FooEcu
///
///Subsystem specific confirmation message after LV or HV shutdown request.
#[derive(Debug, Clone)]
pub struct FooAckMsg {
    data: [u8; 1usize],
}
impl FooAckMsg {
    pub const ID: Id = Id::Standard(unsafe { StandardId::new_unchecked(338u16) });
    pub const LEN: usize = 1usize;
    pub fn new(
        hv_safe_to_disable: bool,
        lv_safe_to_disable: bool,
        request_foo_hv_restart: bool,
    ) -> Result<Self, CanError> {
        let mut msg = Self { data: [0u8; Self::LEN] };
        msg.set_hv_safe_to_disable(hv_safe_to_disable)?;
        msg.set_lv_safe_to_disable(lv_safe_to_disable)?;
        msg.set_request_foo_hv_restart(request_foo_hv_restart)?;
        Ok(msg)
    }
    ///hvSafeToDisable
    ///- Min: 0
    ///- Max: 1
    ///- Unit: -
    ///- Receivers: VectorXXX
    ///- Start bit: 0
    ///- Size: 1 bits
    ///- Factor: 1
    ///- Offset: 0
    ///- Byte order: LittleEndian
    ///- Type: unsigned
    pub fn hv_safe_to_disable(&self) -> bool {
        let raw_hv_safe_to_disable = self
            .data
            .view_bits::<Lsb0>()[0usize..1usize]
            .load_le::<u8>();
        raw_hv_safe_to_disable == 1
    }
    ///lvSafeToDisable
    ///- Min: 0
    ///- Max: 1
    ///- Unit: -
    ///- Receivers: VectorXXX
    ///- Start bit: 1
    ///- Size: 1 bits
    ///- Factor: 1
    ///- Offset: 0
    ///- Byte order: LittleEndian
    ///- Type: unsigned
    pub fn lv_safe_to_disable(&self) -> bool {
        let raw_lv_safe_to_disable = self
            .data
            .view_bits::<Lsb0>()[1usize..2usize]
            .load_le::<u8>();
        raw_lv_safe_to_disable == 1
    }
    ///requestFooHvRestart
    ///- Min: 0
    ///- Max: 1
    ///- Unit:
    ///- Receivers: VectorXXX
    ///- Start bit: 2
    ///- Size: 1 bits
    ///- Factor: 1
    ///- Offset: 0
    ///- Byte order: LittleEndian
    ///- Type: unsigned
    ///
    ///Foo ECU request to Bar ECU to start HV restart sequence for Foo.
    pub fn request_foo_hv_restart(&self) -> bool {
        let raw_request_foo_hv_restart = self
            .data
            .view_bits::<Lsb0>()[2usize..3usize]
            .load_le::<u8>();
        raw_request_foo_hv_restart == 1
    }
    ///Set value of hvSafeToDisable
    ///- Min: 0
    ///- Max: 1
    pub fn set_hv_safe_to_disable(&mut self, value: bool) -> Result<(), CanError> {
        self.data.view_bits_mut::<Lsb0>()[0usize..1usize].store_le(u8::from(value));
        Ok(())
    }
    ///Set value of lvSafeToDisable
    ///- Min: 0
    ///- Max: 1
    pub fn set_lv_safe_to_disable(&mut self, value: bool) -> Result<(), CanError> {
        self.data.view_bits_mut::<Lsb0>()[1usize..2usize].store_le(u8::from(value));
        Ok(())
    }
    ///Set value of requestFooHvRestart
    ///- Min: 0
    ///- Max: 1
    ///
    ///Foo ECU request to Bar ECU to start HV restart sequence for Foo.
    pub fn set_request_foo_hv_restart(&mut self, value: bool) -> Result<(), CanError> {
        self.data.view_bits_mut::<Lsb0>()[2usize..3usize].store_le(u8::from(value));
        Ok(())
    }
}
impl GeneratedCanMessage<{ Self::LEN }> for FooAckMsg {
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
///quuxAck
///- ID: Standard 321 (0x141)
///- Size: 1 bytes
///- Transmitter: quuxEcu
///
///Subsystem specific confirmation message after LV or HV shutdown request.
#[derive(Debug, Clone)]
pub struct QuuxAckMsg {
    data: [u8; 1usize],
}
impl QuuxAckMsg {
    pub const ID: Id = Id::Standard(unsafe { StandardId::new_unchecked(321u16) });
    pub const LEN: usize = 1usize;
    pub fn new(
        hv_safe_to_disable: bool,
        lv_safe_to_disable: bool,
    ) -> Result<Self, CanError> {
        let mut msg = Self { data: [0u8; Self::LEN] };
        msg.set_hv_safe_to_disable(hv_safe_to_disable)?;
        msg.set_lv_safe_to_disable(lv_safe_to_disable)?;
        Ok(msg)
    }
    ///hvSafeToDisable
    ///- Min: 0
    ///- Max: 1
    ///- Unit: -
    ///- Receivers: VectorXXX
    ///- Start bit: 0
    ///- Size: 1 bits
    ///- Factor: 1
    ///- Offset: 0
    ///- Byte order: LittleEndian
    ///- Type: unsigned
    pub fn hv_safe_to_disable(&self) -> bool {
        let raw_hv_safe_to_disable = self
            .data
            .view_bits::<Lsb0>()[0usize..1usize]
            .load_le::<u8>();
        raw_hv_safe_to_disable == 1
    }
    ///lvSafeToDisable
    ///- Min: 0
    ///- Max: 1
    ///- Unit: -
    ///- Receivers: VectorXXX
    ///- Start bit: 1
    ///- Size: 1 bits
    ///- Factor: 1
    ///- Offset: 0
    ///- Byte order: LittleEndian
    ///- Type: unsigned
    pub fn lv_safe_to_disable(&self) -> bool {
        let raw_lv_safe_to_disable = self
            .data
            .view_bits::<Lsb0>()[1usize..2usize]
            .load_le::<u8>();
        raw_lv_safe_to_disable == 1
    }
    ///Set value of hvSafeToDisable
    ///- Min: 0
    ///- Max: 1
    pub fn set_hv_safe_to_disable(&mut self, value: bool) -> Result<(), CanError> {
        self.data.view_bits_mut::<Lsb0>()[0usize..1usize].store_le(u8::from(value));
        Ok(())
    }
    ///Set value of lvSafeToDisable
    ///- Min: 0
    ///- Max: 1
    pub fn set_lv_safe_to_disable(&mut self, value: bool) -> Result<(), CanError> {
        self.data.view_bits_mut::<Lsb0>()[1usize..2usize].store_le(u8::from(value));
        Ok(())
    }
}
impl GeneratedCanMessage<{ Self::LEN }> for QuuxAckMsg {
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
#[derive(Debug, Clone)]
pub enum FooLinearControlMsgMux {
    V0(FooLinearControlMsgMux0),
    V1(FooLinearControlMsgMux1),
    V2(FooLinearControlMsgMux2),
}
#[derive(Debug, Clone, Default)]
pub struct FooLinearControlMsgMux0 {
    data: [u8; 8usize],
}
impl FooLinearControlMsgMux0 {
    pub fn new(throttle: f32, brake: f32) -> Result<Self, CanError> {
        let mut msg = Self { data: [0u8; 8usize] };
        msg.set_throttle(throttle)?;
        msg.set_brake(brake)?;
        Ok(msg)
    }
    ///throttle
    ///- Min: 0
    ///- Max: 100
    ///- Unit: %
    ///- Receivers: VectorXXX
    ///- Start bit: 8
    ///- Size: 8 bits
    ///- Factor: 0.4
    ///- Offset: 0
    ///- Byte order: LittleEndian
    ///- Type: unsigned
    pub fn throttle(&self) -> f32 {
        let raw_throttle = self
            .data
            .view_bits::<Lsb0>()[8usize..16usize]
            .load_le::<u8>();
        (raw_throttle as f32) * (0.4f32) + (0f32)
    }
    ///brake
    ///- Min: 0
    ///- Max: 100
    ///- Unit: %
    ///- Receivers: VectorXXX
    ///- Start bit: 16
    ///- Size: 8 bits
    ///- Factor: 0.4
    ///- Offset: 0
    ///- Byte order: LittleEndian
    ///- Type: unsigned
    pub fn brake(&self) -> f32 {
        let raw_brake = self.data.view_bits::<Lsb0>()[16usize..24usize].load_le::<u8>();
        (raw_brake as f32) * (0.4f32) + (0f32)
    }
    ///Set value of throttle
    ///- Min: 0
    ///- Max: 100
    pub fn set_throttle(&mut self, value: f32) -> Result<(), CanError> {
        if value < 0f32 || value > 100f32 {
            return Err(CanError::ValueOutOfRange);
        }
        self.data
            .view_bits_mut::<Lsb0>()[8usize..16usize]
            .store_le(((value - (0f32)) / (0.4f32)) as u8);
        Ok(())
    }
    ///Set value of brake
    ///- Min: 0
    ///- Max: 100
    pub fn set_brake(&mut self, value: f32) -> Result<(), CanError> {
        if value < 0f32 || value > 100f32 {
            return Err(CanError::ValueOutOfRange);
        }
        self.data
            .view_bits_mut::<Lsb0>()[16usize..24usize]
            .store_le(((value - (0f32)) / (0.4f32)) as u8);
        Ok(())
    }
}
#[derive(Debug, Clone, Default)]
pub struct FooLinearControlMsgMux1 {
    data: [u8; 8usize],
}
impl FooLinearControlMsgMux1 {
    pub fn new(torque: i32) -> Result<Self, CanError> {
        let mut msg = Self { data: [0u8; 8usize] };
        msg.set_torque(torque)?;
        Ok(msg)
    }
    ///torque
    ///- Min: -300000
    ///- Max: 300000
    ///- Unit: Nm
    ///- Receivers: VectorXXX
    ///- Start bit: 8
    ///- Size: 24 bits
    ///- Factor: 1
    ///- Offset: 0
    ///- Byte order: LittleEndian
    ///- Type: signed
    pub fn torque(&self) -> i32 {
        let raw_torque = self.data.view_bits::<Lsb0>()[8usize..32usize].load_le::<u32>();
        (raw_torque as i32) * (1i32) + (0i32)
    }
    ///Set value of torque
    ///- Min: -300000
    ///- Max: 300000
    pub fn set_torque(&mut self, value: i32) -> Result<(), CanError> {
        if value < -300000i32 || value > 300000i32 {
            return Err(CanError::ValueOutOfRange);
        }
        self.data
            .view_bits_mut::<Lsb0>()[8usize..32usize]
            .store_le((value - (0i32)) / (1i32));
        Ok(())
    }
}
#[derive(Debug, Clone, Default)]
pub struct FooLinearControlMsgMux2 {
    data: [u8; 8usize],
}
impl FooLinearControlMsgMux2 {
    pub fn new(speed: f32) -> Result<Self, CanError> {
        let mut msg = Self { data: [0u8; 8usize] };
        msg.set_speed(speed)?;
        Ok(msg)
    }
    ///speed
    ///- Min: -35
    ///- Max: 35
    ///- Unit: m/s
    ///- Receivers: VectorXXX
    ///- Start bit: 8
    ///- Size: 16 bits
    ///- Factor: 0.1
    ///- Offset: 0
    ///- Byte order: LittleEndian
    ///- Type: signed
    pub fn speed(&self) -> f32 {
        let raw_speed = self.data.view_bits::<Lsb0>()[8usize..24usize].load_le::<u16>();
        (raw_speed as f32) * (0.1f32) + (0f32)
    }
    ///Set value of speed
    ///- Min: -35
    ///- Max: 35
    pub fn set_speed(&mut self, value: f32) -> Result<(), CanError> {
        if value < -35f32 || value > 35f32 {
            return Err(CanError::ValueOutOfRange);
        }
        self.data
            .view_bits_mut::<Lsb0>()[8usize..24usize]
            .store_le(((value - (0f32)) / (0.1f32)) as u16);
        Ok(())
    }
}
///FooLinearControl
///- ID: Standard 273 (0x111)
///- Size: 8 bytes
///- Transmitter: Bcu
#[derive(Debug, Clone)]
pub struct FooLinearControlMsg {
    data: [u8; 8usize],
}
impl FooLinearControlMsg {
    pub const ID: Id = Id::Standard(unsafe { StandardId::new_unchecked(273u16) });
    pub const LEN: usize = 8usize;
    pub fn new(mux: FooLinearControlMsgMux) -> Result<Self, CanError> {
        let mut msg = Self { data: [0u8; Self::LEN] };
        match mux {
            FooLinearControlMsgMux::V0(v) => {
                msg.set_mux0(v)?;
            }
            FooLinearControlMsgMux::V1(v) => {
                msg.set_mux1(v)?;
            }
            FooLinearControlMsgMux::V2(v) => {
                msg.set_mux2(v)?;
            }
        }
        Ok(msg)
    }
    pub fn mux(&self) -> Result<FooLinearControlMsgMux, CanError> {
        let raw_mux_linear_control = self
            .data
            .view_bits::<Lsb0>()[0usize..8usize]
            .load_le::<u8>();
        match raw_mux_linear_control {
            0 => {
                Ok(
                    FooLinearControlMsgMux::V0(FooLinearControlMsgMux0 {
                        data: self.data,
                    }),
                )
            }
            1 => {
                Ok(
                    FooLinearControlMsgMux::V1(FooLinearControlMsgMux1 {
                        data: self.data,
                    }),
                )
            }
            2 => {
                Ok(
                    FooLinearControlMsgMux::V2(FooLinearControlMsgMux2 {
                        data: self.data,
                    }),
                )
            }
            _ => Err(CanError::UnknownMuxValue),
        }
    }
    pub fn set_mux0(&mut self, value: FooLinearControlMsgMux0) -> Result<(), CanError> {
        let b0 = BitArray::<_, LocalBits>::new(self.data);
        let b1 = BitArray::<_, LocalBits>::new(value.data);
        self.data = b0.bitor(b1).into_inner();
        self.data.view_bits_mut::<Lsb0>()[0usize..8usize].store_le(0u64);
        Ok(())
    }
    pub fn set_mux1(&mut self, value: FooLinearControlMsgMux1) -> Result<(), CanError> {
        let b0 = BitArray::<_, LocalBits>::new(self.data);
        let b1 = BitArray::<_, LocalBits>::new(value.data);
        self.data = b0.bitor(b1).into_inner();
        self.data.view_bits_mut::<Lsb0>()[0usize..8usize].store_le(1u64);
        Ok(())
    }
    pub fn set_mux2(&mut self, value: FooLinearControlMsgMux2) -> Result<(), CanError> {
        let b0 = BitArray::<_, LocalBits>::new(self.data);
        let b1 = BitArray::<_, LocalBits>::new(value.data);
        self.data = b0.bitor(b1).into_inner();
        self.data.view_bits_mut::<Lsb0>()[0usize..8usize].store_le(2u64);
        Ok(())
    }
}
impl GeneratedCanMessage<{ Self::LEN }> for FooLinearControlMsg {
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
#[derive(Debug, Clone)]
pub enum FooAngularControlMsgMux {
    V0(FooAngularControlMsgMux0),
    V1(FooAngularControlMsgMux1),
    V2(FooAngularControlMsgMux2),
}
#[derive(Debug, Clone, Default)]
pub struct FooAngularControlMsgMux0 {
    data: [u8; 8usize],
}
impl FooAngularControlMsgMux0 {
    pub fn new(steer_angle: f32) -> Result<Self, CanError> {
        let mut msg = Self { data: [0u8; 8usize] };
        msg.set_steer_angle(steer_angle)?;
        Ok(msg)
    }
    ///steerAngle
    ///- Min: -2
    ///- Max: 2
    ///- Unit: rad
    ///- Receivers: VectorXXX
    ///- Start bit: 8
    ///- Size: 16 bits
    ///- Factor: 0.0002
    ///- Offset: 0
    ///- Byte order: LittleEndian
    ///- Type: signed
    ///
    ///Leading axle kinematic steering angle target
    pub fn steer_angle(&self) -> f32 {
        let raw_steer_angle = self
            .data
            .view_bits::<Lsb0>()[8usize..24usize]
            .load_le::<u16>();
        (raw_steer_angle as f32) * (0.0002f32) + (0f32)
    }
    ///Set value of steerAngle
    ///- Min: -2
    ///- Max: 2
    ///
    ///Leading axle kinematic steering angle target
    pub fn set_steer_angle(&mut self, value: f32) -> Result<(), CanError> {
        if value < -2f32 || value > 2f32 {
            return Err(CanError::ValueOutOfRange);
        }
        self.data
            .view_bits_mut::<Lsb0>()[8usize..24usize]
            .store_le(((value - (0f32)) / (0.0002f32)) as u16);
        Ok(())
    }
}
#[derive(Debug, Clone, Default)]
pub struct FooAngularControlMsgMux1 {
    data: [u8; 8usize],
}
impl FooAngularControlMsgMux1 {
    pub fn new(curvature: i16) -> Result<Self, CanError> {
        let mut msg = Self { data: [0u8; 8usize] };
        msg.set_curvature(curvature)?;
        Ok(msg)
    }
    ///curvature
    ///- Min: -32768
    ///- Max: 32767
    ///- Unit: 1/m
    ///- Receivers: VectorXXX
    ///- Start bit: 8
    ///- Size: 16 bits
    ///- Factor: 1
    ///- Offset: 0
    ///- Byte order: LittleEndian
    ///- Type: signed
    pub fn curvature(&self) -> i16 {
        let raw_curvature = self
            .data
            .view_bits::<Lsb0>()[8usize..24usize]
            .load_le::<u16>();
        (raw_curvature as i16) * (1i16) + (0i16)
    }
    ///Set value of curvature
    ///- Min: -32768
    ///- Max: 32767
    pub fn set_curvature(&mut self, value: i16) -> Result<(), CanError> {
        self.data
            .view_bits_mut::<Lsb0>()[8usize..24usize]
            .store_le((value - (0i16)) / (1i16));
        Ok(())
    }
}
#[derive(Debug, Clone, Default)]
pub struct FooAngularControlMsgMux2 {
    data: [u8; 8usize],
}
impl FooAngularControlMsgMux2 {
    pub fn new(yaw_rate: f32) -> Result<Self, CanError> {
        let mut msg = Self { data: [0u8; 8usize] };
        msg.set_yaw_rate(yaw_rate)?;
        Ok(msg)
    }
    ///yawRate
    ///- Min: -5
    ///- Max: 5
    ///- Unit: rad/s
    ///- Receivers: VectorXXX
    ///- Start bit: 8
    ///- Size: 16 bits
    ///- Factor: 0.01
    ///- Offset: 0
    ///- Byte order: LittleEndian
    ///- Type: signed
    pub fn yaw_rate(&self) -> f32 {
        let raw_yaw_rate = self
            .data
            .view_bits::<Lsb0>()[8usize..24usize]
            .load_le::<u16>();
        (raw_yaw_rate as f32) * (0.01f32) + (0f32)
    }
    ///Set value of yawRate
    ///- Min: -5
    ///- Max: 5
    pub fn set_yaw_rate(&mut self, value: f32) -> Result<(), CanError> {
        if value < -5f32 || value > 5f32 {
            return Err(CanError::ValueOutOfRange);
        }
        self.data
            .view_bits_mut::<Lsb0>()[8usize..24usize]
            .store_le(((value - (0f32)) / (0.01f32)) as u16);
        Ok(())
    }
}
///FooAngularControl
///- ID: Standard 274 (0x112)
///- Size: 8 bytes
///- Transmitter: Bcu
#[derive(Debug, Clone)]
pub struct FooAngularControlMsg {
    data: [u8; 8usize],
}
impl FooAngularControlMsg {
    pub const ID: Id = Id::Standard(unsafe { StandardId::new_unchecked(274u16) });
    pub const LEN: usize = 8usize;
    pub fn new(mux: FooAngularControlMsgMux) -> Result<Self, CanError> {
        let mut msg = Self { data: [0u8; Self::LEN] };
        match mux {
            FooAngularControlMsgMux::V0(v) => {
                msg.set_mux0(v)?;
            }
            FooAngularControlMsgMux::V1(v) => {
                msg.set_mux1(v)?;
            }
            FooAngularControlMsgMux::V2(v) => {
                msg.set_mux2(v)?;
            }
        }
        Ok(msg)
    }
    pub fn mux(&self) -> Result<FooAngularControlMsgMux, CanError> {
        let raw_mux_angular_control = self
            .data
            .view_bits::<Lsb0>()[0usize..8usize]
            .load_le::<u8>();
        match raw_mux_angular_control {
            0 => {
                Ok(
                    FooAngularControlMsgMux::V0(FooAngularControlMsgMux0 {
                        data: self.data,
                    }),
                )
            }
            1 => {
                Ok(
                    FooAngularControlMsgMux::V1(FooAngularControlMsgMux1 {
                        data: self.data,
                    }),
                )
            }
            2 => {
                Ok(
                    FooAngularControlMsgMux::V2(FooAngularControlMsgMux2 {
                        data: self.data,
                    }),
                )
            }
            _ => Err(CanError::UnknownMuxValue),
        }
    }
    pub fn set_mux0(&mut self, value: FooAngularControlMsgMux0) -> Result<(), CanError> {
        let b0 = BitArray::<_, LocalBits>::new(self.data);
        let b1 = BitArray::<_, LocalBits>::new(value.data);
        self.data = b0.bitor(b1).into_inner();
        self.data.view_bits_mut::<Lsb0>()[0usize..8usize].store_le(0u64);
        Ok(())
    }
    pub fn set_mux1(&mut self, value: FooAngularControlMsgMux1) -> Result<(), CanError> {
        let b0 = BitArray::<_, LocalBits>::new(self.data);
        let b1 = BitArray::<_, LocalBits>::new(value.data);
        self.data = b0.bitor(b1).into_inner();
        self.data.view_bits_mut::<Lsb0>()[0usize..8usize].store_le(1u64);
        Ok(())
    }
    pub fn set_mux2(&mut self, value: FooAngularControlMsgMux2) -> Result<(), CanError> {
        let b0 = BitArray::<_, LocalBits>::new(self.data);
        let b1 = BitArray::<_, LocalBits>::new(value.data);
        self.data = b0.bitor(b1).into_inner();
        self.data.view_bits_mut::<Lsb0>()[0usize..8usize].store_le(2u64);
        Ok(())
    }
}
impl GeneratedCanMessage<{ Self::LEN }> for FooAngularControlMsg {
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
///BarMaintenanceControl
///- ID: Standard 1824 (0x720)
///- Size: 8 bytes
///- Transmitter: BarEcu
#[derive(Debug, Clone)]
pub struct BarMaintenanceControlMsg {
    data: [u8; 8usize],
}
impl BarMaintenanceControlMsg {
    pub const ID: Id = Id::Standard(unsafe { StandardId::new_unchecked(1824u16) });
    pub const LEN: usize = 8usize;
    pub fn new(hv_state_control: HvStateControlEnum) -> Result<Self, CanError> {
        let mut msg = Self { data: [0u8; Self::LEN] };
        msg.set_hv_state_control(hv_state_control)?;
        Ok(msg)
    }
    ///hvStateControl
    ///- Min: 0
    ///- Max: 0
    ///- Unit:
    ///- Receivers: VectorXXX
    ///- Start bit: 0
    ///- Size: 4 bits
    ///- Factor: 1
    ///- Offset: 0
    ///- Byte order: LittleEndian
    ///- Type: unsigned
    ///
    ///Granular control (in maintenance)
    ///Command to Bar ECU to switch HV states.
    pub fn hv_state_control(&self) -> HvStateControlEnum {
        let raw_hv_state_control = self
            .data
            .view_bits::<Lsb0>()[0usize..4usize]
            .load_le::<u8>();
        HvStateControlEnum::from(raw_hv_state_control as u8)
    }
    ///Set value of hvStateControl
    ///- Min: 0
    ///- Max: 0
    ///
    ///Granular control (in maintenance)
    ///Command to Bar ECU to switch HV states.
    pub fn set_hv_state_control(
        &mut self,
        value: HvStateControlEnum,
    ) -> Result<(), CanError> {
        self.data.view_bits_mut::<Lsb0>()[0usize..4usize].store_le(u8::from(value));
        Ok(())
    }
}
impl GeneratedCanMessage<{ Self::LEN }> for BarMaintenanceControlMsg {
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
///BazMaintenanceControl
///- ID: Standard 1856 (0x740)
///- Size: 8 bytes
///- Transmitter: PowerManagementEcu
///
///Control Baz Components independently in Service mode
#[derive(Debug, Clone)]
pub struct BazMaintenanceControlMsg {
    data: [u8; 8usize],
}
impl BazMaintenanceControlMsg {
    pub const ID: Id = Id::Standard(unsafe { StandardId::new_unchecked(1856u16) });
    pub const LEN: usize = 8usize;
    pub fn new(
        hv_battery_enable: bool,
        bc_enable: bool,
        bc_power_reference: f32,
        obc_enable: bool,
        dgs_enable: bool,
        fans_enable: bool,
        can45_supply: bool,
        can45_ignition: bool,
        current_sensor_supply: bool,
        hv_battery_supply: bool,
        hv_battery_ignition: bool,
        obcquux_solenoid_control: bool,
        obc_supply: bool,
        obc_ignition: bool,
        bc_ignition: bool,
        bc_supply: bool,
        feed_pump_ctrl: bool,
    ) -> Result<Self, CanError> {
        let mut msg = Self { data: [0u8; Self::LEN] };
        msg.set_hv_battery_enable(hv_battery_enable)?;
        msg.set_bc_enable(bc_enable)?;
        msg.set_bc_power_reference(bc_power_reference)?;
        msg.set_obc_enable(obc_enable)?;
        msg.set_dgs_enable(dgs_enable)?;
        msg.set_fans_enable(fans_enable)?;
        msg.set_can45_supply(can45_supply)?;
        msg.set_can45_ignition(can45_ignition)?;
        msg.set_current_sensor_supply(current_sensor_supply)?;
        msg.set_hv_battery_supply(hv_battery_supply)?;
        msg.set_hv_battery_ignition(hv_battery_ignition)?;
        msg.set_obcquux_solenoid_control(obcquux_solenoid_control)?;
        msg.set_obc_supply(obc_supply)?;
        msg.set_obc_ignition(obc_ignition)?;
        msg.set_bc_ignition(bc_ignition)?;
        msg.set_bc_supply(bc_supply)?;
        msg.set_feed_pump_ctrl(feed_pump_ctrl)?;
        Ok(msg)
    }
    ///hvBatteryEnable
    ///- Min: 0
    ///- Max: 1
    ///- Unit:
    ///- Receivers: VectorXXX
    ///- Start bit: 0
    ///- Size: 1 bits
    ///- Factor: 1
    ///- Offset: 0
    ///- Byte order: LittleEndian
    ///- Type: unsigned
    ///
    ///HV battery contactor control
    pub fn hv_battery_enable(&self) -> bool {
        let raw_hv_battery_enable = self
            .data
            .view_bits::<Lsb0>()[0usize..1usize]
            .load_le::<u8>();
        raw_hv_battery_enable == 1
    }
    ///bcEnable
    ///- Min: 0
    ///- Max: 1
    ///- Unit:
    ///- Receivers: VectorXXX
    ///- Start bit: 1
    ///- Size: 1 bits
    ///- Factor: 1
    ///- Offset: 0
    ///- Byte order: LittleEndian
    ///- Type: unsigned
    ///
    ///Brake Chopper enable
    pub fn bc_enable(&self) -> bool {
        let raw_bc_enable = self
            .data
            .view_bits::<Lsb0>()[1usize..2usize]
            .load_le::<u8>();
        raw_bc_enable == 1
    }
    ///bcPowerReference
    ///- Min: -4016
    ///- Max: 4016
    ///- Unit: kW
    ///- Receivers: VectorXXX
    ///- Start bit: 48
    ///- Size: 16 bits
    ///- Factor: 0.125
    ///- Offset: -4016
    ///- Byte order: LittleEndian
    ///- Type: unsigned
    ///
    ///Brake Chopper Power reference
    pub fn bc_power_reference(&self) -> f32 {
        let raw_bc_power_reference = self
            .data
            .view_bits::<Lsb0>()[48usize..64usize]
            .load_le::<u16>();
        (raw_bc_power_reference as f32) * (0.125f32) + (-4016f32)
    }
    ///obcEnable
    ///- Min: 0
    ///- Max: 1
    ///- Unit:
    ///- Receivers: VectorXXX
    ///- Start bit: 2
    ///- Size: 1 bits
    ///- Factor: 1
    ///- Offset: 0
    ///- Byte order: LittleEndian
    ///- Type: unsigned
    ///
    ///On-Board Charger control
    pub fn obc_enable(&self) -> bool {
        let raw_obc_enable = self
            .data
            .view_bits::<Lsb0>()[2usize..3usize]
            .load_le::<u8>();
        raw_obc_enable == 1
    }
    ///dgsEnable
    ///- Min: 0
    ///- Max: 1
    ///- Unit:
    ///- Receivers: VectorXXX
    ///- Start bit: 3
    ///- Size: 1 bits
    ///- Factor: 1
    ///- Offset: 0
    ///- Byte order: LittleEndian
    ///- Type: unsigned
    ///
    ///Diesel Generator Set SM enable
    pub fn dgs_enable(&self) -> bool {
        let raw_dgs_enable = self
            .data
            .view_bits::<Lsb0>()[3usize..4usize]
            .load_le::<u8>();
        raw_dgs_enable == 1
    }
    ///fansEnable
    ///- Min: 0
    ///- Max: 1
    ///- Unit:
    ///- Receivers: VectorXXX
    ///- Start bit: 4
    ///- Size: 1 bits
    ///- Factor: 1
    ///- Offset: 0
    ///- Byte order: LittleEndian
    ///- Type: unsigned
    ///
    ///Baz Fans control
    pub fn fans_enable(&self) -> bool {
        let raw_fans_enable = self
            .data
            .view_bits::<Lsb0>()[4usize..5usize]
            .load_le::<u8>();
        raw_fans_enable == 1
    }
    ///can45Supply
    ///- Min: 0
    ///- Max: 1
    ///- Unit:
    ///- Receivers: VectorXXX
    ///- Start bit: 5
    ///- Size: 1 bits
    ///- Factor: 1
    ///- Offset: 0
    ///- Byte order: LittleEndian
    ///- Type: unsigned
    ///
    ///CAN 4 and 5 supply
    pub fn can45_supply(&self) -> bool {
        let raw_can45_supply = self
            .data
            .view_bits::<Lsb0>()[5usize..6usize]
            .load_le::<u8>();
        raw_can45_supply == 1
    }
    ///can45Ignition
    ///- Min: 0
    ///- Max: 1
    ///- Unit:
    ///- Receivers: VectorXXX
    ///- Start bit: 6
    ///- Size: 1 bits
    ///- Factor: 1
    ///- Offset: 0
    ///- Byte order: LittleEndian
    ///- Type: unsigned
    ///
    ///CAN 4 and 5 ignition
    pub fn can45_ignition(&self) -> bool {
        let raw_can45_ignition = self
            .data
            .view_bits::<Lsb0>()[6usize..7usize]
            .load_le::<u8>();
        raw_can45_ignition == 1
    }
    ///currentSensorSupply
    ///- Min: 0
    ///- Max: 1
    ///- Unit:
    ///- Receivers: VectorXXX
    ///- Start bit: 7
    ///- Size: 1 bits
    ///- Factor: 1
    ///- Offset: 0
    ///- Byte order: LittleEndian
    ///- Type: unsigned
    ///
    ///Current Sensor supply
    pub fn current_sensor_supply(&self) -> bool {
        let raw_current_sensor_supply = self
            .data
            .view_bits::<Lsb0>()[7usize..8usize]
            .load_le::<u8>();
        raw_current_sensor_supply == 1
    }
    ///hvBatterySupply
    ///- Min: 0
    ///- Max: 1
    ///- Unit:
    ///- Receivers: VectorXXX
    ///- Start bit: 8
    ///- Size: 1 bits
    ///- Factor: 1
    ///- Offset: 0
    ///- Byte order: LittleEndian
    ///- Type: unsigned
    ///
    ///HVESS supply
    pub fn hv_battery_supply(&self) -> bool {
        let raw_hv_battery_supply = self
            .data
            .view_bits::<Lsb0>()[8usize..9usize]
            .load_le::<u8>();
        raw_hv_battery_supply == 1
    }
    ///hvBatteryIgnition
    ///- Min: 0
    ///- Max: 1
    ///- Unit:
    ///- Receivers: VectorXXX
    ///- Start bit: 9
    ///- Size: 1 bits
    ///- Factor: 1
    ///- Offset: 0
    ///- Byte order: LittleEndian
    ///- Type: unsigned
    ///
    ///HVESS ignition
    pub fn hv_battery_ignition(&self) -> bool {
        let raw_hv_battery_ignition = self
            .data
            .view_bits::<Lsb0>()[9usize..10usize]
            .load_le::<u8>();
        raw_hv_battery_ignition == 1
    }
    ///obcquuxSolenoidControl
    ///- Min: 0
    ///- Max: 1
    ///- Unit:
    ///- Receivers: VectorXXX
    ///- Start bit: 10
    ///- Size: 1 bits
    ///- Factor: 1
    ///- Offset: 0
    ///- Byte order: LittleEndian
    ///- Type: unsigned
    ///
    ///On Board Charger quux solenoid control
    pub fn obcquux_solenoid_control(&self) -> bool {
        let raw_obcquux_solenoid_control = self
            .data
            .view_bits::<Lsb0>()[10usize..11usize]
            .load_le::<u8>();
        raw_obcquux_solenoid_control == 1
    }
    ///obcSupply
    ///- Min: 0
    ///- Max: 1
    ///- Unit:
    ///- Receivers: VectorXXX
    ///- Start bit: 11
    ///- Size: 1 bits
    ///- Factor: 1
    ///- Offset: 0
    ///- Byte order: LittleEndian
    ///- Type: unsigned
    ///
    ///On Board Charger supply
    pub fn obc_supply(&self) -> bool {
        let raw_obc_supply = self
            .data
            .view_bits::<Lsb0>()[11usize..12usize]
            .load_le::<u8>();
        raw_obc_supply == 1
    }
    ///obcIgnition
    ///- Min: 0
    ///- Max: 1
    ///- Unit:
    ///- Receivers: VectorXXX
    ///- Start bit: 12
    ///- Size: 1 bits
    ///- Factor: 1
    ///- Offset: 0
    ///- Byte order: LittleEndian
    ///- Type: unsigned
    ///
    ///On Board Charger ignition
    pub fn obc_ignition(&self) -> bool {
        let raw_obc_ignition = self
            .data
            .view_bits::<Lsb0>()[12usize..13usize]
            .load_le::<u8>();
        raw_obc_ignition == 1
    }
    ///bcIgnition
    ///- Min: 0
    ///- Max: 1
    ///- Unit:
    ///- Receivers: VectorXXX
    ///- Start bit: 13
    ///- Size: 1 bits
    ///- Factor: 1
    ///- Offset: 0
    ///- Byte order: LittleEndian
    ///- Type: unsigned
    ///
    ///Brake Chopper ignition
    pub fn bc_ignition(&self) -> bool {
        let raw_bc_ignition = self
            .data
            .view_bits::<Lsb0>()[13usize..14usize]
            .load_le::<u8>();
        raw_bc_ignition == 1
    }
    ///bcSupply
    ///- Min: 0
    ///- Max: 1
    ///- Unit:
    ///- Receivers: VectorXXX
    ///- Start bit: 14
    ///- Size: 1 bits
    ///- Factor: 1
    ///- Offset: 0
    ///- Byte order: LittleEndian
    ///- Type: unsigned
    ///
    ///Brake Chopper supply
    pub fn bc_supply(&self) -> bool {
        let raw_bc_supply = self
            .data
            .view_bits::<Lsb0>()[14usize..15usize]
            .load_le::<u8>();
        raw_bc_supply == 1
    }
    ///feedPumpCtrl
    ///- Min: 0
    ///- Max: 1
    ///- Unit:
    ///- Receivers: VectorXXX
    ///- Start bit: 15
    ///- Size: 1 bits
    ///- Factor: 1
    ///- Offset: 0
    ///- Byte order: LittleEndian
    ///- Type: unsigned
    ///
    ///Feed Pump control
    pub fn feed_pump_ctrl(&self) -> bool {
        let raw_feed_pump_ctrl = self
            .data
            .view_bits::<Lsb0>()[15usize..16usize]
            .load_le::<u8>();
        raw_feed_pump_ctrl == 1
    }
    ///Set value of hvBatteryEnable
    ///- Min: 0
    ///- Max: 1
    ///
    ///HV battery contactor control
    pub fn set_hv_battery_enable(&mut self, value: bool) -> Result<(), CanError> {
        self.data.view_bits_mut::<Lsb0>()[0usize..1usize].store_le(u8::from(value));
        Ok(())
    }
    ///Set value of bcEnable
    ///- Min: 0
    ///- Max: 1
    ///
    ///Brake Chopper enable
    pub fn set_bc_enable(&mut self, value: bool) -> Result<(), CanError> {
        self.data.view_bits_mut::<Lsb0>()[1usize..2usize].store_le(u8::from(value));
        Ok(())
    }
    ///Set value of bcPowerReference
    ///- Min: -4016
    ///- Max: 4016
    ///
    ///Brake Chopper Power reference
    pub fn set_bc_power_reference(&mut self, value: f32) -> Result<(), CanError> {
        if value < -4016f32 || value > 4016f32 {
            return Err(CanError::ValueOutOfRange);
        }
        self.data
            .view_bits_mut::<Lsb0>()[48usize..64usize]
            .store_le(((value - (-4016f32)) / (0.125f32)) as u16);
        Ok(())
    }
    ///Set value of obcEnable
    ///- Min: 0
    ///- Max: 1
    ///
    ///On-Board Charger control
    pub fn set_obc_enable(&mut self, value: bool) -> Result<(), CanError> {
        self.data.view_bits_mut::<Lsb0>()[2usize..3usize].store_le(u8::from(value));
        Ok(())
    }
    ///Set value of dgsEnable
    ///- Min: 0
    ///- Max: 1
    ///
    ///Diesel Generator Set SM enable
    pub fn set_dgs_enable(&mut self, value: bool) -> Result<(), CanError> {
        self.data.view_bits_mut::<Lsb0>()[3usize..4usize].store_le(u8::from(value));
        Ok(())
    }
    ///Set value of fansEnable
    ///- Min: 0
    ///- Max: 1
    ///
    ///Baz Fans control
    pub fn set_fans_enable(&mut self, value: bool) -> Result<(), CanError> {
        self.data.view_bits_mut::<Lsb0>()[4usize..5usize].store_le(u8::from(value));
        Ok(())
    }
    ///Set value of can45Supply
    ///- Min: 0
    ///- Max: 1
    ///
    ///CAN 4 and 5 supply
    pub fn set_can45_supply(&mut self, value: bool) -> Result<(), CanError> {
        self.data.view_bits_mut::<Lsb0>()[5usize..6usize].store_le(u8::from(value));
        Ok(())
    }
    ///Set value of can45Ignition
    ///- Min: 0
    ///- Max: 1
    ///
    ///CAN 4 and 5 ignition
    pub fn set_can45_ignition(&mut self, value: bool) -> Result<(), CanError> {
        self.data.view_bits_mut::<Lsb0>()[6usize..7usize].store_le(u8::from(value));
        Ok(())
    }
    ///Set value of currentSensorSupply
    ///- Min: 0
    ///- Max: 1
    ///
    ///Current Sensor supply
    pub fn set_current_sensor_supply(&mut self, value: bool) -> Result<(), CanError> {
        self.data.view_bits_mut::<Lsb0>()[7usize..8usize].store_le(u8::from(value));
        Ok(())
    }
    ///Set value of hvBatterySupply
    ///- Min: 0
    ///- Max: 1
    ///
    ///HVESS supply
    pub fn set_hv_battery_supply(&mut self, value: bool) -> Result<(), CanError> {
        self.data.view_bits_mut::<Lsb0>()[8usize..9usize].store_le(u8::from(value));
        Ok(())
    }
    ///Set value of hvBatteryIgnition
    ///- Min: 0
    ///- Max: 1
    ///
    ///HVESS ignition
    pub fn set_hv_battery_ignition(&mut self, value: bool) -> Result<(), CanError> {
        self.data.view_bits_mut::<Lsb0>()[9usize..10usize].store_le(u8::from(value));
        Ok(())
    }
    ///Set value of obcquuxSolenoidControl
    ///- Min: 0
    ///- Max: 1
    ///
    ///On Board Charger quux solenoid control
    pub fn set_obcquux_solenoid_control(&mut self, value: bool) -> Result<(), CanError> {
        self.data.view_bits_mut::<Lsb0>()[10usize..11usize].store_le(u8::from(value));
        Ok(())
    }
    ///Set value of obcSupply
    ///- Min: 0
    ///- Max: 1
    ///
    ///On Board Charger supply
    pub fn set_obc_supply(&mut self, value: bool) -> Result<(), CanError> {
        self.data.view_bits_mut::<Lsb0>()[11usize..12usize].store_le(u8::from(value));
        Ok(())
    }
    ///Set value of obcIgnition
    ///- Min: 0
    ///- Max: 1
    ///
    ///On Board Charger ignition
    pub fn set_obc_ignition(&mut self, value: bool) -> Result<(), CanError> {
        self.data.view_bits_mut::<Lsb0>()[12usize..13usize].store_le(u8::from(value));
        Ok(())
    }
    ///Set value of bcIgnition
    ///- Min: 0
    ///- Max: 1
    ///
    ///Brake Chopper ignition
    pub fn set_bc_ignition(&mut self, value: bool) -> Result<(), CanError> {
        self.data.view_bits_mut::<Lsb0>()[13usize..14usize].store_le(u8::from(value));
        Ok(())
    }
    ///Set value of bcSupply
    ///- Min: 0
    ///- Max: 1
    ///
    ///Brake Chopper supply
    pub fn set_bc_supply(&mut self, value: bool) -> Result<(), CanError> {
        self.data.view_bits_mut::<Lsb0>()[14usize..15usize].store_le(u8::from(value));
        Ok(())
    }
    ///Set value of feedPumpCtrl
    ///- Min: 0
    ///- Max: 1
    ///
    ///Feed Pump control
    pub fn set_feed_pump_ctrl(&mut self, value: bool) -> Result<(), CanError> {
        self.data.view_bits_mut::<Lsb0>()[15usize..16usize].store_le(u8::from(value));
        Ok(())
    }
}
impl GeneratedCanMessage<{ Self::LEN }> for BazMaintenanceControlMsg {
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
///quuxMaintenanceControl1
///- ID: Standard 1872 (0x750)
///- Size: 8 bytes
///- Transmitter: Bcu
#[derive(Debug, Clone)]
pub struct QuuxMaintenanceControl1Msg {
    data: [u8; 8usize],
}
impl QuuxMaintenanceControl1Msg {
    pub const ID: Id = Id::Standard(unsafe { StandardId::new_unchecked(1872u16) });
    pub const LEN: usize = 8usize;
    pub fn new(
        fan_speed_setpoint: f32,
        fan_direct_control_enable: bool,
        ac_enable: bool,
        ac_reset: bool,
        ac_setpoint: f32,
        chiller_enable: bool,
        chiller_reset: bool,
        chiller_setpoint: f32,
        ac_direct_control_enable: bool,
        chiller_direct_control_enable: bool,
        ac_power_limit: u8,
        chiller_power_limit: u8,
        liquid_chiller_pump_direct_control_en: bool,
        liquid_chiller_pump_on: bool,
        lt_bar_pump_direct_control_ena: bool,
        lt_bar_pump_on: bool,
        lt_foo_pump_direct_control_enable: bool,
        lt_foo_pump_on: bool,
        bc_cac_pump_direct_control_enable: bool,
        bc_cac_pump_on: bool,
    ) -> Result<Self, CanError> {
        let mut msg = Self { data: [0u8; Self::LEN] };
        msg.set_fan_speed_setpoint(fan_speed_setpoint)?;
        msg.set_fan_direct_control_enable(fan_direct_control_enable)?;
        msg.set_ac_enable(ac_enable)?;
        msg.set_ac_reset(ac_reset)?;
        msg.set_ac_setpoint(ac_setpoint)?;
        msg.set_chiller_enable(chiller_enable)?;
        msg.set_chiller_reset(chiller_reset)?;
        msg.set_chiller_setpoint(chiller_setpoint)?;
        msg.set_ac_direct_control_enable(ac_direct_control_enable)?;
        msg.set_chiller_direct_control_enable(chiller_direct_control_enable)?;
        msg.set_ac_power_limit(ac_power_limit)?;
        msg.set_chiller_power_limit(chiller_power_limit)?;
        msg.set_liquid_chiller_pump_direct_control_en(
            liquid_chiller_pump_direct_control_en,
        )?;
        msg.set_liquid_chiller_pump_on(liquid_chiller_pump_on)?;
        msg.set_lt_bar_pump_direct_control_ena(lt_bar_pump_direct_control_ena)?;
        msg.set_lt_bar_pump_on(lt_bar_pump_on)?;
        msg.set_lt_foo_pump_direct_control_enable(lt_foo_pump_direct_control_enable)?;
        msg.set_lt_foo_pump_on(lt_foo_pump_on)?;
        msg.set_bc_cac_pump_direct_control_enable(bc_cac_pump_direct_control_enable)?;
        msg.set_bc_cac_pump_on(bc_cac_pump_on)?;
        Ok(msg)
    }
    ///fanSpeedSetpoint
    ///- Min: -16064
    ///- Max: 16063.5
    ///- Unit: RPM
    ///- Receivers: VectorXXX
    ///- Start bit: 0
    ///- Size: 16 bits
    ///- Factor: 0.5
    ///- Offset: -16064
    ///- Byte order: LittleEndian
    ///- Type: unsigned
    pub fn fan_speed_setpoint(&self) -> f32 {
        let raw_fan_speed_setpoint = self
            .data
            .view_bits::<Lsb0>()[0usize..16usize]
            .load_le::<u16>();
        (raw_fan_speed_setpoint as f32) * (0.5f32) + (-16064f32)
    }
    ///fanDirectControlEnable
    ///- Min: 0
    ///- Max: 1
    ///- Unit:
    ///- Receivers: VectorXXX
    ///- Start bit: 39
    ///- Size: 1 bits
    ///- Factor: 1
    ///- Offset: 0
    ///- Byte order: LittleEndian
    ///- Type: unsigned
    pub fn fan_direct_control_enable(&self) -> bool {
        let raw_fan_direct_control_enable = self
            .data
            .view_bits::<Lsb0>()[39usize..40usize]
            .load_le::<u8>();
        raw_fan_direct_control_enable == 1
    }
    ///acEnable
    ///- Min: 0
    ///- Max: 1
    ///- Unit:
    ///- Receivers: VectorXXX
    ///- Start bit: 33
    ///- Size: 1 bits
    ///- Factor: 1
    ///- Offset: 0
    ///- Byte order: LittleEndian
    ///- Type: unsigned
    pub fn ac_enable(&self) -> bool {
        let raw_ac_enable = self
            .data
            .view_bits::<Lsb0>()[33usize..34usize]
            .load_le::<u8>();
        raw_ac_enable == 1
    }
    ///acReset
    ///- Min: 0
    ///- Max: 1
    ///- Unit:
    ///- Receivers: VectorXXX
    ///- Start bit: 34
    ///- Size: 1 bits
    ///- Factor: 1
    ///- Offset: 0
    ///- Byte order: LittleEndian
    ///- Type: unsigned
    pub fn ac_reset(&self) -> bool {
        let raw_ac_reset = self
            .data
            .view_bits::<Lsb0>()[34usize..35usize]
            .load_le::<u8>();
        raw_ac_reset == 1
    }
    ///acSetpoint
    ///- Min: 15
    ///- Max: 35
    ///- Unit: degC
    ///- Receivers: VectorXXX
    ///- Start bit: 16
    ///- Size: 8 bits
    ///- Factor: 0.1
    ///- Offset: 10
    ///- Byte order: LittleEndian
    ///- Type: unsigned
    pub fn ac_setpoint(&self) -> f32 {
        let raw_ac_setpoint = self
            .data
            .view_bits::<Lsb0>()[16usize..24usize]
            .load_le::<u8>();
        (raw_ac_setpoint as f32) * (0.1f32) + (10f32)
    }
    ///chillerEnable
    ///- Min: 0
    ///- Max: 1
    ///- Unit:
    ///- Receivers: VectorXXX
    ///- Start bit: 37
    ///- Size: 1 bits
    ///- Factor: 1
    ///- Offset: 0
    ///- Byte order: LittleEndian
    ///- Type: unsigned
    pub fn chiller_enable(&self) -> bool {
        let raw_chiller_enable = self
            .data
            .view_bits::<Lsb0>()[37usize..38usize]
            .load_le::<u8>();
        raw_chiller_enable == 1
    }
    ///chillerReset
    ///- Min: 0
    ///- Max: 1
    ///- Unit:
    ///- Receivers: VectorXXX
    ///- Start bit: 38
    ///- Size: 1 bits
    ///- Factor: 1
    ///- Offset: 0
    ///- Byte order: LittleEndian
    ///- Type: unsigned
    pub fn chiller_reset(&self) -> bool {
        let raw_chiller_reset = self
            .data
            .view_bits::<Lsb0>()[38usize..39usize]
            .load_le::<u8>();
        raw_chiller_reset == 1
    }
    ///chillerSetpoint
    ///- Min: 15
    ///- Max: 35
    ///- Unit: degC
    ///- Receivers: VectorXXX
    ///- Start bit: 24
    ///- Size: 8 bits
    ///- Factor: 0.1
    ///- Offset: 10
    ///- Byte order: LittleEndian
    ///- Type: unsigned
    pub fn chiller_setpoint(&self) -> f32 {
        let raw_chiller_setpoint = self
            .data
            .view_bits::<Lsb0>()[24usize..32usize]
            .load_le::<u8>();
        (raw_chiller_setpoint as f32) * (0.1f32) + (10f32)
    }
    ///acDirectControlEnable
    ///- Min: 0
    ///- Max: 1
    ///- Unit:
    ///- Receivers: VectorXXX
    ///- Start bit: 32
    ///- Size: 1 bits
    ///- Factor: 1
    ///- Offset: 0
    ///- Byte order: LittleEndian
    ///- Type: unsigned
    pub fn ac_direct_control_enable(&self) -> bool {
        let raw_ac_direct_control_enable = self
            .data
            .view_bits::<Lsb0>()[32usize..33usize]
            .load_le::<u8>();
        raw_ac_direct_control_enable == 1
    }
    ///chillerDirectControlEnable
    ///- Min: 0
    ///- Max: 1
    ///- Unit:
    ///- Receivers: VectorXXX
    ///- Start bit: 36
    ///- Size: 1 bits
    ///- Factor: 1
    ///- Offset: 0
    ///- Byte order: LittleEndian
    ///- Type: unsigned
    pub fn chiller_direct_control_enable(&self) -> bool {
        let raw_chiller_direct_control_enable = self
            .data
            .view_bits::<Lsb0>()[36usize..37usize]
            .load_le::<u8>();
        raw_chiller_direct_control_enable == 1
    }
    ///acPowerLimit
    ///- Min: 0
    ///- Max: 100
    ///- Unit: %
    ///- Receivers: VectorXXX
    ///- Start bit: 40
    ///- Size: 8 bits
    ///- Factor: 1
    ///- Offset: 0
    ///- Byte order: LittleEndian
    ///- Type: unsigned
    pub fn ac_power_limit(&self) -> u8 {
        let raw_ac_power_limit = self
            .data
            .view_bits::<Lsb0>()[40usize..48usize]
            .load_le::<u8>();
        (raw_ac_power_limit) * (1u8) + (0u8)
    }
    ///chillerPowerLimit
    ///- Min: 0
    ///- Max: 100
    ///- Unit: %
    ///- Receivers: VectorXXX
    ///- Start bit: 48
    ///- Size: 8 bits
    ///- Factor: 1
    ///- Offset: 0
    ///- Byte order: LittleEndian
    ///- Type: unsigned
    pub fn chiller_power_limit(&self) -> u8 {
        let raw_chiller_power_limit = self
            .data
            .view_bits::<Lsb0>()[48usize..56usize]
            .load_le::<u8>();
        (raw_chiller_power_limit) * (1u8) + (0u8)
    }
    ///liquidChillerPumpDirectControlEn
    ///- Min: 0
    ///- Max: 1
    ///- Unit:
    ///- Receivers: VectorXXX
    ///- Start bit: 56
    ///- Size: 1 bits
    ///- Factor: 1
    ///- Offset: 0
    ///- Byte order: LittleEndian
    ///- Type: unsigned
    ///
    ///liquidChillerPumpDirectControlEnable
    pub fn liquid_chiller_pump_direct_control_en(&self) -> bool {
        let raw_liquid_chiller_pump_direct_control_en = self
            .data
            .view_bits::<Lsb0>()[56usize..57usize]
            .load_le::<u8>();
        raw_liquid_chiller_pump_direct_control_en == 1
    }
    ///liquidChillerPumpOn
    ///- Min: 0
    ///- Max: 1
    ///- Unit:
    ///- Receivers: VectorXXX
    ///- Start bit: 57
    ///- Size: 1 bits
    ///- Factor: 1
    ///- Offset: 0
    ///- Byte order: LittleEndian
    ///- Type: unsigned
    pub fn liquid_chiller_pump_on(&self) -> bool {
        let raw_liquid_chiller_pump_on = self
            .data
            .view_bits::<Lsb0>()[57usize..58usize]
            .load_le::<u8>();
        raw_liquid_chiller_pump_on == 1
    }
    ///ltBarPumpDirectControlEna
    ///- Min: 0
    ///- Max: 1
    ///- Unit:
    ///- Receivers: VectorXXX
    ///- Start bit: 58
    ///- Size: 1 bits
    ///- Factor: 1
    ///- Offset: 0
    ///- Byte order: LittleEndian
    ///- Type: unsigned
    ///
    ///ltBarPumpDirectControlEnable
    pub fn lt_bar_pump_direct_control_ena(&self) -> bool {
        let raw_lt_bar_pump_direct_control_ena = self
            .data
            .view_bits::<Lsb0>()[58usize..59usize]
            .load_le::<u8>();
        raw_lt_bar_pump_direct_control_ena == 1
    }
    ///ltBarPumpOn
    ///- Min: 0
    ///- Max: 1
    ///- Unit:
    ///- Receivers: VectorXXX
    ///- Start bit: 59
    ///- Size: 1 bits
    ///- Factor: 1
    ///- Offset: 0
    ///- Byte order: LittleEndian
    ///- Type: unsigned
    pub fn lt_bar_pump_on(&self) -> bool {
        let raw_lt_bar_pump_on = self
            .data
            .view_bits::<Lsb0>()[59usize..60usize]
            .load_le::<u8>();
        raw_lt_bar_pump_on == 1
    }
    ///ltFooPumpDirectControlEnable
    ///- Min: 0
    ///- Max: 1
    ///- Unit:
    ///- Receivers: VectorXXX
    ///- Start bit: 60
    ///- Size: 1 bits
    ///- Factor: 1
    ///- Offset: 0
    ///- Byte order: LittleEndian
    ///- Type: unsigned
    pub fn lt_foo_pump_direct_control_enable(&self) -> bool {
        let raw_lt_foo_pump_direct_control_enable = self
            .data
            .view_bits::<Lsb0>()[60usize..61usize]
            .load_le::<u8>();
        raw_lt_foo_pump_direct_control_enable == 1
    }
    ///ltFooPumpOn
    ///- Min: 0
    ///- Max: 1
    ///- Unit:
    ///- Receivers: VectorXXX
    ///- Start bit: 61
    ///- Size: 1 bits
    ///- Factor: 1
    ///- Offset: 0
    ///- Byte order: LittleEndian
    ///- Type: unsigned
    pub fn lt_foo_pump_on(&self) -> bool {
        let raw_lt_foo_pump_on = self
            .data
            .view_bits::<Lsb0>()[61usize..62usize]
            .load_le::<u8>();
        raw_lt_foo_pump_on == 1
    }
    ///bcCacPumpDirectControlEnable
    ///- Min: 0
    ///- Max: 1
    ///- Unit:
    ///- Receivers: VectorXXX
    ///- Start bit: 62
    ///- Size: 1 bits
    ///- Factor: 1
    ///- Offset: 0
    ///- Byte order: LittleEndian
    ///- Type: unsigned
    pub fn bc_cac_pump_direct_control_enable(&self) -> bool {
        let raw_bc_cac_pump_direct_control_enable = self
            .data
            .view_bits::<Lsb0>()[62usize..63usize]
            .load_le::<u8>();
        raw_bc_cac_pump_direct_control_enable == 1
    }
    ///bcCacPumpOn
    ///- Min: 0
    ///- Max: 1
    ///- Unit:
    ///- Receivers: VectorXXX
    ///- Start bit: 63
    ///- Size: 1 bits
    ///- Factor: 1
    ///- Offset: 0
    ///- Byte order: LittleEndian
    ///- Type: unsigned
    pub fn bc_cac_pump_on(&self) -> bool {
        let raw_bc_cac_pump_on = self
            .data
            .view_bits::<Lsb0>()[63usize..64usize]
            .load_le::<u8>();
        raw_bc_cac_pump_on == 1
    }
    ///Set value of fanSpeedSetpoint
    ///- Min: -16064
    ///- Max: 16063.5
    pub fn set_fan_speed_setpoint(&mut self, value: f32) -> Result<(), CanError> {
        if value < -16064f32 || value > 16063.5f32 {
            return Err(CanError::ValueOutOfRange);
        }
        self.data
            .view_bits_mut::<Lsb0>()[0usize..16usize]
            .store_le(((value - (-16064f32)) / (0.5f32)) as u16);
        Ok(())
    }
    ///Set value of fanDirectControlEnable
    ///- Min: 0
    ///- Max: 1
    pub fn set_fan_direct_control_enable(
        &mut self,
        value: bool,
    ) -> Result<(), CanError> {
        self.data.view_bits_mut::<Lsb0>()[39usize..40usize].store_le(u8::from(value));
        Ok(())
    }
    ///Set value of acEnable
    ///- Min: 0
    ///- Max: 1
    pub fn set_ac_enable(&mut self, value: bool) -> Result<(), CanError> {
        self.data.view_bits_mut::<Lsb0>()[33usize..34usize].store_le(u8::from(value));
        Ok(())
    }
    ///Set value of acReset
    ///- Min: 0
    ///- Max: 1
    pub fn set_ac_reset(&mut self, value: bool) -> Result<(), CanError> {
        self.data.view_bits_mut::<Lsb0>()[34usize..35usize].store_le(u8::from(value));
        Ok(())
    }
    ///Set value of acSetpoint
    ///- Min: 15
    ///- Max: 35
    pub fn set_ac_setpoint(&mut self, value: f32) -> Result<(), CanError> {
        if value < 15f32 || value > 35f32 {
            return Err(CanError::ValueOutOfRange);
        }
        self.data
            .view_bits_mut::<Lsb0>()[16usize..24usize]
            .store_le(((value - (10f32)) / (0.1f32)) as u8);
        Ok(())
    }
    ///Set value of chillerEnable
    ///- Min: 0
    ///- Max: 1
    pub fn set_chiller_enable(&mut self, value: bool) -> Result<(), CanError> {
        self.data.view_bits_mut::<Lsb0>()[37usize..38usize].store_le(u8::from(value));
        Ok(())
    }
    ///Set value of chillerReset
    ///- Min: 0
    ///- Max: 1
    pub fn set_chiller_reset(&mut self, value: bool) -> Result<(), CanError> {
        self.data.view_bits_mut::<Lsb0>()[38usize..39usize].store_le(u8::from(value));
        Ok(())
    }
    ///Set value of chillerSetpoint
    ///- Min: 15
    ///- Max: 35
    pub fn set_chiller_setpoint(&mut self, value: f32) -> Result<(), CanError> {
        if value < 15f32 || value > 35f32 {
            return Err(CanError::ValueOutOfRange);
        }
        self.data
            .view_bits_mut::<Lsb0>()[24usize..32usize]
            .store_le(((value - (10f32)) / (0.1f32)) as u8);
        Ok(())
    }
    ///Set value of acDirectControlEnable
    ///- Min: 0
    ///- Max: 1
    pub fn set_ac_direct_control_enable(&mut self, value: bool) -> Result<(), CanError> {
        self.data.view_bits_mut::<Lsb0>()[32usize..33usize].store_le(u8::from(value));
        Ok(())
    }
    ///Set value of chillerDirectControlEnable
    ///- Min: 0
    ///- Max: 1
    pub fn set_chiller_direct_control_enable(
        &mut self,
        value: bool,
    ) -> Result<(), CanError> {
        self.data.view_bits_mut::<Lsb0>()[36usize..37usize].store_le(u8::from(value));
        Ok(())
    }
    ///Set value of acPowerLimit
    ///- Min: 0
    ///- Max: 100
    pub fn set_ac_power_limit(&mut self, value: u8) -> Result<(), CanError> {
        if value > 100u8 {
            return Err(CanError::ValueOutOfRange);
        }
        self.data
            .view_bits_mut::<Lsb0>()[40usize..48usize]
            .store_le((value - (0u8)) / (1u8));
        Ok(())
    }
    ///Set value of chillerPowerLimit
    ///- Min: 0
    ///- Max: 100
    pub fn set_chiller_power_limit(&mut self, value: u8) -> Result<(), CanError> {
        if value > 100u8 {
            return Err(CanError::ValueOutOfRange);
        }
        self.data
            .view_bits_mut::<Lsb0>()[48usize..56usize]
            .store_le((value - (0u8)) / (1u8));
        Ok(())
    }
    ///Set value of liquidChillerPumpDirectControlEn
    ///- Min: 0
    ///- Max: 1
    ///
    ///liquidChillerPumpDirectControlEnable
    pub fn set_liquid_chiller_pump_direct_control_en(
        &mut self,
        value: bool,
    ) -> Result<(), CanError> {
        self.data.view_bits_mut::<Lsb0>()[56usize..57usize].store_le(u8::from(value));
        Ok(())
    }
    ///Set value of liquidChillerPumpOn
    ///- Min: 0
    ///- Max: 1
    pub fn set_liquid_chiller_pump_on(&mut self, value: bool) -> Result<(), CanError> {
        self.data.view_bits_mut::<Lsb0>()[57usize..58usize].store_le(u8::from(value));
        Ok(())
    }
    ///Set value of ltBarPumpDirectControlEna
    ///- Min: 0
    ///- Max: 1
    ///
    ///ltBarPumpDirectControlEnable
    pub fn set_lt_bar_pump_direct_control_ena(
        &mut self,
        value: bool,
    ) -> Result<(), CanError> {
        self.data.view_bits_mut::<Lsb0>()[58usize..59usize].store_le(u8::from(value));
        Ok(())
    }
    ///Set value of ltBarPumpOn
    ///- Min: 0
    ///- Max: 1
    pub fn set_lt_bar_pump_on(&mut self, value: bool) -> Result<(), CanError> {
        self.data.view_bits_mut::<Lsb0>()[59usize..60usize].store_le(u8::from(value));
        Ok(())
    }
    ///Set value of ltFooPumpDirectControlEnable
    ///- Min: 0
    ///- Max: 1
    pub fn set_lt_foo_pump_direct_control_enable(
        &mut self,
        value: bool,
    ) -> Result<(), CanError> {
        self.data.view_bits_mut::<Lsb0>()[60usize..61usize].store_le(u8::from(value));
        Ok(())
    }
    ///Set value of ltFooPumpOn
    ///- Min: 0
    ///- Max: 1
    pub fn set_lt_foo_pump_on(&mut self, value: bool) -> Result<(), CanError> {
        self.data.view_bits_mut::<Lsb0>()[61usize..62usize].store_le(u8::from(value));
        Ok(())
    }
    ///Set value of bcCacPumpDirectControlEnable
    ///- Min: 0
    ///- Max: 1
    pub fn set_bc_cac_pump_direct_control_enable(
        &mut self,
        value: bool,
    ) -> Result<(), CanError> {
        self.data.view_bits_mut::<Lsb0>()[62usize..63usize].store_le(u8::from(value));
        Ok(())
    }
    ///Set value of bcCacPumpOn
    ///- Min: 0
    ///- Max: 1
    pub fn set_bc_cac_pump_on(&mut self, value: bool) -> Result<(), CanError> {
        self.data.view_bits_mut::<Lsb0>()[63usize..64usize].store_le(u8::from(value));
        Ok(())
    }
}
impl GeneratedCanMessage<{ Self::LEN }> for QuuxMaintenanceControl1Msg {
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
///ContactorUpdateResponse
///- ID: Standard 2017 (0x7E1)
///- Size: 8 bytes
///- Transmitter: BarEcu
///
///Respond from contactor Foor
#[derive(Debug, Clone)]
pub struct ContactorUpdateResponseMsg {
    data: [u8; 8usize],
}
impl ContactorUpdateResponseMsg {
    pub const ID: Id = Id::Standard(unsafe { StandardId::new_unchecked(2017u16) });
    pub const LEN: usize = 8usize;
    pub fn new() -> Result<Self, CanError> {
        let mut msg = Self { data: [0u8; Self::LEN] };
        Ok(msg)
    }
}
impl GeneratedCanMessage<{ Self::LEN }> for ContactorUpdateResponseMsg {
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
///ContactorUpdateRequest
///- ID: Standard 1639 (0x667)
///- Size: 8 bytes
///- Transmitter: Bcu
///
///Request to contactor Foor
#[derive(Debug, Clone)]
pub struct ContactorUpdateRequestMsg {
    data: [u8; 8usize],
}
impl ContactorUpdateRequestMsg {
    pub const ID: Id = Id::Standard(unsafe { StandardId::new_unchecked(1639u16) });
    pub const LEN: usize = 8usize;
    pub fn new() -> Result<Self, CanError> {
        let mut msg = Self { data: [0u8; Self::LEN] };
        Ok(msg)
    }
}
impl GeneratedCanMessage<{ Self::LEN }> for ContactorUpdateRequestMsg {
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
///FooContactorBootRequest
///- ID: Standard 1281 (0x501)
///- Size: 8 bytes
///- Transmitter: Bcu
#[derive(Debug, Clone)]
pub struct FooContactorBootRequestMsg {
    data: [u8; 8usize],
}
impl FooContactorBootRequestMsg {
    pub const ID: Id = Id::Standard(unsafe { StandardId::new_unchecked(1281u16) });
    pub const LEN: usize = 8usize;
    pub fn new(
        contactor_to_send_boot: ContactorToSendBootEnum,
    ) -> Result<Self, CanError> {
        let mut msg = Self { data: [0u8; Self::LEN] };
        msg.set_contactor_to_send_boot(contactor_to_send_boot)?;
        Ok(msg)
    }
    ///ContactorToSendBoot
    ///- Min: 1
    ///- Max: 3
    ///- Unit:
    ///- Receivers: VectorXXX
    ///- Start bit: 0
    ///- Size: 8 bits
    ///- Factor: 1
    ///- Offset: 0
    ///- Byte order: LittleEndian
    ///- Type: unsigned
    pub fn contactor_to_send_boot(&self) -> ContactorToSendBootEnum {
        let raw_contactor_to_send_boot = self
            .data
            .view_bits::<Lsb0>()[0usize..8usize]
            .load_le::<u8>();
        ContactorToSendBootEnum::from(raw_contactor_to_send_boot as u8)
    }
    ///Set value of ContactorToSendBoot
    ///- Min: 1
    ///- Max: 3
    pub fn set_contactor_to_send_boot(
        &mut self,
        value: ContactorToSendBootEnum,
    ) -> Result<(), CanError> {
        self.data.view_bits_mut::<Lsb0>()[0usize..8usize].store_le(u8::from(value));
        Ok(())
    }
}
impl GeneratedCanMessage<{ Self::LEN }> for FooContactorBootRequestMsg {
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
///BarContactorBootRequest
///- ID: Standard 1280 (0x500)
///- Size: 8 bytes
///- Transmitter: Bcu
#[derive(Debug, Clone)]
pub struct BarContactorBootRequestMsg {
    data: [u8; 8usize],
}
impl BarContactorBootRequestMsg {
    pub const ID: Id = Id::Standard(unsafe { StandardId::new_unchecked(1280u16) });
    pub const LEN: usize = 8usize;
    pub fn new(
        contactor_to_send_boot: ContactorToSendBootEnum,
    ) -> Result<Self, CanError> {
        let mut msg = Self { data: [0u8; Self::LEN] };
        msg.set_contactor_to_send_boot(contactor_to_send_boot)?;
        Ok(msg)
    }
    ///ContactorToSendBoot
    ///- Min: 1
    ///- Max: 3
    ///- Unit:
    ///- Receivers: VectorXXX
    ///- Start bit: 0
    ///- Size: 8 bits
    ///- Factor: 1
    ///- Offset: 0
    ///- Byte order: LittleEndian
    ///- Type: unsigned
    pub fn contactor_to_send_boot(&self) -> ContactorToSendBootEnum {
        let raw_contactor_to_send_boot = self
            .data
            .view_bits::<Lsb0>()[0usize..8usize]
            .load_le::<u8>();
        ContactorToSendBootEnum::from(raw_contactor_to_send_boot as u8)
    }
    ///Set value of ContactorToSendBoot
    ///- Min: 1
    ///- Max: 3
    pub fn set_contactor_to_send_boot(
        &mut self,
        value: ContactorToSendBootEnum,
    ) -> Result<(), CanError> {
        self.data.view_bits_mut::<Lsb0>()[0usize..8usize].store_le(u8::from(value));
        Ok(())
    }
}
impl GeneratedCanMessage<{ Self::LEN }> for BarContactorBootRequestMsg {
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
///quuxMaintenanceControl2
///- ID: Standard 1873 (0x751)
///- Size: 8 bytes
///- Transmitter: Bcu
#[derive(Debug, Clone)]
pub struct QuuxMaintenanceControl2Msg {
    data: [u8; 8usize],
}
impl QuuxMaintenanceControl2Msg {
    pub const ID: Id = Id::Standard(unsafe { StandardId::new_unchecked(1873u16) });
    pub const LEN: usize = 8usize;
    pub fn new(
        liquid_chiller_pump_rpm: u16,
        lt_bar_pump_rpm: u16,
        lt_foo_pump_rpm: u16,
        bc_cac_pump_rpm: u16,
    ) -> Result<Self, CanError> {
        let mut msg = Self { data: [0u8; Self::LEN] };
        msg.set_liquid_chiller_pump_rpm(liquid_chiller_pump_rpm)?;
        msg.set_lt_bar_pump_rpm(lt_bar_pump_rpm)?;
        msg.set_lt_foo_pump_rpm(lt_foo_pump_rpm)?;
        msg.set_bc_cac_pump_rpm(bc_cac_pump_rpm)?;
        Ok(msg)
    }
    ///liquidChillerPumpRpm
    ///- Min: 1000
    ///- Max: 5500
    ///- Unit:
    ///- Receivers: VectorXXX
    ///- Start bit: 0
    ///- Size: 16 bits
    ///- Factor: 1
    ///- Offset: 0
    ///- Byte order: LittleEndian
    ///- Type: unsigned
    pub fn liquid_chiller_pump_rpm(&self) -> u16 {
        let raw_liquid_chiller_pump_rpm = self
            .data
            .view_bits::<Lsb0>()[0usize..16usize]
            .load_le::<u16>();
        (raw_liquid_chiller_pump_rpm) * (1u16) + (0u16)
    }
    ///ltBarPumpRpm
    ///- Min: 500
    ///- Max: 5000
    ///- Unit:
    ///- Receivers: VectorXXX
    ///- Start bit: 16
    ///- Size: 16 bits
    ///- Factor: 1
    ///- Offset: 0
    ///- Byte order: LittleEndian
    ///- Type: unsigned
    pub fn lt_bar_pump_rpm(&self) -> u16 {
        let raw_lt_bar_pump_rpm = self
            .data
            .view_bits::<Lsb0>()[16usize..32usize]
            .load_le::<u16>();
        (raw_lt_bar_pump_rpm) * (1u16) + (0u16)
    }
    ///ltFooPumpRpm
    ///- Min: 500
    ///- Max: 5000
    ///- Unit:
    ///- Receivers: VectorXXX
    ///- Start bit: 32
    ///- Size: 16 bits
    ///- Factor: 1
    ///- Offset: 0
    ///- Byte order: LittleEndian
    ///- Type: unsigned
    pub fn lt_foo_pump_rpm(&self) -> u16 {
        let raw_lt_foo_pump_rpm = self
            .data
            .view_bits::<Lsb0>()[32usize..48usize]
            .load_le::<u16>();
        (raw_lt_foo_pump_rpm) * (1u16) + (0u16)
    }
    ///bcCacPumpRpm
    ///- Min: 500
    ///- Max: 5000
    ///- Unit:
    ///- Receivers: VectorXXX
    ///- Start bit: 48
    ///- Size: 16 bits
    ///- Factor: 1
    ///- Offset: 0
    ///- Byte order: LittleEndian
    ///- Type: unsigned
    pub fn bc_cac_pump_rpm(&self) -> u16 {
        let raw_bc_cac_pump_rpm = self
            .data
            .view_bits::<Lsb0>()[48usize..64usize]
            .load_le::<u16>();
        (raw_bc_cac_pump_rpm) * (1u16) + (0u16)
    }
    ///Set value of liquidChillerPumpRpm
    ///- Min: 1000
    ///- Max: 5500
    pub fn set_liquid_chiller_pump_rpm(&mut self, value: u16) -> Result<(), CanError> {
        if value < 1000u16 || value > 5500u16 {
            return Err(CanError::ValueOutOfRange);
        }
        self.data
            .view_bits_mut::<Lsb0>()[0usize..16usize]
            .store_le((value - (0u16)) / (1u16));
        Ok(())
    }
    ///Set value of ltBarPumpRpm
    ///- Min: 500
    ///- Max: 5000
    pub fn set_lt_bar_pump_rpm(&mut self, value: u16) -> Result<(), CanError> {
        if value < 500u16 || value > 5000u16 {
            return Err(CanError::ValueOutOfRange);
        }
        self.data
            .view_bits_mut::<Lsb0>()[16usize..32usize]
            .store_le((value - (0u16)) / (1u16));
        Ok(())
    }
    ///Set value of ltFooPumpRpm
    ///- Min: 500
    ///- Max: 5000
    pub fn set_lt_foo_pump_rpm(&mut self, value: u16) -> Result<(), CanError> {
        if value < 500u16 || value > 5000u16 {
            return Err(CanError::ValueOutOfRange);
        }
        self.data
            .view_bits_mut::<Lsb0>()[32usize..48usize]
            .store_le((value - (0u16)) / (1u16));
        Ok(())
    }
    ///Set value of bcCacPumpRpm
    ///- Min: 500
    ///- Max: 5000
    pub fn set_bc_cac_pump_rpm(&mut self, value: u16) -> Result<(), CanError> {
        if value < 500u16 || value > 5000u16 {
            return Err(CanError::ValueOutOfRange);
        }
        self.data
            .view_bits_mut::<Lsb0>()[48usize..64usize]
            .store_le((value - (0u16)) / (1u16));
        Ok(())
    }
}
impl GeneratedCanMessage<{ Self::LEN }> for QuuxMaintenanceControl2Msg {
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
///quuxMaintenanceControl3
///- ID: Standard 1874 (0x752)
///- Size: 8 bytes
///- Transmitter: Bcu
#[derive(Debug, Clone)]
pub struct QuuxMaintenanceControl3Msg {
    data: [u8; 8usize],
}
impl QuuxMaintenanceControl3Msg {
    pub const ID: Id = Id::Standard(unsafe { StandardId::new_unchecked(1874u16) });
    pub const LEN: usize = 8usize;
    pub fn new(
        lt_dr_div_valve_direct_control_enable: bool,
        lt_foo_diverter_valve_control: bool,
        sens_valves_direct_control_enable: bool,
        sensorics_valves_control: bool,
        hpc_valve_direct_control_enable: bool,
        hpc_valve_control: bool,
        heater_direct_control_enable: bool,
        heater_on_control: bool,
        heater_temp_setpoint: i8,
    ) -> Result<Self, CanError> {
        let mut msg = Self { data: [0u8; Self::LEN] };
        msg.set_lt_dr_div_valve_direct_control_enable(
            lt_dr_div_valve_direct_control_enable,
        )?;
        msg.set_lt_foo_diverter_valve_control(lt_foo_diverter_valve_control)?;
        msg.set_sens_valves_direct_control_enable(sens_valves_direct_control_enable)?;
        msg.set_sensorics_valves_control(sensorics_valves_control)?;
        msg.set_hpc_valve_direct_control_enable(hpc_valve_direct_control_enable)?;
        msg.set_hpc_valve_control(hpc_valve_control)?;
        msg.set_heater_direct_control_enable(heater_direct_control_enable)?;
        msg.set_heater_on_control(heater_on_control)?;
        msg.set_heater_temp_setpoint(heater_temp_setpoint)?;
        Ok(msg)
    }
    ///ltDrDivValveDirectControlEnable
    ///- Min: 0
    ///- Max: 1
    ///- Unit:
    ///- Receivers: VectorXXX
    ///- Start bit: 0
    ///- Size: 1 bits
    ///- Factor: 1
    ///- Offset: 0
    ///- Byte order: LittleEndian
    ///- Type: unsigned
    pub fn lt_dr_div_valve_direct_control_enable(&self) -> bool {
        let raw_lt_dr_div_valve_direct_control_enable = self
            .data
            .view_bits::<Lsb0>()[0usize..1usize]
            .load_le::<u8>();
        raw_lt_dr_div_valve_direct_control_enable == 1
    }
    ///ltFooDiverterValveControl
    ///- Min: 0
    ///- Max: 1
    ///- Unit:
    ///- Receivers: VectorXXX
    ///- Start bit: 1
    ///- Size: 1 bits
    ///- Factor: 1
    ///- Offset: 0
    ///- Byte order: LittleEndian
    ///- Type: unsigned
    pub fn lt_foo_diverter_valve_control(&self) -> bool {
        let raw_lt_foo_diverter_valve_control = self
            .data
            .view_bits::<Lsb0>()[1usize..2usize]
            .load_le::<u8>();
        raw_lt_foo_diverter_valve_control == 1
    }
    ///sensValvesDirectControlEnable
    ///- Min: 0
    ///- Max: 1
    ///- Unit:
    ///- Receivers: VectorXXX
    ///- Start bit: 2
    ///- Size: 1 bits
    ///- Factor: 1
    ///- Offset: 0
    ///- Byte order: LittleEndian
    ///- Type: unsigned
    pub fn sens_valves_direct_control_enable(&self) -> bool {
        let raw_sens_valves_direct_control_enable = self
            .data
            .view_bits::<Lsb0>()[2usize..3usize]
            .load_le::<u8>();
        raw_sens_valves_direct_control_enable == 1
    }
    ///sensoricsValvesControl
    ///- Min: 0
    ///- Max: 1
    ///- Unit:
    ///- Receivers: VectorXXX
    ///- Start bit: 3
    ///- Size: 1 bits
    ///- Factor: 1
    ///- Offset: 0
    ///- Byte order: LittleEndian
    ///- Type: unsigned
    pub fn sensorics_valves_control(&self) -> bool {
        let raw_sensorics_valves_control = self
            .data
            .view_bits::<Lsb0>()[3usize..4usize]
            .load_le::<u8>();
        raw_sensorics_valves_control == 1
    }
    ///hpcValveDirectControlEnable
    ///- Min: 0
    ///- Max: 1
    ///- Unit:
    ///- Receivers: VectorXXX
    ///- Start bit: 4
    ///- Size: 1 bits
    ///- Factor: 1
    ///- Offset: 0
    ///- Byte order: LittleEndian
    ///- Type: unsigned
    pub fn hpc_valve_direct_control_enable(&self) -> bool {
        let raw_hpc_valve_direct_control_enable = self
            .data
            .view_bits::<Lsb0>()[4usize..5usize]
            .load_le::<u8>();
        raw_hpc_valve_direct_control_enable == 1
    }
    ///hpcValveControl
    ///- Min: 0
    ///- Max: 1
    ///- Unit:
    ///- Receivers: VectorXXX
    ///- Start bit: 5
    ///- Size: 1 bits
    ///- Factor: 1
    ///- Offset: 0
    ///- Byte order: LittleEndian
    ///- Type: unsigned
    pub fn hpc_valve_control(&self) -> bool {
        let raw_hpc_valve_control = self
            .data
            .view_bits::<Lsb0>()[5usize..6usize]
            .load_le::<u8>();
        raw_hpc_valve_control == 1
    }
    ///heaterDirectControlEnable
    ///- Min: 0
    ///- Max: 1
    ///- Unit:
    ///- Receivers: VectorXXX
    ///- Start bit: 6
    ///- Size: 1 bits
    ///- Factor: 1
    ///- Offset: 0
    ///- Byte order: LittleEndian
    ///- Type: unsigned
    pub fn heater_direct_control_enable(&self) -> bool {
        let raw_heater_direct_control_enable = self
            .data
            .view_bits::<Lsb0>()[6usize..7usize]
            .load_le::<u8>();
        raw_heater_direct_control_enable == 1
    }
    ///heaterOnControl
    ///- Min: 0
    ///- Max: 1
    ///- Unit:
    ///- Receivers: VectorXXX
    ///- Start bit: 7
    ///- Size: 1 bits
    ///- Factor: 1
    ///- Offset: 0
    ///- Byte order: LittleEndian
    ///- Type: unsigned
    pub fn heater_on_control(&self) -> bool {
        let raw_heater_on_control = self
            .data
            .view_bits::<Lsb0>()[7usize..8usize]
            .load_le::<u8>();
        raw_heater_on_control == 1
    }
    ///heaterTempSetpoint
    ///- Min: -39
    ///- Max: 80
    ///- Unit:
    ///- Receivers: VectorXXX
    ///- Start bit: 8
    ///- Size: 8 bits
    ///- Factor: 1
    ///- Offset: 0
    ///- Byte order: LittleEndian
    ///- Type: signed
    pub fn heater_temp_setpoint(&self) -> i8 {
        let raw_heater_temp_setpoint = self
            .data
            .view_bits::<Lsb0>()[8usize..16usize]
            .load_le::<u8>();
        (raw_heater_temp_setpoint as i8) * (1i8) + (0i8)
    }
    ///Set value of ltDrDivValveDirectControlEnable
    ///- Min: 0
    ///- Max: 1
    pub fn set_lt_dr_div_valve_direct_control_enable(
        &mut self,
        value: bool,
    ) -> Result<(), CanError> {
        self.data.view_bits_mut::<Lsb0>()[0usize..1usize].store_le(u8::from(value));
        Ok(())
    }
    ///Set value of ltFooDiverterValveControl
    ///- Min: 0
    ///- Max: 1
    pub fn set_lt_foo_diverter_valve_control(
        &mut self,
        value: bool,
    ) -> Result<(), CanError> {
        self.data.view_bits_mut::<Lsb0>()[1usize..2usize].store_le(u8::from(value));
        Ok(())
    }
    ///Set value of sensValvesDirectControlEnable
    ///- Min: 0
    ///- Max: 1
    pub fn set_sens_valves_direct_control_enable(
        &mut self,
        value: bool,
    ) -> Result<(), CanError> {
        self.data.view_bits_mut::<Lsb0>()[2usize..3usize].store_le(u8::from(value));
        Ok(())
    }
    ///Set value of sensoricsValvesControl
    ///- Min: 0
    ///- Max: 1
    pub fn set_sensorics_valves_control(&mut self, value: bool) -> Result<(), CanError> {
        self.data.view_bits_mut::<Lsb0>()[3usize..4usize].store_le(u8::from(value));
        Ok(())
    }
    ///Set value of hpcValveDirectControlEnable
    ///- Min: 0
    ///- Max: 1
    pub fn set_hpc_valve_direct_control_enable(
        &mut self,
        value: bool,
    ) -> Result<(), CanError> {
        self.data.view_bits_mut::<Lsb0>()[4usize..5usize].store_le(u8::from(value));
        Ok(())
    }
    ///Set value of hpcValveControl
    ///- Min: 0
    ///- Max: 1
    pub fn set_hpc_valve_control(&mut self, value: bool) -> Result<(), CanError> {
        self.data.view_bits_mut::<Lsb0>()[5usize..6usize].store_le(u8::from(value));
        Ok(())
    }
    ///Set value of heaterDirectControlEnable
    ///- Min: 0
    ///- Max: 1
    pub fn set_heater_direct_control_enable(
        &mut self,
        value: bool,
    ) -> Result<(), CanError> {
        self.data.view_bits_mut::<Lsb0>()[6usize..7usize].store_le(u8::from(value));
        Ok(())
    }
    ///Set value of heaterOnControl
    ///- Min: 0
    ///- Max: 1
    pub fn set_heater_on_control(&mut self, value: bool) -> Result<(), CanError> {
        self.data.view_bits_mut::<Lsb0>()[7usize..8usize].store_le(u8::from(value));
        Ok(())
    }
    ///Set value of heaterTempSetpoint
    ///- Min: -39
    ///- Max: 80
    pub fn set_heater_temp_setpoint(&mut self, value: i8) -> Result<(), CanError> {
        if value < -39i8 || value > 80i8 {
            return Err(CanError::ValueOutOfRange);
        }
        self.data
            .view_bits_mut::<Lsb0>()[8usize..16usize]
            .store_le((value - (0i8)) / (1i8));
        Ok(())
    }
}
impl GeneratedCanMessage<{ Self::LEN }> for QuuxMaintenanceControl3Msg {
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
    #[test]
    fn test_general_control_msg() {
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
        for seed in SEEDS {
            let mut u = Unstructured::new(seed);
            let vehicle_mode_control_value = VehicleModeControlEnum::Undefined;
            let payload_hv_control_value: bool = u
                .arbitrary()
                .expect("failed to generate bool");
            let dgs_control_value: bool = u
                .arbitrary()
                .expect("failed to generate bool");
            let clear_faults_value: bool = u
                .arbitrary()
                .expect("failed to generate bool");
            let faults_from_last_n_minutes_value: u8 = {
                let raw = u
                    .int_in_range(0u8..=255u8)
                    .expect("failed to generate physical interger value");
                raw
            };
            let diesel_preheater_control_value: bool = u
                .arbitrary()
                .expect("failed to generate bool");
            let high_charge_control_value: bool = u
                .arbitrary()
                .expect("failed to generate bool");
            let boost_control_value: bool = u
                .arbitrary()
                .expect("failed to generate bool");
            let park_brake_value: bool = u.arbitrary().expect("failed to generate bool");
            let foo_enable_value: bool = u.arbitrary().expect("failed to generate bool");
            let steering_mode_value = SteeringModeEnum::Normal;
            let differential_lock_value: bool = u
                .arbitrary()
                .expect("failed to generate bool");
            let direction_value = DirectionEnum::Undefined;
            let gear_value = GearEnum::Undefined;
            let load_value = LoadEnum::Empty;
            let terrain_value = TerrainEnum::Road;
            let ride_height_value = RideHeightEnum::Normal;
            let vehicle_mode_status_value = VehicleModeStatusEnum::Undefined;
            let differential_lock_longitude_value: bool = u
                .arbitrary()
                .expect("failed to generate bool");
            let park_brake_override_acknowledged_value: bool = u
                .arbitrary()
                .expect("failed to generate bool");
            let mut msg = GeneralControlMsg::new(
                    vehicle_mode_control_value,
                    payload_hv_control_value,
                    dgs_control_value,
                    clear_faults_value,
                    faults_from_last_n_minutes_value,
                    diesel_preheater_control_value,
                    high_charge_control_value,
                    boost_control_value,
                    park_brake_value,
                    foo_enable_value,
                    steering_mode_value,
                    differential_lock_value,
                    direction_value,
                    gear_value,
                    load_value,
                    terrain_value,
                    ride_height_value,
                    vehicle_mode_status_value,
                    differential_lock_longitude_value,
                    park_brake_override_acknowledged_value,
                )
                .expect("constructor should accept generated test values");
            assert_eq!(
                msg.vehicle_mode_control(), vehicle_mode_control_value,
                "getter `{}` returned unexpected value",
                stringify!(vehicle_mode_control),
            );
            assert_eq!(
                msg.payload_hv_control(), payload_hv_control_value,
                "getter `{}` returned unexpected value", stringify!(payload_hv_control),
            );
            assert_eq!(
                msg.dgs_control(), dgs_control_value,
                "getter `{}` returned unexpected value", stringify!(dgs_control),
            );
            assert_eq!(
                msg.clear_faults(), clear_faults_value,
                "getter `{}` returned unexpected value", stringify!(clear_faults),
            );
            assert_eq!(
                msg.faults_from_last_n_minutes(), faults_from_last_n_minutes_value,
                "getter `{}` returned unexpected value",
                stringify!(faults_from_last_n_minutes),
            );
            assert_eq!(
                msg.diesel_preheater_control(), diesel_preheater_control_value,
                "getter `{}` returned unexpected value",
                stringify!(diesel_preheater_control),
            );
            assert_eq!(
                msg.high_charge_control(), high_charge_control_value,
                "getter `{}` returned unexpected value", stringify!(high_charge_control),
            );
            assert_eq!(
                msg.boost_control(), boost_control_value,
                "getter `{}` returned unexpected value", stringify!(boost_control),
            );
            assert_eq!(
                msg.park_brake(), park_brake_value,
                "getter `{}` returned unexpected value", stringify!(park_brake),
            );
            assert_eq!(
                msg.foo_enable(), foo_enable_value,
                "getter `{}` returned unexpected value", stringify!(foo_enable),
            );
            assert_eq!(
                msg.steering_mode(), steering_mode_value,
                "getter `{}` returned unexpected value", stringify!(steering_mode),
            );
            assert_eq!(
                msg.differential_lock(), differential_lock_value,
                "getter `{}` returned unexpected value", stringify!(differential_lock),
            );
            assert_eq!(
                msg.direction(), direction_value,
                "getter `{}` returned unexpected value", stringify!(direction),
            );
            assert_eq!(
                msg.gear(), gear_value, "getter `{}` returned unexpected value",
                stringify!(gear),
            );
            assert_eq!(
                msg.load(), load_value, "getter `{}` returned unexpected value",
                stringify!(load),
            );
            assert_eq!(
                msg.terrain(), terrain_value, "getter `{}` returned unexpected value",
                stringify!(terrain),
            );
            assert_eq!(
                msg.ride_height(), ride_height_value,
                "getter `{}` returned unexpected value", stringify!(ride_height),
            );
            assert_eq!(
                msg.vehicle_mode_status(), vehicle_mode_status_value,
                "getter `{}` returned unexpected value", stringify!(vehicle_mode_status),
            );
            assert_eq!(
                msg.differential_lock_longitude(), differential_lock_longitude_value,
                "getter `{}` returned unexpected value",
                stringify!(differential_lock_longitude),
            );
            assert_eq!(
                msg.park_brake_override_acknowledged(),
                park_brake_override_acknowledged_value,
                "getter `{}` returned unexpected value",
                stringify!(park_brake_override_acknowledged),
            );
            let vehicle_mode_control_next_value = VehicleModeControlEnum::Undefined;
            let payload_hv_control_next_value: bool = u
                .arbitrary()
                .expect("failed to generate bool");
            let dgs_control_next_value: bool = u
                .arbitrary()
                .expect("failed to generate bool");
            let clear_faults_next_value: bool = u
                .arbitrary()
                .expect("failed to generate bool");
            let faults_from_last_n_minutes_next_value: u8 = {
                let raw = u
                    .int_in_range(0u8..=255u8)
                    .expect("failed to generate physical interger value");
                raw
            };
            let diesel_preheater_control_next_value: bool = u
                .arbitrary()
                .expect("failed to generate bool");
            let high_charge_control_next_value: bool = u
                .arbitrary()
                .expect("failed to generate bool");
            let boost_control_next_value: bool = u
                .arbitrary()
                .expect("failed to generate bool");
            let park_brake_next_value: bool = u
                .arbitrary()
                .expect("failed to generate bool");
            let foo_enable_next_value: bool = u
                .arbitrary()
                .expect("failed to generate bool");
            let steering_mode_next_value = SteeringModeEnum::Normal;
            let differential_lock_next_value: bool = u
                .arbitrary()
                .expect("failed to generate bool");
            let direction_next_value = DirectionEnum::Undefined;
            let gear_next_value = GearEnum::Undefined;
            let load_next_value = LoadEnum::Empty;
            let terrain_next_value = TerrainEnum::Road;
            let ride_height_next_value = RideHeightEnum::Normal;
            let vehicle_mode_status_next_value = VehicleModeStatusEnum::Undefined;
            let differential_lock_longitude_next_value: bool = u
                .arbitrary()
                .expect("failed to generate bool");
            let park_brake_override_acknowledged_next_value: bool = u
                .arbitrary()
                .expect("failed to generate bool");
            msg.set_vehicle_mode_control(vehicle_mode_control_next_value)
                .expect("setter should accept generated test value");
            msg.set_payload_hv_control(payload_hv_control_next_value)
                .expect("setter should accept generated test value");
            msg.set_dgs_control(dgs_control_next_value)
                .expect("setter should accept generated test value");
            msg.set_clear_faults(clear_faults_next_value)
                .expect("setter should accept generated test value");
            msg.set_faults_from_last_n_minutes(faults_from_last_n_minutes_next_value)
                .expect("setter should accept generated test value");
            msg.set_diesel_preheater_control(diesel_preheater_control_next_value)
                .expect("setter should accept generated test value");
            msg.set_high_charge_control(high_charge_control_next_value)
                .expect("setter should accept generated test value");
            msg.set_boost_control(boost_control_next_value)
                .expect("setter should accept generated test value");
            msg.set_park_brake(park_brake_next_value)
                .expect("setter should accept generated test value");
            msg.set_foo_enable(foo_enable_next_value)
                .expect("setter should accept generated test value");
            msg.set_steering_mode(steering_mode_next_value)
                .expect("setter should accept generated test value");
            msg.set_differential_lock(differential_lock_next_value)
                .expect("setter should accept generated test value");
            msg.set_direction(direction_next_value)
                .expect("setter should accept generated test value");
            msg.set_gear(gear_next_value)
                .expect("setter should accept generated test value");
            msg.set_load(load_next_value)
                .expect("setter should accept generated test value");
            msg.set_terrain(terrain_next_value)
                .expect("setter should accept generated test value");
            msg.set_ride_height(ride_height_next_value)
                .expect("setter should accept generated test value");
            msg.set_vehicle_mode_status(vehicle_mode_status_next_value)
                .expect("setter should accept generated test value");
            msg.set_differential_lock_longitude(differential_lock_longitude_next_value)
                .expect("setter should accept generated test value");
            msg.set_park_brake_override_acknowledged(
                    park_brake_override_acknowledged_next_value,
                )
                .expect("setter should accept generated test value");
            assert_eq!(
                msg.vehicle_mode_control(), vehicle_mode_control_next_value,
                "getter `{}` returned unexpected value",
                stringify!(vehicle_mode_control),
            );
            assert_eq!(
                msg.payload_hv_control(), payload_hv_control_next_value,
                "getter `{}` returned unexpected value", stringify!(payload_hv_control),
            );
            assert_eq!(
                msg.dgs_control(), dgs_control_next_value,
                "getter `{}` returned unexpected value", stringify!(dgs_control),
            );
            assert_eq!(
                msg.clear_faults(), clear_faults_next_value,
                "getter `{}` returned unexpected value", stringify!(clear_faults),
            );
            assert_eq!(
                msg.faults_from_last_n_minutes(), faults_from_last_n_minutes_next_value,
                "getter `{}` returned unexpected value",
                stringify!(faults_from_last_n_minutes),
            );
            assert_eq!(
                msg.diesel_preheater_control(), diesel_preheater_control_next_value,
                "getter `{}` returned unexpected value",
                stringify!(diesel_preheater_control),
            );
            assert_eq!(
                msg.high_charge_control(), high_charge_control_next_value,
                "getter `{}` returned unexpected value", stringify!(high_charge_control),
            );
            assert_eq!(
                msg.boost_control(), boost_control_next_value,
                "getter `{}` returned unexpected value", stringify!(boost_control),
            );
            assert_eq!(
                msg.park_brake(), park_brake_next_value,
                "getter `{}` returned unexpected value", stringify!(park_brake),
            );
            assert_eq!(
                msg.foo_enable(), foo_enable_next_value,
                "getter `{}` returned unexpected value", stringify!(foo_enable),
            );
            assert_eq!(
                msg.steering_mode(), steering_mode_next_value,
                "getter `{}` returned unexpected value", stringify!(steering_mode),
            );
            assert_eq!(
                msg.differential_lock(), differential_lock_next_value,
                "getter `{}` returned unexpected value", stringify!(differential_lock),
            );
            assert_eq!(
                msg.direction(), direction_next_value,
                "getter `{}` returned unexpected value", stringify!(direction),
            );
            assert_eq!(
                msg.gear(), gear_next_value, "getter `{}` returned unexpected value",
                stringify!(gear),
            );
            assert_eq!(
                msg.load(), load_next_value, "getter `{}` returned unexpected value",
                stringify!(load),
            );
            assert_eq!(
                msg.terrain(), terrain_next_value,
                "getter `{}` returned unexpected value", stringify!(terrain),
            );
            assert_eq!(
                msg.ride_height(), ride_height_next_value,
                "getter `{}` returned unexpected value", stringify!(ride_height),
            );
            assert_eq!(
                msg.vehicle_mode_status(), vehicle_mode_status_next_value,
                "getter `{}` returned unexpected value", stringify!(vehicle_mode_status),
            );
            assert_eq!(
                msg.differential_lock_longitude(),
                differential_lock_longitude_next_value,
                "getter `{}` returned unexpected value",
                stringify!(differential_lock_longitude),
            );
            assert_eq!(
                msg.park_brake_override_acknowledged(),
                park_brake_override_acknowledged_next_value,
                "getter `{}` returned unexpected value",
                stringify!(park_brake_override_acknowledged),
            );
        }
    }
    #[test]
    fn test_power_limits_msg() {
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
        for seed in SEEDS {
            let mut u = Unstructured::new(seed);
            let foo_available_power_value: f32 = {
                let mut selected: Option<f32> = None;
                loop {
                    let raw: u16 = u
                        .int_in_range(0..=65535)
                        .expect("failed to generate raw float-backed signal value");
                    let value: f32 = (raw as f32) * (0.1f32) + (0f32);
                    if value >= 0f32 && value <= 6553.5f32 {
                        selected = Some(value);
                        break;
                    }
                }
                selected.expect("could not generate encodable float signal value")
            };
            let foo_available_regen_value: f32 = {
                let mut selected: Option<f32> = None;
                loop {
                    let raw: u16 = u
                        .int_in_range(0..=65535)
                        .expect("failed to generate raw float-backed signal value");
                    let value: f32 = (raw as f32) * (0.1f32) + (0f32);
                    if value >= 0f32 && value <= 6553.5f32 {
                        selected = Some(value);
                        break;
                    }
                }
                selected.expect("could not generate encodable float signal value")
            };
            let quux_available_power_value: f32 = {
                let mut selected: Option<f32> = None;
                loop {
                    let raw: u16 = u
                        .int_in_range(0..=65535)
                        .expect("failed to generate raw float-backed signal value");
                    let value: f32 = (raw as f32) * (0.1f32) + (0f32);
                    if value >= 0f32 && value <= 6553.5f32 {
                        selected = Some(value);
                        break;
                    }
                }
                selected.expect("could not generate encodable float signal value")
            };
            let mut msg = PowerLimitsMsg::new(
                    foo_available_power_value,
                    foo_available_regen_value,
                    quux_available_power_value,
                )
                .expect("constructor should accept generated test values");
            let actual = msg.foo_available_power();
            let expected = foo_available_power_value;
            let tolerance = (expected.abs() * 0.0001).max(0.0001);
            assert!(
                (actual - expected).abs() <= tolerance,
                "getter `{}` returned {:?}, expected {:?}",
                stringify!(foo_available_power), actual, expected,
            );
            let actual = msg.foo_available_regen();
            let expected = foo_available_regen_value;
            let tolerance = (expected.abs() * 0.0001).max(0.0001);
            assert!(
                (actual - expected).abs() <= tolerance,
                "getter `{}` returned {:?}, expected {:?}",
                stringify!(foo_available_regen), actual, expected,
            );
            let actual = msg.quux_available_power();
            let expected = quux_available_power_value;
            let tolerance = (expected.abs() * 0.0001).max(0.0001);
            assert!(
                (actual - expected).abs() <= tolerance,
                "getter `{}` returned {:?}, expected {:?}",
                stringify!(quux_available_power), actual, expected,
            );
            let foo_available_power_next_value: f32 = {
                let mut selected: Option<f32> = None;
                loop {
                    let raw: u16 = u
                        .int_in_range(0..=65535)
                        .expect("failed to generate raw float-backed signal value");
                    let value: f32 = (raw as f32) * (0.1f32) + (0f32);
                    if value >= 0f32 && value <= 6553.5f32 {
                        selected = Some(value);
                        break;
                    }
                }
                selected.expect("could not generate encodable float signal value")
            };
            let foo_available_regen_next_value: f32 = {
                let mut selected: Option<f32> = None;
                loop {
                    let raw: u16 = u
                        .int_in_range(0..=65535)
                        .expect("failed to generate raw float-backed signal value");
                    let value: f32 = (raw as f32) * (0.1f32) + (0f32);
                    if value >= 0f32 && value <= 6553.5f32 {
                        selected = Some(value);
                        break;
                    }
                }
                selected.expect("could not generate encodable float signal value")
            };
            let quux_available_power_next_value: f32 = {
                let mut selected: Option<f32> = None;
                loop {
                    let raw: u16 = u
                        .int_in_range(0..=65535)
                        .expect("failed to generate raw float-backed signal value");
                    let value: f32 = (raw as f32) * (0.1f32) + (0f32);
                    if value >= 0f32 && value <= 6553.5f32 {
                        selected = Some(value);
                        break;
                    }
                }
                selected.expect("could not generate encodable float signal value")
            };
            msg.set_foo_available_power(foo_available_power_next_value)
                .expect("setter should accept generated test value");
            msg.set_foo_available_regen(foo_available_regen_next_value)
                .expect("setter should accept generated test value");
            msg.set_quux_available_power(quux_available_power_next_value)
                .expect("setter should accept generated test value");
            let actual = msg.foo_available_power();
            let expected = foo_available_power_next_value;
            let tolerance = (expected.abs() * 0.0001).max(0.0001);
            assert!(
                (actual - expected).abs() <= tolerance,
                "getter `{}` returned {:?}, expected {:?}",
                stringify!(foo_available_power), actual, expected,
            );
            let actual = msg.foo_available_regen();
            let expected = foo_available_regen_next_value;
            let tolerance = (expected.abs() * 0.0001).max(0.0001);
            assert!(
                (actual - expected).abs() <= tolerance,
                "getter `{}` returned {:?}, expected {:?}",
                stringify!(foo_available_regen), actual, expected,
            );
            let actual = msg.quux_available_power();
            let expected = quux_available_power_next_value;
            let tolerance = (expected.abs() * 0.0001).max(0.0001);
            assert!(
                (actual - expected).abs() <= tolerance,
                "getter `{}` returned {:?}, expected {:?}",
                stringify!(quux_available_power), actual, expected,
            );
        }
    }
    #[test]
    fn test_quux_status_msg() {
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
        for seed in SEEDS {
            let mut u = Unstructured::new(seed);
            let quux_total_current_value: u8 = {
                let raw = u
                    .int_in_range(0u8..=0u8)
                    .expect("failed to generate physical interger value");
                raw
            };
            let vehicle_mode_value = VehicleModeEnum::Undefined;
            let mut msg = QuuxStatusMsg::new(
                    quux_total_current_value,
                    vehicle_mode_value,
                )
                .expect("constructor should accept generated test values");
            assert_eq!(
                msg.quux_total_current(), quux_total_current_value,
                "getter `{}` returned unexpected value", stringify!(quux_total_current),
            );
            assert_eq!(
                msg.vehicle_mode(), vehicle_mode_value,
                "getter `{}` returned unexpected value", stringify!(vehicle_mode),
            );
            let quux_total_current_next_value: u8 = {
                let raw = u
                    .int_in_range(0u8..=0u8)
                    .expect("failed to generate physical interger value");
                raw
            };
            let vehicle_mode_next_value = VehicleModeEnum::Undefined;
            msg.set_quux_total_current(quux_total_current_next_value)
                .expect("setter should accept generated test value");
            msg.set_vehicle_mode(vehicle_mode_next_value)
                .expect("setter should accept generated test value");
            assert_eq!(
                msg.quux_total_current(), quux_total_current_next_value,
                "getter `{}` returned unexpected value", stringify!(quux_total_current),
            );
            assert_eq!(
                msg.vehicle_mode(), vehicle_mode_next_value,
                "getter `{}` returned unexpected value", stringify!(vehicle_mode),
            );
        }
    }
    #[test]
    fn test_foo_status1_msg() {
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
        for seed in SEEDS {
            let mut u = Unstructured::new(seed);
            let foo_total_current_value: f32 = {
                let mut selected: Option<f32> = None;
                loop {
                    let raw: u16 = u
                        .int_in_range(0..=65535)
                        .expect("failed to generate raw float-backed signal value");
                    let value: f32 = (raw as f32) * (0.1f32) + (0f32);
                    if value >= -3276f32 && value <= 3276f32 {
                        selected = Some(value);
                        break;
                    }
                }
                selected.expect("could not generate encodable float signal value")
            };
            let vehicle_speed_value: f32 = {
                let mut selected: Option<f32> = None;
                loop {
                    let raw: u8 = u
                        .int_in_range(0..=255)
                        .expect("failed to generate raw float-backed signal value");
                    let value: f32 = (raw as f32) * (0.14f32) + (0f32);
                    if value >= 0f32 && value <= 35f32 {
                        selected = Some(value);
                        break;
                    }
                }
                selected.expect("could not generate encodable float signal value")
            };
            let roll_value: i8 = {
                let raw = u
                    .int_in_range(-128i8..=127i8)
                    .expect("failed to generate physical interger value");
                raw
            };
            let pitch_value: i8 = {
                let raw = u
                    .int_in_range(-128i8..=127i8)
                    .expect("failed to generate physical interger value");
                raw
            };
            let mut msg = FooStatus1Msg::new(
                    foo_total_current_value,
                    vehicle_speed_value,
                    roll_value,
                    pitch_value,
                )
                .expect("constructor should accept generated test values");
            let actual = msg.foo_total_current();
            let expected = foo_total_current_value;
            let tolerance = (expected.abs() * 0.0001).max(0.0001);
            assert!(
                (actual - expected).abs() <= tolerance,
                "getter `{}` returned {:?}, expected {:?}",
                stringify!(foo_total_current), actual, expected,
            );
            let actual = msg.vehicle_speed();
            let expected = vehicle_speed_value;
            let tolerance = (expected.abs() * 0.0001).max(0.0001);
            assert!(
                (actual - expected).abs() <= tolerance,
                "getter `{}` returned {:?}, expected {:?}", stringify!(vehicle_speed),
                actual, expected,
            );
            assert_eq!(
                msg.roll(), roll_value, "getter `{}` returned unexpected value",
                stringify!(roll),
            );
            assert_eq!(
                msg.pitch(), pitch_value, "getter `{}` returned unexpected value",
                stringify!(pitch),
            );
            let foo_total_current_next_value: f32 = {
                let mut selected: Option<f32> = None;
                loop {
                    let raw: u16 = u
                        .int_in_range(0..=65535)
                        .expect("failed to generate raw float-backed signal value");
                    let value: f32 = (raw as f32) * (0.1f32) + (0f32);
                    if value >= -3276f32 && value <= 3276f32 {
                        selected = Some(value);
                        break;
                    }
                }
                selected.expect("could not generate encodable float signal value")
            };
            let vehicle_speed_next_value: f32 = {
                let mut selected: Option<f32> = None;
                loop {
                    let raw: u8 = u
                        .int_in_range(0..=255)
                        .expect("failed to generate raw float-backed signal value");
                    let value: f32 = (raw as f32) * (0.14f32) + (0f32);
                    if value >= 0f32 && value <= 35f32 {
                        selected = Some(value);
                        break;
                    }
                }
                selected.expect("could not generate encodable float signal value")
            };
            let roll_next_value: i8 = {
                let raw = u
                    .int_in_range(-128i8..=127i8)
                    .expect("failed to generate physical interger value");
                raw
            };
            let pitch_next_value: i8 = {
                let raw = u
                    .int_in_range(-128i8..=127i8)
                    .expect("failed to generate physical interger value");
                raw
            };
            msg.set_foo_total_current(foo_total_current_next_value)
                .expect("setter should accept generated test value");
            msg.set_vehicle_speed(vehicle_speed_next_value)
                .expect("setter should accept generated test value");
            msg.set_roll(roll_next_value)
                .expect("setter should accept generated test value");
            msg.set_pitch(pitch_next_value)
                .expect("setter should accept generated test value");
            let actual = msg.foo_total_current();
            let expected = foo_total_current_next_value;
            let tolerance = (expected.abs() * 0.0001).max(0.0001);
            assert!(
                (actual - expected).abs() <= tolerance,
                "getter `{}` returned {:?}, expected {:?}",
                stringify!(foo_total_current), actual, expected,
            );
            let actual = msg.vehicle_speed();
            let expected = vehicle_speed_next_value;
            let tolerance = (expected.abs() * 0.0001).max(0.0001);
            assert!(
                (actual - expected).abs() <= tolerance,
                "getter `{}` returned {:?}, expected {:?}", stringify!(vehicle_speed),
                actual, expected,
            );
            assert_eq!(
                msg.roll(), roll_next_value, "getter `{}` returned unexpected value",
                stringify!(roll),
            );
            assert_eq!(
                msg.pitch(), pitch_next_value, "getter `{}` returned unexpected value",
                stringify!(pitch),
            );
        }
    }
    #[test]
    fn test_foo_status2_msg() {
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
        for seed in SEEDS {
            let mut u = Unstructured::new(seed);
            let max_inverter_temp_value: u8 = {
                let raw = u
                    .int_in_range(216u8..=215u8)
                    .expect("failed to generate physical interger value");
                raw
            };
            let max_motor_temp_value: u8 = {
                let raw = u
                    .int_in_range(216u8..=215u8)
                    .expect("failed to generate physical interger value");
                raw
            };
            let compressor_temp_value: u8 = {
                let raw = u
                    .int_in_range(216u8..=215u8)
                    .expect("failed to generate physical interger value");
                raw
            };
            let vehicle_mode_value = VehicleModeEnum::Undefined;
            let park_brake_override_detected_value: bool = u
                .arbitrary()
                .expect("failed to generate bool");
            let foo_sub_contactors_status_value: bool = u
                .arbitrary()
                .expect("failed to generate bool");
            let fooline_enabled_value: bool = u
                .arbitrary()
                .expect("failed to generate bool");
            let compressor_safed_value: bool = u
                .arbitrary()
                .expect("failed to generate bool");
            let mut msg = FooStatus2Msg::new(
                    max_inverter_temp_value,
                    max_motor_temp_value,
                    compressor_temp_value,
                    vehicle_mode_value,
                    park_brake_override_detected_value,
                    foo_sub_contactors_status_value,
                    fooline_enabled_value,
                    compressor_safed_value,
                )
                .expect("constructor should accept generated test values");
            assert_eq!(
                msg.max_inverter_temp(), max_inverter_temp_value,
                "getter `{}` returned unexpected value", stringify!(max_inverter_temp),
            );
            assert_eq!(
                msg.max_motor_temp(), max_motor_temp_value,
                "getter `{}` returned unexpected value", stringify!(max_motor_temp),
            );
            assert_eq!(
                msg.compressor_temp(), compressor_temp_value,
                "getter `{}` returned unexpected value", stringify!(compressor_temp),
            );
            assert_eq!(
                msg.vehicle_mode(), vehicle_mode_value,
                "getter `{}` returned unexpected value", stringify!(vehicle_mode),
            );
            assert_eq!(
                msg.park_brake_override_detected(), park_brake_override_detected_value,
                "getter `{}` returned unexpected value",
                stringify!(park_brake_override_detected),
            );
            assert_eq!(
                msg.foo_sub_contactors_status(), foo_sub_contactors_status_value,
                "getter `{}` returned unexpected value",
                stringify!(foo_sub_contactors_status),
            );
            assert_eq!(
                msg.fooline_enabled(), fooline_enabled_value,
                "getter `{}` returned unexpected value", stringify!(fooline_enabled),
            );
            assert_eq!(
                msg.compressor_safed(), compressor_safed_value,
                "getter `{}` returned unexpected value", stringify!(compressor_safed),
            );
            let max_inverter_temp_next_value: u8 = {
                let raw = u
                    .int_in_range(216u8..=215u8)
                    .expect("failed to generate physical interger value");
                raw
            };
            let max_motor_temp_next_value: u8 = {
                let raw = u
                    .int_in_range(216u8..=215u8)
                    .expect("failed to generate physical interger value");
                raw
            };
            let compressor_temp_next_value: u8 = {
                let raw = u
                    .int_in_range(216u8..=215u8)
                    .expect("failed to generate physical interger value");
                raw
            };
            let vehicle_mode_next_value = VehicleModeEnum::Undefined;
            let park_brake_override_detected_next_value: bool = u
                .arbitrary()
                .expect("failed to generate bool");
            let foo_sub_contactors_status_next_value: bool = u
                .arbitrary()
                .expect("failed to generate bool");
            let fooline_enabled_next_value: bool = u
                .arbitrary()
                .expect("failed to generate bool");
            let compressor_safed_next_value: bool = u
                .arbitrary()
                .expect("failed to generate bool");
            msg.set_max_inverter_temp(max_inverter_temp_next_value)
                .expect("setter should accept generated test value");
            msg.set_max_motor_temp(max_motor_temp_next_value)
                .expect("setter should accept generated test value");
            msg.set_compressor_temp(compressor_temp_next_value)
                .expect("setter should accept generated test value");
            msg.set_vehicle_mode(vehicle_mode_next_value)
                .expect("setter should accept generated test value");
            msg.set_park_brake_override_detected(park_brake_override_detected_next_value)
                .expect("setter should accept generated test value");
            msg.set_foo_sub_contactors_status(foo_sub_contactors_status_next_value)
                .expect("setter should accept generated test value");
            msg.set_fooline_enabled(fooline_enabled_next_value)
                .expect("setter should accept generated test value");
            msg.set_compressor_safed(compressor_safed_next_value)
                .expect("setter should accept generated test value");
            assert_eq!(
                msg.max_inverter_temp(), max_inverter_temp_next_value,
                "getter `{}` returned unexpected value", stringify!(max_inverter_temp),
            );
            assert_eq!(
                msg.max_motor_temp(), max_motor_temp_next_value,
                "getter `{}` returned unexpected value", stringify!(max_motor_temp),
            );
            assert_eq!(
                msg.compressor_temp(), compressor_temp_next_value,
                "getter `{}` returned unexpected value", stringify!(compressor_temp),
            );
            assert_eq!(
                msg.vehicle_mode(), vehicle_mode_next_value,
                "getter `{}` returned unexpected value", stringify!(vehicle_mode),
            );
            assert_eq!(
                msg.park_brake_override_detected(),
                park_brake_override_detected_next_value,
                "getter `{}` returned unexpected value",
                stringify!(park_brake_override_detected),
            );
            assert_eq!(
                msg.foo_sub_contactors_status(), foo_sub_contactors_status_next_value,
                "getter `{}` returned unexpected value",
                stringify!(foo_sub_contactors_status),
            );
            assert_eq!(
                msg.fooline_enabled(), fooline_enabled_next_value,
                "getter `{}` returned unexpected value", stringify!(fooline_enabled),
            );
            assert_eq!(
                msg.compressor_safed(), compressor_safed_next_value,
                "getter `{}` returned unexpected value", stringify!(compressor_safed),
            );
        }
    }
    #[test]
    fn test_baz_status_msg() {
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
        for seed in SEEDS {
            let mut u = Unstructured::new(seed);
            let hv_battery_contactor_status_value: bool = u
                .arbitrary()
                .expect("failed to generate bool");
            let hv_battery_voltage_value: f32 = {
                let mut selected: Option<f32> = None;
                loop {
                    let raw: u16 = u
                        .int_in_range(0..=65535)
                        .expect("failed to generate raw float-backed signal value");
                    let value: f32 = (raw as f32) * (0.02f32) + (0f32);
                    if value >= 0f32 && value <= 0f32 {
                        selected = Some(value);
                        break;
                    }
                }
                selected.expect("could not generate encodable float signal value")
            };
            let obc_type2_connected_value: bool = u
                .arbitrary()
                .expect("failed to generate bool");
            let dgs_enabled_value: bool = u
                .arbitrary()
                .expect("failed to generate bool");
            let vehicle_mode_value = VehicleModeEnum::Undefined;
            let hv_battery_ready_value: bool = u
                .arbitrary()
                .expect("failed to generate bool");
            let disable_ext_precharge_req_value: bool = u
                .arbitrary()
                .expect("failed to generate bool");
            let mut msg = BazStatusMsg::new(
                    hv_battery_contactor_status_value,
                    hv_battery_voltage_value,
                    obc_type2_connected_value,
                    dgs_enabled_value,
                    vehicle_mode_value,
                    hv_battery_ready_value,
                    disable_ext_precharge_req_value,
                )
                .expect("constructor should accept generated test values");
            assert_eq!(
                msg.hv_battery_contactor_status(), hv_battery_contactor_status_value,
                "getter `{}` returned unexpected value",
                stringify!(hv_battery_contactor_status),
            );
            let actual = msg.hv_battery_voltage();
            let expected = hv_battery_voltage_value;
            let tolerance = (expected.abs() * 0.0001).max(0.0001);
            assert!(
                (actual - expected).abs() <= tolerance,
                "getter `{}` returned {:?}, expected {:?}",
                stringify!(hv_battery_voltage), actual, expected,
            );
            assert_eq!(
                msg.obc_type2_connected(), obc_type2_connected_value,
                "getter `{}` returned unexpected value", stringify!(obc_type2_connected),
            );
            assert_eq!(
                msg.dgs_enabled(), dgs_enabled_value,
                "getter `{}` returned unexpected value", stringify!(dgs_enabled),
            );
            assert_eq!(
                msg.vehicle_mode(), vehicle_mode_value,
                "getter `{}` returned unexpected value", stringify!(vehicle_mode),
            );
            assert_eq!(
                msg.hv_battery_ready(), hv_battery_ready_value,
                "getter `{}` returned unexpected value", stringify!(hv_battery_ready),
            );
            assert_eq!(
                msg.disable_ext_precharge_req(), disable_ext_precharge_req_value,
                "getter `{}` returned unexpected value",
                stringify!(disable_ext_precharge_req),
            );
            let hv_battery_contactor_status_next_value: bool = u
                .arbitrary()
                .expect("failed to generate bool");
            let hv_battery_voltage_next_value: f32 = {
                let mut selected: Option<f32> = None;
                loop {
                    let raw: u16 = u
                        .int_in_range(0..=65535)
                        .expect("failed to generate raw float-backed signal value");
                    let value: f32 = (raw as f32) * (0.02f32) + (0f32);
                    if value >= 0f32 && value <= 0f32 {
                        selected = Some(value);
                        break;
                    }
                }
                selected.expect("could not generate encodable float signal value")
            };
            let obc_type2_connected_next_value: bool = u
                .arbitrary()
                .expect("failed to generate bool");
            let dgs_enabled_next_value: bool = u
                .arbitrary()
                .expect("failed to generate bool");
            let vehicle_mode_next_value = VehicleModeEnum::Undefined;
            let hv_battery_ready_next_value: bool = u
                .arbitrary()
                .expect("failed to generate bool");
            let disable_ext_precharge_req_next_value: bool = u
                .arbitrary()
                .expect("failed to generate bool");
            msg.set_hv_battery_contactor_status(hv_battery_contactor_status_next_value)
                .expect("setter should accept generated test value");
            msg.set_hv_battery_voltage(hv_battery_voltage_next_value)
                .expect("setter should accept generated test value");
            msg.set_obc_type2_connected(obc_type2_connected_next_value)
                .expect("setter should accept generated test value");
            msg.set_dgs_enabled(dgs_enabled_next_value)
                .expect("setter should accept generated test value");
            msg.set_vehicle_mode(vehicle_mode_next_value)
                .expect("setter should accept generated test value");
            msg.set_hv_battery_ready(hv_battery_ready_next_value)
                .expect("setter should accept generated test value");
            msg.set_disable_ext_precharge_req(disable_ext_precharge_req_next_value)
                .expect("setter should accept generated test value");
            assert_eq!(
                msg.hv_battery_contactor_status(),
                hv_battery_contactor_status_next_value,
                "getter `{}` returned unexpected value",
                stringify!(hv_battery_contactor_status),
            );
            let actual = msg.hv_battery_voltage();
            let expected = hv_battery_voltage_next_value;
            let tolerance = (expected.abs() * 0.0001).max(0.0001);
            assert!(
                (actual - expected).abs() <= tolerance,
                "getter `{}` returned {:?}, expected {:?}",
                stringify!(hv_battery_voltage), actual, expected,
            );
            assert_eq!(
                msg.obc_type2_connected(), obc_type2_connected_next_value,
                "getter `{}` returned unexpected value", stringify!(obc_type2_connected),
            );
            assert_eq!(
                msg.dgs_enabled(), dgs_enabled_next_value,
                "getter `{}` returned unexpected value", stringify!(dgs_enabled),
            );
            assert_eq!(
                msg.vehicle_mode(), vehicle_mode_next_value,
                "getter `{}` returned unexpected value", stringify!(vehicle_mode),
            );
            assert_eq!(
                msg.hv_battery_ready(), hv_battery_ready_next_value,
                "getter `{}` returned unexpected value", stringify!(hv_battery_ready),
            );
            assert_eq!(
                msg.disable_ext_precharge_req(), disable_ext_precharge_req_next_value,
                "getter `{}` returned unexpected value",
                stringify!(disable_ext_precharge_req),
            );
        }
    }
    #[test]
    fn test_baz_temperatures1_msg() {
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
        for seed in SEEDS {
            let mut u = Unstructured::new(seed);
            let hv_battery_max_temp_value: u8 = {
                let raw = u
                    .int_in_range(206u8..=205u8)
                    .expect("failed to generate physical interger value");
                raw
            };
            let hv_min_battery_temp_value: u8 = {
                let raw = u
                    .int_in_range(206u8..=205u8)
                    .expect("failed to generate physical interger value");
                raw
            };
            let obc_buck_boost_temp_value: u8 = {
                let raw = u
                    .int_in_range(206u8..=205u8)
                    .expect("failed to generate physical interger value");
                raw
            };
            let obc_chassis_temp_value: u8 = {
                let raw = u
                    .int_in_range(206u8..=205u8)
                    .expect("failed to generate physical interger value");
                raw
            };
            let obc_inverter_temp_value: u8 = {
                let raw = u
                    .int_in_range(206u8..=205u8)
                    .expect("failed to generate physical interger value");
                raw
            };
            let bc_max_junction_temp_value: u8 = {
                let raw = u
                    .int_in_range(216u8..=210u8)
                    .expect("failed to generate physical interger value");
                raw
            };
            let obc_ambient_temp_value: u8 = {
                let raw = u
                    .int_in_range(206u8..=205u8)
                    .expect("failed to generate physical interger value");
                raw
            };
            let gen_ambient_temp_value: u8 = {
                let raw = u
                    .int_in_range(216u8..=215u8)
                    .expect("failed to generate physical interger value");
                raw
            };
            let mut msg = BazTemperatures1Msg::new(
                    hv_battery_max_temp_value,
                    hv_min_battery_temp_value,
                    obc_buck_boost_temp_value,
                    obc_chassis_temp_value,
                    obc_inverter_temp_value,
                    bc_max_junction_temp_value,
                    obc_ambient_temp_value,
                    gen_ambient_temp_value,
                )
                .expect("constructor should accept generated test values");
            assert_eq!(
                msg.hv_battery_max_temp(), hv_battery_max_temp_value,
                "getter `{}` returned unexpected value", stringify!(hv_battery_max_temp),
            );
            assert_eq!(
                msg.hv_min_battery_temp(), hv_min_battery_temp_value,
                "getter `{}` returned unexpected value", stringify!(hv_min_battery_temp),
            );
            assert_eq!(
                msg.obc_buck_boost_temp(), obc_buck_boost_temp_value,
                "getter `{}` returned unexpected value", stringify!(obc_buck_boost_temp),
            );
            assert_eq!(
                msg.obc_chassis_temp(), obc_chassis_temp_value,
                "getter `{}` returned unexpected value", stringify!(obc_chassis_temp),
            );
            assert_eq!(
                msg.obc_inverter_temp(), obc_inverter_temp_value,
                "getter `{}` returned unexpected value", stringify!(obc_inverter_temp),
            );
            assert_eq!(
                msg.bc_max_junction_temp(), bc_max_junction_temp_value,
                "getter `{}` returned unexpected value",
                stringify!(bc_max_junction_temp),
            );
            assert_eq!(
                msg.obc_ambient_temp(), obc_ambient_temp_value,
                "getter `{}` returned unexpected value", stringify!(obc_ambient_temp),
            );
            assert_eq!(
                msg.gen_ambient_temp(), gen_ambient_temp_value,
                "getter `{}` returned unexpected value", stringify!(gen_ambient_temp),
            );
            let hv_battery_max_temp_next_value: u8 = {
                let raw = u
                    .int_in_range(206u8..=205u8)
                    .expect("failed to generate physical interger value");
                raw
            };
            let hv_min_battery_temp_next_value: u8 = {
                let raw = u
                    .int_in_range(206u8..=205u8)
                    .expect("failed to generate physical interger value");
                raw
            };
            let obc_buck_boost_temp_next_value: u8 = {
                let raw = u
                    .int_in_range(206u8..=205u8)
                    .expect("failed to generate physical interger value");
                raw
            };
            let obc_chassis_temp_next_value: u8 = {
                let raw = u
                    .int_in_range(206u8..=205u8)
                    .expect("failed to generate physical interger value");
                raw
            };
            let obc_inverter_temp_next_value: u8 = {
                let raw = u
                    .int_in_range(206u8..=205u8)
                    .expect("failed to generate physical interger value");
                raw
            };
            let bc_max_junction_temp_next_value: u8 = {
                let raw = u
                    .int_in_range(216u8..=210u8)
                    .expect("failed to generate physical interger value");
                raw
            };
            let obc_ambient_temp_next_value: u8 = {
                let raw = u
                    .int_in_range(206u8..=205u8)
                    .expect("failed to generate physical interger value");
                raw
            };
            let gen_ambient_temp_next_value: u8 = {
                let raw = u
                    .int_in_range(216u8..=215u8)
                    .expect("failed to generate physical interger value");
                raw
            };
            msg.set_hv_battery_max_temp(hv_battery_max_temp_next_value)
                .expect("setter should accept generated test value");
            msg.set_hv_min_battery_temp(hv_min_battery_temp_next_value)
                .expect("setter should accept generated test value");
            msg.set_obc_buck_boost_temp(obc_buck_boost_temp_next_value)
                .expect("setter should accept generated test value");
            msg.set_obc_chassis_temp(obc_chassis_temp_next_value)
                .expect("setter should accept generated test value");
            msg.set_obc_inverter_temp(obc_inverter_temp_next_value)
                .expect("setter should accept generated test value");
            msg.set_bc_max_junction_temp(bc_max_junction_temp_next_value)
                .expect("setter should accept generated test value");
            msg.set_obc_ambient_temp(obc_ambient_temp_next_value)
                .expect("setter should accept generated test value");
            msg.set_gen_ambient_temp(gen_ambient_temp_next_value)
                .expect("setter should accept generated test value");
            assert_eq!(
                msg.hv_battery_max_temp(), hv_battery_max_temp_next_value,
                "getter `{}` returned unexpected value", stringify!(hv_battery_max_temp),
            );
            assert_eq!(
                msg.hv_min_battery_temp(), hv_min_battery_temp_next_value,
                "getter `{}` returned unexpected value", stringify!(hv_min_battery_temp),
            );
            assert_eq!(
                msg.obc_buck_boost_temp(), obc_buck_boost_temp_next_value,
                "getter `{}` returned unexpected value", stringify!(obc_buck_boost_temp),
            );
            assert_eq!(
                msg.obc_chassis_temp(), obc_chassis_temp_next_value,
                "getter `{}` returned unexpected value", stringify!(obc_chassis_temp),
            );
            assert_eq!(
                msg.obc_inverter_temp(), obc_inverter_temp_next_value,
                "getter `{}` returned unexpected value", stringify!(obc_inverter_temp),
            );
            assert_eq!(
                msg.bc_max_junction_temp(), bc_max_junction_temp_next_value,
                "getter `{}` returned unexpected value",
                stringify!(bc_max_junction_temp),
            );
            assert_eq!(
                msg.obc_ambient_temp(), obc_ambient_temp_next_value,
                "getter `{}` returned unexpected value", stringify!(obc_ambient_temp),
            );
            assert_eq!(
                msg.gen_ambient_temp(), gen_ambient_temp_next_value,
                "getter `{}` returned unexpected value", stringify!(gen_ambient_temp),
            );
        }
    }
    #[test]
    fn test_baz_temperatures2_msg() {
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
        for seed in SEEDS {
            let mut u = Unstructured::new(seed);
            let engine_coolant_temp_value: u8 = {
                let raw = u
                    .int_in_range(216u8..=210u8)
                    .expect("failed to generate physical interger value");
                raw
            };
            let engine_fuel_temp_value: u8 = {
                let raw = u
                    .int_in_range(216u8..=210u8)
                    .expect("failed to generate physical interger value");
                raw
            };
            let intake_manifold_temp_value: u8 = {
                let raw = u
                    .int_in_range(216u8..=210u8)
                    .expect("failed to generate physical interger value");
                raw
            };
            let gen_motor_temp_value: u8 = {
                let raw = u
                    .int_in_range(216u8..=210u8)
                    .expect("failed to generate physical interger value");
                raw
            };
            let gen_emi_temp_value: u8 = {
                let raw = u
                    .int_in_range(216u8..=210u8)
                    .expect("failed to generate physical interger value");
                raw
            };
            let hv_battery_avg_temp_value: u8 = {
                let raw = u
                    .int_in_range(206u8..=205u8)
                    .expect("failed to generate physical interger value");
                raw
            };
            let mut msg = BazTemperatures2Msg::new(
                    engine_coolant_temp_value,
                    engine_fuel_temp_value,
                    intake_manifold_temp_value,
                    gen_motor_temp_value,
                    gen_emi_temp_value,
                    hv_battery_avg_temp_value,
                )
                .expect("constructor should accept generated test values");
            assert_eq!(
                msg.engine_coolant_temp(), engine_coolant_temp_value,
                "getter `{}` returned unexpected value", stringify!(engine_coolant_temp),
            );
            assert_eq!(
                msg.engine_fuel_temp(), engine_fuel_temp_value,
                "getter `{}` returned unexpected value", stringify!(engine_fuel_temp),
            );
            assert_eq!(
                msg.intake_manifold_temp(), intake_manifold_temp_value,
                "getter `{}` returned unexpected value",
                stringify!(intake_manifold_temp),
            );
            assert_eq!(
                msg.gen_motor_temp(), gen_motor_temp_value,
                "getter `{}` returned unexpected value", stringify!(gen_motor_temp),
            );
            assert_eq!(
                msg.gen_emi_temp(), gen_emi_temp_value,
                "getter `{}` returned unexpected value", stringify!(gen_emi_temp),
            );
            assert_eq!(
                msg.hv_battery_avg_temp(), hv_battery_avg_temp_value,
                "getter `{}` returned unexpected value", stringify!(hv_battery_avg_temp),
            );
            let engine_coolant_temp_next_value: u8 = {
                let raw = u
                    .int_in_range(216u8..=210u8)
                    .expect("failed to generate physical interger value");
                raw
            };
            let engine_fuel_temp_next_value: u8 = {
                let raw = u
                    .int_in_range(216u8..=210u8)
                    .expect("failed to generate physical interger value");
                raw
            };
            let intake_manifold_temp_next_value: u8 = {
                let raw = u
                    .int_in_range(216u8..=210u8)
                    .expect("failed to generate physical interger value");
                raw
            };
            let gen_motor_temp_next_value: u8 = {
                let raw = u
                    .int_in_range(216u8..=210u8)
                    .expect("failed to generate physical interger value");
                raw
            };
            let gen_emi_temp_next_value: u8 = {
                let raw = u
                    .int_in_range(216u8..=210u8)
                    .expect("failed to generate physical interger value");
                raw
            };
            let hv_battery_avg_temp_next_value: u8 = {
                let raw = u
                    .int_in_range(206u8..=205u8)
                    .expect("failed to generate physical interger value");
                raw
            };
            msg.set_engine_coolant_temp(engine_coolant_temp_next_value)
                .expect("setter should accept generated test value");
            msg.set_engine_fuel_temp(engine_fuel_temp_next_value)
                .expect("setter should accept generated test value");
            msg.set_intake_manifold_temp(intake_manifold_temp_next_value)
                .expect("setter should accept generated test value");
            msg.set_gen_motor_temp(gen_motor_temp_next_value)
                .expect("setter should accept generated test value");
            msg.set_gen_emi_temp(gen_emi_temp_next_value)
                .expect("setter should accept generated test value");
            msg.set_hv_battery_avg_temp(hv_battery_avg_temp_next_value)
                .expect("setter should accept generated test value");
            assert_eq!(
                msg.engine_coolant_temp(), engine_coolant_temp_next_value,
                "getter `{}` returned unexpected value", stringify!(engine_coolant_temp),
            );
            assert_eq!(
                msg.engine_fuel_temp(), engine_fuel_temp_next_value,
                "getter `{}` returned unexpected value", stringify!(engine_fuel_temp),
            );
            assert_eq!(
                msg.intake_manifold_temp(), intake_manifold_temp_next_value,
                "getter `{}` returned unexpected value",
                stringify!(intake_manifold_temp),
            );
            assert_eq!(
                msg.gen_motor_temp(), gen_motor_temp_next_value,
                "getter `{}` returned unexpected value", stringify!(gen_motor_temp),
            );
            assert_eq!(
                msg.gen_emi_temp(), gen_emi_temp_next_value,
                "getter `{}` returned unexpected value", stringify!(gen_emi_temp),
            );
            assert_eq!(
                msg.hv_battery_avg_temp(), hv_battery_avg_temp_next_value,
                "getter `{}` returned unexpected value", stringify!(hv_battery_avg_temp),
            );
        }
    }
    #[test]
    fn test_bar_control_and_status_msg() {
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
        for seed in SEEDS {
            let mut u = Unstructured::new(seed);
            let hv_battery_contactor_control_value: bool = u
                .arbitrary()
                .expect("failed to generate bool");
            let foo_sub_contactors_control_value: bool = u
                .arbitrary()
                .expect("failed to generate bool");
            let bc_discharge_control_value: bool = u
                .arbitrary()
                .expect("failed to generate bool");
            let hv_off_control_value: bool = u
                .arbitrary()
                .expect("failed to generate bool");
            let payload_hv_status_value: bool = u
                .arbitrary()
                .expect("failed to generate bool");
            let vehicle_mode_value = VehicleModeEnum::Undefined;
            let lv_on_status_value: bool = u
                .arbitrary()
                .expect("failed to generate bool");
            let mut msg = BarControlAndStatusMsg::new(
                    hv_battery_contactor_control_value,
                    foo_sub_contactors_control_value,
                    bc_discharge_control_value,
                    hv_off_control_value,
                    payload_hv_status_value,
                    vehicle_mode_value,
                    lv_on_status_value,
                )
                .expect("constructor should accept generated test values");
            assert_eq!(
                msg.hv_battery_contactor_control(), hv_battery_contactor_control_value,
                "getter `{}` returned unexpected value",
                stringify!(hv_battery_contactor_control),
            );
            assert_eq!(
                msg.foo_sub_contactors_control(), foo_sub_contactors_control_value,
                "getter `{}` returned unexpected value",
                stringify!(foo_sub_contactors_control),
            );
            assert_eq!(
                msg.bc_discharge_control(), bc_discharge_control_value,
                "getter `{}` returned unexpected value",
                stringify!(bc_discharge_control),
            );
            assert_eq!(
                msg.hv_off_control(), hv_off_control_value,
                "getter `{}` returned unexpected value", stringify!(hv_off_control),
            );
            assert_eq!(
                msg.payload_hv_status(), payload_hv_status_value,
                "getter `{}` returned unexpected value", stringify!(payload_hv_status),
            );
            assert_eq!(
                msg.vehicle_mode(), vehicle_mode_value,
                "getter `{}` returned unexpected value", stringify!(vehicle_mode),
            );
            assert_eq!(
                msg.lv_on_status(), lv_on_status_value,
                "getter `{}` returned unexpected value", stringify!(lv_on_status),
            );
            let hv_battery_contactor_control_next_value: bool = u
                .arbitrary()
                .expect("failed to generate bool");
            let foo_sub_contactors_control_next_value: bool = u
                .arbitrary()
                .expect("failed to generate bool");
            let bc_discharge_control_next_value: bool = u
                .arbitrary()
                .expect("failed to generate bool");
            let hv_off_control_next_value: bool = u
                .arbitrary()
                .expect("failed to generate bool");
            let payload_hv_status_next_value: bool = u
                .arbitrary()
                .expect("failed to generate bool");
            let vehicle_mode_next_value = VehicleModeEnum::Undefined;
            let lv_on_status_next_value: bool = u
                .arbitrary()
                .expect("failed to generate bool");
            msg.set_hv_battery_contactor_control(hv_battery_contactor_control_next_value)
                .expect("setter should accept generated test value");
            msg.set_foo_sub_contactors_control(foo_sub_contactors_control_next_value)
                .expect("setter should accept generated test value");
            msg.set_bc_discharge_control(bc_discharge_control_next_value)
                .expect("setter should accept generated test value");
            msg.set_hv_off_control(hv_off_control_next_value)
                .expect("setter should accept generated test value");
            msg.set_payload_hv_status(payload_hv_status_next_value)
                .expect("setter should accept generated test value");
            msg.set_vehicle_mode(vehicle_mode_next_value)
                .expect("setter should accept generated test value");
            msg.set_lv_on_status(lv_on_status_next_value)
                .expect("setter should accept generated test value");
            assert_eq!(
                msg.hv_battery_contactor_control(),
                hv_battery_contactor_control_next_value,
                "getter `{}` returned unexpected value",
                stringify!(hv_battery_contactor_control),
            );
            assert_eq!(
                msg.foo_sub_contactors_control(), foo_sub_contactors_control_next_value,
                "getter `{}` returned unexpected value",
                stringify!(foo_sub_contactors_control),
            );
            assert_eq!(
                msg.bc_discharge_control(), bc_discharge_control_next_value,
                "getter `{}` returned unexpected value",
                stringify!(bc_discharge_control),
            );
            assert_eq!(
                msg.hv_off_control(), hv_off_control_next_value,
                "getter `{}` returned unexpected value", stringify!(hv_off_control),
            );
            assert_eq!(
                msg.payload_hv_status(), payload_hv_status_next_value,
                "getter `{}` returned unexpected value", stringify!(payload_hv_status),
            );
            assert_eq!(
                msg.vehicle_mode(), vehicle_mode_next_value,
                "getter `{}` returned unexpected value", stringify!(vehicle_mode),
            );
            assert_eq!(
                msg.lv_on_status(), lv_on_status_next_value,
                "getter `{}` returned unexpected value", stringify!(lv_on_status),
            );
        }
    }
    #[test]
    fn test_foo_maintenance_control_msg() {
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
        for seed in SEEDS {
            let mut u = Unstructured::new(seed);
            let brake_pump_control_value: bool = u
                .arbitrary()
                .expect("failed to generate bool");
            let service_brake_axle1_prop_control_value: u8 = {
                let raw = u
                    .int_in_range(0u8..=100u8)
                    .expect("failed to generate physical interger value");
                raw
            };
            let service_brake_axle2_prop_control_value: u8 = {
                let raw = u
                    .int_in_range(0u8..=100u8)
                    .expect("failed to generate physical interger value");
                raw
            };
            let service_brake_axle3_prop_control_value: u8 = {
                let raw = u
                    .int_in_range(0u8..=100u8)
                    .expect("failed to generate physical interger value");
                raw
            };
            let service_brake_axle4_prop_control_value: u8 = {
                let raw = u
                    .int_in_range(0u8..=100u8)
                    .expect("failed to generate physical interger value");
                raw
            };
            let park_tank_outflow_valve_control_value: bool = u
                .arbitrary()
                .expect("failed to generate bool");
            let axle1_tank_outflow_valve_control_value: bool = u
                .arbitrary()
                .expect("failed to generate bool");
            let axle2_tank_outflow_valve_control_value: bool = u
                .arbitrary()
                .expect("failed to generate bool");
            let axle3_tank_outflow_valve_control_value: bool = u
                .arbitrary()
                .expect("failed to generate bool");
            let axle4_tank_outflow_valve_control_value: bool = u
                .arbitrary()
                .expect("failed to generate bool");
            let left_steering_ignition_value = LeftSteeringIgnitionEnum::NoChange;
            let right_steering_ignition_value = RightSteeringIgnitionEnum::NoChange;
            let eaxle_ignition_value = EaxleIgnitionEnum::NoChange;
            let pneumatics_enabled_value: bool = u
                .arbitrary()
                .expect("failed to generate bool");
            let mut msg = FooMaintenanceControlMsg::new(
                    brake_pump_control_value,
                    service_brake_axle1_prop_control_value,
                    service_brake_axle2_prop_control_value,
                    service_brake_axle3_prop_control_value,
                    service_brake_axle4_prop_control_value,
                    park_tank_outflow_valve_control_value,
                    axle1_tank_outflow_valve_control_value,
                    axle2_tank_outflow_valve_control_value,
                    axle3_tank_outflow_valve_control_value,
                    axle4_tank_outflow_valve_control_value,
                    left_steering_ignition_value,
                    right_steering_ignition_value,
                    eaxle_ignition_value,
                    pneumatics_enabled_value,
                )
                .expect("constructor should accept generated test values");
            assert_eq!(
                msg.brake_pump_control(), brake_pump_control_value,
                "getter `{}` returned unexpected value", stringify!(brake_pump_control),
            );
            assert_eq!(
                msg.service_brake_axle1_prop_control(),
                service_brake_axle1_prop_control_value,
                "getter `{}` returned unexpected value",
                stringify!(service_brake_axle1_prop_control),
            );
            assert_eq!(
                msg.service_brake_axle2_prop_control(),
                service_brake_axle2_prop_control_value,
                "getter `{}` returned unexpected value",
                stringify!(service_brake_axle2_prop_control),
            );
            assert_eq!(
                msg.service_brake_axle3_prop_control(),
                service_brake_axle3_prop_control_value,
                "getter `{}` returned unexpected value",
                stringify!(service_brake_axle3_prop_control),
            );
            assert_eq!(
                msg.service_brake_axle4_prop_control(),
                service_brake_axle4_prop_control_value,
                "getter `{}` returned unexpected value",
                stringify!(service_brake_axle4_prop_control),
            );
            assert_eq!(
                msg.park_tank_outflow_valve_control(),
                park_tank_outflow_valve_control_value,
                "getter `{}` returned unexpected value",
                stringify!(park_tank_outflow_valve_control),
            );
            assert_eq!(
                msg.axle1_tank_outflow_valve_control(),
                axle1_tank_outflow_valve_control_value,
                "getter `{}` returned unexpected value",
                stringify!(axle1_tank_outflow_valve_control),
            );
            assert_eq!(
                msg.axle2_tank_outflow_valve_control(),
                axle2_tank_outflow_valve_control_value,
                "getter `{}` returned unexpected value",
                stringify!(axle2_tank_outflow_valve_control),
            );
            assert_eq!(
                msg.axle3_tank_outflow_valve_control(),
                axle3_tank_outflow_valve_control_value,
                "getter `{}` returned unexpected value",
                stringify!(axle3_tank_outflow_valve_control),
            );
            assert_eq!(
                msg.axle4_tank_outflow_valve_control(),
                axle4_tank_outflow_valve_control_value,
                "getter `{}` returned unexpected value",
                stringify!(axle4_tank_outflow_valve_control),
            );
            assert_eq!(
                msg.left_steering_ignition(), left_steering_ignition_value,
                "getter `{}` returned unexpected value",
                stringify!(left_steering_ignition),
            );
            assert_eq!(
                msg.right_steering_ignition(), right_steering_ignition_value,
                "getter `{}` returned unexpected value",
                stringify!(right_steering_ignition),
            );
            assert_eq!(
                msg.eaxle_ignition(), eaxle_ignition_value,
                "getter `{}` returned unexpected value", stringify!(eaxle_ignition),
            );
            assert_eq!(
                msg.pneumatics_enabled(), pneumatics_enabled_value,
                "getter `{}` returned unexpected value", stringify!(pneumatics_enabled),
            );
            let brake_pump_control_next_value: bool = u
                .arbitrary()
                .expect("failed to generate bool");
            let service_brake_axle1_prop_control_next_value: u8 = {
                let raw = u
                    .int_in_range(0u8..=100u8)
                    .expect("failed to generate physical interger value");
                raw
            };
            let service_brake_axle2_prop_control_next_value: u8 = {
                let raw = u
                    .int_in_range(0u8..=100u8)
                    .expect("failed to generate physical interger value");
                raw
            };
            let service_brake_axle3_prop_control_next_value: u8 = {
                let raw = u
                    .int_in_range(0u8..=100u8)
                    .expect("failed to generate physical interger value");
                raw
            };
            let service_brake_axle4_prop_control_next_value: u8 = {
                let raw = u
                    .int_in_range(0u8..=100u8)
                    .expect("failed to generate physical interger value");
                raw
            };
            let park_tank_outflow_valve_control_next_value: bool = u
                .arbitrary()
                .expect("failed to generate bool");
            let axle1_tank_outflow_valve_control_next_value: bool = u
                .arbitrary()
                .expect("failed to generate bool");
            let axle2_tank_outflow_valve_control_next_value: bool = u
                .arbitrary()
                .expect("failed to generate bool");
            let axle3_tank_outflow_valve_control_next_value: bool = u
                .arbitrary()
                .expect("failed to generate bool");
            let axle4_tank_outflow_valve_control_next_value: bool = u
                .arbitrary()
                .expect("failed to generate bool");
            let left_steering_ignition_next_value = LeftSteeringIgnitionEnum::NoChange;
            let right_steering_ignition_next_value = RightSteeringIgnitionEnum::NoChange;
            let eaxle_ignition_next_value = EaxleIgnitionEnum::NoChange;
            let pneumatics_enabled_next_value: bool = u
                .arbitrary()
                .expect("failed to generate bool");
            msg.set_brake_pump_control(brake_pump_control_next_value)
                .expect("setter should accept generated test value");
            msg.set_service_brake_axle1_prop_control(
                    service_brake_axle1_prop_control_next_value,
                )
                .expect("setter should accept generated test value");
            msg.set_service_brake_axle2_prop_control(
                    service_brake_axle2_prop_control_next_value,
                )
                .expect("setter should accept generated test value");
            msg.set_service_brake_axle3_prop_control(
                    service_brake_axle3_prop_control_next_value,
                )
                .expect("setter should accept generated test value");
            msg.set_service_brake_axle4_prop_control(
                    service_brake_axle4_prop_control_next_value,
                )
                .expect("setter should accept generated test value");
            msg.set_park_tank_outflow_valve_control(
                    park_tank_outflow_valve_control_next_value,
                )
                .expect("setter should accept generated test value");
            msg.set_axle1_tank_outflow_valve_control(
                    axle1_tank_outflow_valve_control_next_value,
                )
                .expect("setter should accept generated test value");
            msg.set_axle2_tank_outflow_valve_control(
                    axle2_tank_outflow_valve_control_next_value,
                )
                .expect("setter should accept generated test value");
            msg.set_axle3_tank_outflow_valve_control(
                    axle3_tank_outflow_valve_control_next_value,
                )
                .expect("setter should accept generated test value");
            msg.set_axle4_tank_outflow_valve_control(
                    axle4_tank_outflow_valve_control_next_value,
                )
                .expect("setter should accept generated test value");
            msg.set_left_steering_ignition(left_steering_ignition_next_value)
                .expect("setter should accept generated test value");
            msg.set_right_steering_ignition(right_steering_ignition_next_value)
                .expect("setter should accept generated test value");
            msg.set_eaxle_ignition(eaxle_ignition_next_value)
                .expect("setter should accept generated test value");
            msg.set_pneumatics_enabled(pneumatics_enabled_next_value)
                .expect("setter should accept generated test value");
            assert_eq!(
                msg.brake_pump_control(), brake_pump_control_next_value,
                "getter `{}` returned unexpected value", stringify!(brake_pump_control),
            );
            assert_eq!(
                msg.service_brake_axle1_prop_control(),
                service_brake_axle1_prop_control_next_value,
                "getter `{}` returned unexpected value",
                stringify!(service_brake_axle1_prop_control),
            );
            assert_eq!(
                msg.service_brake_axle2_prop_control(),
                service_brake_axle2_prop_control_next_value,
                "getter `{}` returned unexpected value",
                stringify!(service_brake_axle2_prop_control),
            );
            assert_eq!(
                msg.service_brake_axle3_prop_control(),
                service_brake_axle3_prop_control_next_value,
                "getter `{}` returned unexpected value",
                stringify!(service_brake_axle3_prop_control),
            );
            assert_eq!(
                msg.service_brake_axle4_prop_control(),
                service_brake_axle4_prop_control_next_value,
                "getter `{}` returned unexpected value",
                stringify!(service_brake_axle4_prop_control),
            );
            assert_eq!(
                msg.park_tank_outflow_valve_control(),
                park_tank_outflow_valve_control_next_value,
                "getter `{}` returned unexpected value",
                stringify!(park_tank_outflow_valve_control),
            );
            assert_eq!(
                msg.axle1_tank_outflow_valve_control(),
                axle1_tank_outflow_valve_control_next_value,
                "getter `{}` returned unexpected value",
                stringify!(axle1_tank_outflow_valve_control),
            );
            assert_eq!(
                msg.axle2_tank_outflow_valve_control(),
                axle2_tank_outflow_valve_control_next_value,
                "getter `{}` returned unexpected value",
                stringify!(axle2_tank_outflow_valve_control),
            );
            assert_eq!(
                msg.axle3_tank_outflow_valve_control(),
                axle3_tank_outflow_valve_control_next_value,
                "getter `{}` returned unexpected value",
                stringify!(axle3_tank_outflow_valve_control),
            );
            assert_eq!(
                msg.axle4_tank_outflow_valve_control(),
                axle4_tank_outflow_valve_control_next_value,
                "getter `{}` returned unexpected value",
                stringify!(axle4_tank_outflow_valve_control),
            );
            assert_eq!(
                msg.left_steering_ignition(), left_steering_ignition_next_value,
                "getter `{}` returned unexpected value",
                stringify!(left_steering_ignition),
            );
            assert_eq!(
                msg.right_steering_ignition(), right_steering_ignition_next_value,
                "getter `{}` returned unexpected value",
                stringify!(right_steering_ignition),
            );
            assert_eq!(
                msg.eaxle_ignition(), eaxle_ignition_next_value,
                "getter `{}` returned unexpected value", stringify!(eaxle_ignition),
            );
            assert_eq!(
                msg.pneumatics_enabled(), pneumatics_enabled_next_value,
                "getter `{}` returned unexpected value", stringify!(pneumatics_enabled),
            );
        }
    }
    #[test]
    fn test_zero_all_msg() {
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
        for seed in SEEDS {
            let mut u = Unstructured::new(seed);
            let mut msg = ZeroAllMsg::new()
                .expect("constructor should accept generated test values");
        }
    }
    #[test]
    fn test_quux_time_sync_request_msg() {
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
        for seed in SEEDS {
            let mut u = Unstructured::new(seed);
            let timestamp_value: u32 = {
                let raw = u
                    .int_in_range(0u32..=4294967295u32)
                    .expect("failed to generate physical interger value");
                raw
            };
            let mut msg = QuuxTimeSyncRequestMsg::new(timestamp_value)
                .expect("constructor should accept generated test values");
            assert_eq!(
                msg.timestamp(), timestamp_value,
                "getter `{}` returned unexpected value", stringify!(timestamp),
            );
            let timestamp_next_value: u32 = {
                let raw = u
                    .int_in_range(0u32..=4294967295u32)
                    .expect("failed to generate physical interger value");
                raw
            };
            msg.set_timestamp(timestamp_next_value)
                .expect("setter should accept generated test value");
            assert_eq!(
                msg.timestamp(), timestamp_next_value,
                "getter `{}` returned unexpected value", stringify!(timestamp),
            );
        }
    }
    #[test]
    fn test_foo_time_sync_request_msg() {
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
        for seed in SEEDS {
            let mut u = Unstructured::new(seed);
            let timestamp_value: u32 = {
                let raw = u
                    .int_in_range(0u32..=4294967295u32)
                    .expect("failed to generate physical interger value");
                raw
            };
            let mut msg = FooTimeSyncRequestMsg::new(timestamp_value)
                .expect("constructor should accept generated test values");
            assert_eq!(
                msg.timestamp(), timestamp_value,
                "getter `{}` returned unexpected value", stringify!(timestamp),
            );
            let timestamp_next_value: u32 = {
                let raw = u
                    .int_in_range(0u32..=4294967295u32)
                    .expect("failed to generate physical interger value");
                raw
            };
            msg.set_timestamp(timestamp_next_value)
                .expect("setter should accept generated test value");
            assert_eq!(
                msg.timestamp(), timestamp_next_value,
                "getter `{}` returned unexpected value", stringify!(timestamp),
            );
        }
    }
    #[test]
    fn test_bar_time_sync_request_msg() {
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
        for seed in SEEDS {
            let mut u = Unstructured::new(seed);
            let timestamp_value: u32 = {
                let raw = u
                    .int_in_range(0u32..=4294967295u32)
                    .expect("failed to generate physical interger value");
                raw
            };
            let mut msg = BarTimeSyncRequestMsg::new(timestamp_value)
                .expect("constructor should accept generated test values");
            assert_eq!(
                msg.timestamp(), timestamp_value,
                "getter `{}` returned unexpected value", stringify!(timestamp),
            );
            let timestamp_next_value: u32 = {
                let raw = u
                    .int_in_range(0u32..=4294967295u32)
                    .expect("failed to generate physical interger value");
                raw
            };
            msg.set_timestamp(timestamp_next_value)
                .expect("setter should accept generated test value");
            assert_eq!(
                msg.timestamp(), timestamp_next_value,
                "getter `{}` returned unexpected value", stringify!(timestamp),
            );
        }
    }
    #[test]
    fn test_baz_time_sync_request_msg() {
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
        for seed in SEEDS {
            let mut u = Unstructured::new(seed);
            let timestamp_value: u32 = {
                let raw = u
                    .int_in_range(0u32..=4294967295u32)
                    .expect("failed to generate physical interger value");
                raw
            };
            let mut msg = BazTimeSyncRequestMsg::new(timestamp_value)
                .expect("constructor should accept generated test values");
            assert_eq!(
                msg.timestamp(), timestamp_value,
                "getter `{}` returned unexpected value", stringify!(timestamp),
            );
            let timestamp_next_value: u32 = {
                let raw = u
                    .int_in_range(0u32..=4294967295u32)
                    .expect("failed to generate physical interger value");
                raw
            };
            msg.set_timestamp(timestamp_next_value)
                .expect("setter should accept generated test value");
            assert_eq!(
                msg.timestamp(), timestamp_next_value,
                "getter `{}` returned unexpected value", stringify!(timestamp),
            );
        }
    }
    #[test]
    fn test_quux_time_sync_response_msg() {
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
        for seed in SEEDS {
            let mut u = Unstructured::new(seed);
            let time_diff_value: i32 = {
                let raw = u
                    .int_in_range(-2147483648i32..=2147483647i32)
                    .expect("failed to generate physical interger value");
                raw
            };
            let timestamp_value: u32 = {
                let raw = u
                    .int_in_range(0u32..=4294967295u32)
                    .expect("failed to generate physical interger value");
                raw
            };
            let mut msg = QuuxTimeSyncResponseMsg::new(time_diff_value, timestamp_value)
                .expect("constructor should accept generated test values");
            assert_eq!(
                msg.time_diff(), time_diff_value,
                "getter `{}` returned unexpected value", stringify!(time_diff),
            );
            assert_eq!(
                msg.timestamp(), timestamp_value,
                "getter `{}` returned unexpected value", stringify!(timestamp),
            );
            let time_diff_next_value: i32 = {
                let raw = u
                    .int_in_range(-2147483648i32..=2147483647i32)
                    .expect("failed to generate physical interger value");
                raw
            };
            let timestamp_next_value: u32 = {
                let raw = u
                    .int_in_range(0u32..=4294967295u32)
                    .expect("failed to generate physical interger value");
                raw
            };
            msg.set_time_diff(time_diff_next_value)
                .expect("setter should accept generated test value");
            msg.set_timestamp(timestamp_next_value)
                .expect("setter should accept generated test value");
            assert_eq!(
                msg.time_diff(), time_diff_next_value,
                "getter `{}` returned unexpected value", stringify!(time_diff),
            );
            assert_eq!(
                msg.timestamp(), timestamp_next_value,
                "getter `{}` returned unexpected value", stringify!(timestamp),
            );
        }
    }
    #[test]
    fn test_foo_time_sync_response_msg() {
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
        for seed in SEEDS {
            let mut u = Unstructured::new(seed);
            let time_diff_value: i32 = {
                let raw = u
                    .int_in_range(-2147483648i32..=2147483647i32)
                    .expect("failed to generate physical interger value");
                raw
            };
            let timestamp_value: u32 = {
                let raw = u
                    .int_in_range(0u32..=4294967295u32)
                    .expect("failed to generate physical interger value");
                raw
            };
            let mut msg = FooTimeSyncResponseMsg::new(time_diff_value, timestamp_value)
                .expect("constructor should accept generated test values");
            assert_eq!(
                msg.time_diff(), time_diff_value,
                "getter `{}` returned unexpected value", stringify!(time_diff),
            );
            assert_eq!(
                msg.timestamp(), timestamp_value,
                "getter `{}` returned unexpected value", stringify!(timestamp),
            );
            let time_diff_next_value: i32 = {
                let raw = u
                    .int_in_range(-2147483648i32..=2147483647i32)
                    .expect("failed to generate physical interger value");
                raw
            };
            let timestamp_next_value: u32 = {
                let raw = u
                    .int_in_range(0u32..=4294967295u32)
                    .expect("failed to generate physical interger value");
                raw
            };
            msg.set_time_diff(time_diff_next_value)
                .expect("setter should accept generated test value");
            msg.set_timestamp(timestamp_next_value)
                .expect("setter should accept generated test value");
            assert_eq!(
                msg.time_diff(), time_diff_next_value,
                "getter `{}` returned unexpected value", stringify!(time_diff),
            );
            assert_eq!(
                msg.timestamp(), timestamp_next_value,
                "getter `{}` returned unexpected value", stringify!(timestamp),
            );
        }
    }
    #[test]
    fn test_bar_time_sync_response_msg() {
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
        for seed in SEEDS {
            let mut u = Unstructured::new(seed);
            let time_diff_value: i32 = {
                let raw = u
                    .int_in_range(-2147483648i32..=2147483647i32)
                    .expect("failed to generate physical interger value");
                raw
            };
            let timestamp_value: u32 = {
                let raw = u
                    .int_in_range(0u32..=4294967295u32)
                    .expect("failed to generate physical interger value");
                raw
            };
            let mut msg = BarTimeSyncResponseMsg::new(time_diff_value, timestamp_value)
                .expect("constructor should accept generated test values");
            assert_eq!(
                msg.time_diff(), time_diff_value,
                "getter `{}` returned unexpected value", stringify!(time_diff),
            );
            assert_eq!(
                msg.timestamp(), timestamp_value,
                "getter `{}` returned unexpected value", stringify!(timestamp),
            );
            let time_diff_next_value: i32 = {
                let raw = u
                    .int_in_range(-2147483648i32..=2147483647i32)
                    .expect("failed to generate physical interger value");
                raw
            };
            let timestamp_next_value: u32 = {
                let raw = u
                    .int_in_range(0u32..=4294967295u32)
                    .expect("failed to generate physical interger value");
                raw
            };
            msg.set_time_diff(time_diff_next_value)
                .expect("setter should accept generated test value");
            msg.set_timestamp(timestamp_next_value)
                .expect("setter should accept generated test value");
            assert_eq!(
                msg.time_diff(), time_diff_next_value,
                "getter `{}` returned unexpected value", stringify!(time_diff),
            );
            assert_eq!(
                msg.timestamp(), timestamp_next_value,
                "getter `{}` returned unexpected value", stringify!(timestamp),
            );
        }
    }
    #[test]
    fn test_baz_time_sync_response_msg() {
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
        for seed in SEEDS {
            let mut u = Unstructured::new(seed);
            let time_diff_value: i32 = {
                let raw = u
                    .int_in_range(-2147483648i32..=2147483647i32)
                    .expect("failed to generate physical interger value");
                raw
            };
            let timestamp_value: u32 = {
                let raw = u
                    .int_in_range(0u32..=4294967295u32)
                    .expect("failed to generate physical interger value");
                raw
            };
            let mut msg = BazTimeSyncResponseMsg::new(time_diff_value, timestamp_value)
                .expect("constructor should accept generated test values");
            assert_eq!(
                msg.time_diff(), time_diff_value,
                "getter `{}` returned unexpected value", stringify!(time_diff),
            );
            assert_eq!(
                msg.timestamp(), timestamp_value,
                "getter `{}` returned unexpected value", stringify!(timestamp),
            );
            let time_diff_next_value: i32 = {
                let raw = u
                    .int_in_range(-2147483648i32..=2147483647i32)
                    .expect("failed to generate physical interger value");
                raw
            };
            let timestamp_next_value: u32 = {
                let raw = u
                    .int_in_range(0u32..=4294967295u32)
                    .expect("failed to generate physical interger value");
                raw
            };
            msg.set_time_diff(time_diff_next_value)
                .expect("setter should accept generated test value");
            msg.set_timestamp(timestamp_next_value)
                .expect("setter should accept generated test value");
            assert_eq!(
                msg.time_diff(), time_diff_next_value,
                "getter `{}` returned unexpected value", stringify!(time_diff),
            );
            assert_eq!(
                msg.timestamp(), timestamp_next_value,
                "getter `{}` returned unexpected value", stringify!(timestamp),
            );
        }
    }
    #[test]
    fn test_baz_ack_msg() {
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
        for seed in SEEDS {
            let mut u = Unstructured::new(seed);
            let hv_safe_to_disable_value: bool = u
                .arbitrary()
                .expect("failed to generate bool");
            let lv_safe_to_disable_value: bool = u
                .arbitrary()
                .expect("failed to generate bool");
            let mut msg = BazAckMsg::new(
                    hv_safe_to_disable_value,
                    lv_safe_to_disable_value,
                )
                .expect("constructor should accept generated test values");
            assert_eq!(
                msg.hv_safe_to_disable(), hv_safe_to_disable_value,
                "getter `{}` returned unexpected value", stringify!(hv_safe_to_disable),
            );
            assert_eq!(
                msg.lv_safe_to_disable(), lv_safe_to_disable_value,
                "getter `{}` returned unexpected value", stringify!(lv_safe_to_disable),
            );
            let hv_safe_to_disable_next_value: bool = u
                .arbitrary()
                .expect("failed to generate bool");
            let lv_safe_to_disable_next_value: bool = u
                .arbitrary()
                .expect("failed to generate bool");
            msg.set_hv_safe_to_disable(hv_safe_to_disable_next_value)
                .expect("setter should accept generated test value");
            msg.set_lv_safe_to_disable(lv_safe_to_disable_next_value)
                .expect("setter should accept generated test value");
            assert_eq!(
                msg.hv_safe_to_disable(), hv_safe_to_disable_next_value,
                "getter `{}` returned unexpected value", stringify!(hv_safe_to_disable),
            );
            assert_eq!(
                msg.lv_safe_to_disable(), lv_safe_to_disable_next_value,
                "getter `{}` returned unexpected value", stringify!(lv_safe_to_disable),
            );
        }
    }
    #[test]
    fn test_bcu_ack_msg() {
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
        for seed in SEEDS {
            let mut u = Unstructured::new(seed);
            let lv_safe_to_disable_value: bool = u
                .arbitrary()
                .expect("failed to generate bool");
            let mut msg = BcuAckMsg::new(lv_safe_to_disable_value)
                .expect("constructor should accept generated test values");
            assert_eq!(
                msg.lv_safe_to_disable(), lv_safe_to_disable_value,
                "getter `{}` returned unexpected value", stringify!(lv_safe_to_disable),
            );
            let lv_safe_to_disable_next_value: bool = u
                .arbitrary()
                .expect("failed to generate bool");
            msg.set_lv_safe_to_disable(lv_safe_to_disable_next_value)
                .expect("setter should accept generated test value");
            assert_eq!(
                msg.lv_safe_to_disable(), lv_safe_to_disable_next_value,
                "getter `{}` returned unexpected value", stringify!(lv_safe_to_disable),
            );
        }
    }
    #[test]
    fn test_foo_ack_msg() {
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
        for seed in SEEDS {
            let mut u = Unstructured::new(seed);
            let hv_safe_to_disable_value: bool = u
                .arbitrary()
                .expect("failed to generate bool");
            let lv_safe_to_disable_value: bool = u
                .arbitrary()
                .expect("failed to generate bool");
            let request_foo_hv_restart_value: bool = u
                .arbitrary()
                .expect("failed to generate bool");
            let mut msg = FooAckMsg::new(
                    hv_safe_to_disable_value,
                    lv_safe_to_disable_value,
                    request_foo_hv_restart_value,
                )
                .expect("constructor should accept generated test values");
            assert_eq!(
                msg.hv_safe_to_disable(), hv_safe_to_disable_value,
                "getter `{}` returned unexpected value", stringify!(hv_safe_to_disable),
            );
            assert_eq!(
                msg.lv_safe_to_disable(), lv_safe_to_disable_value,
                "getter `{}` returned unexpected value", stringify!(lv_safe_to_disable),
            );
            assert_eq!(
                msg.request_foo_hv_restart(), request_foo_hv_restart_value,
                "getter `{}` returned unexpected value",
                stringify!(request_foo_hv_restart),
            );
            let hv_safe_to_disable_next_value: bool = u
                .arbitrary()
                .expect("failed to generate bool");
            let lv_safe_to_disable_next_value: bool = u
                .arbitrary()
                .expect("failed to generate bool");
            let request_foo_hv_restart_next_value: bool = u
                .arbitrary()
                .expect("failed to generate bool");
            msg.set_hv_safe_to_disable(hv_safe_to_disable_next_value)
                .expect("setter should accept generated test value");
            msg.set_lv_safe_to_disable(lv_safe_to_disable_next_value)
                .expect("setter should accept generated test value");
            msg.set_request_foo_hv_restart(request_foo_hv_restart_next_value)
                .expect("setter should accept generated test value");
            assert_eq!(
                msg.hv_safe_to_disable(), hv_safe_to_disable_next_value,
                "getter `{}` returned unexpected value", stringify!(hv_safe_to_disable),
            );
            assert_eq!(
                msg.lv_safe_to_disable(), lv_safe_to_disable_next_value,
                "getter `{}` returned unexpected value", stringify!(lv_safe_to_disable),
            );
            assert_eq!(
                msg.request_foo_hv_restart(), request_foo_hv_restart_next_value,
                "getter `{}` returned unexpected value",
                stringify!(request_foo_hv_restart),
            );
        }
    }
    #[test]
    fn test_quux_ack_msg() {
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
        for seed in SEEDS {
            let mut u = Unstructured::new(seed);
            let hv_safe_to_disable_value: bool = u
                .arbitrary()
                .expect("failed to generate bool");
            let lv_safe_to_disable_value: bool = u
                .arbitrary()
                .expect("failed to generate bool");
            let mut msg = QuuxAckMsg::new(
                    hv_safe_to_disable_value,
                    lv_safe_to_disable_value,
                )
                .expect("constructor should accept generated test values");
            assert_eq!(
                msg.hv_safe_to_disable(), hv_safe_to_disable_value,
                "getter `{}` returned unexpected value", stringify!(hv_safe_to_disable),
            );
            assert_eq!(
                msg.lv_safe_to_disable(), lv_safe_to_disable_value,
                "getter `{}` returned unexpected value", stringify!(lv_safe_to_disable),
            );
            let hv_safe_to_disable_next_value: bool = u
                .arbitrary()
                .expect("failed to generate bool");
            let lv_safe_to_disable_next_value: bool = u
                .arbitrary()
                .expect("failed to generate bool");
            msg.set_hv_safe_to_disable(hv_safe_to_disable_next_value)
                .expect("setter should accept generated test value");
            msg.set_lv_safe_to_disable(lv_safe_to_disable_next_value)
                .expect("setter should accept generated test value");
            assert_eq!(
                msg.hv_safe_to_disable(), hv_safe_to_disable_next_value,
                "getter `{}` returned unexpected value", stringify!(hv_safe_to_disable),
            );
            assert_eq!(
                msg.lv_safe_to_disable(), lv_safe_to_disable_next_value,
                "getter `{}` returned unexpected value", stringify!(lv_safe_to_disable),
            );
        }
    }
    #[test]
    fn test_bar_maintenance_control_msg() {
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
        for seed in SEEDS {
            let mut u = Unstructured::new(seed);
            let hv_state_control_value = HvStateControlEnum::Off;
            let mut msg = BarMaintenanceControlMsg::new(hv_state_control_value)
                .expect("constructor should accept generated test values");
            assert_eq!(
                msg.hv_state_control(), hv_state_control_value,
                "getter `{}` returned unexpected value", stringify!(hv_state_control),
            );
            let hv_state_control_next_value = HvStateControlEnum::Off;
            msg.set_hv_state_control(hv_state_control_next_value)
                .expect("setter should accept generated test value");
            assert_eq!(
                msg.hv_state_control(), hv_state_control_next_value,
                "getter `{}` returned unexpected value", stringify!(hv_state_control),
            );
        }
    }
    #[test]
    fn test_baz_maintenance_control_msg() {
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
        for seed in SEEDS {
            let mut u = Unstructured::new(seed);
            let hv_battery_enable_value: bool = u
                .arbitrary()
                .expect("failed to generate bool");
            let bc_enable_value: bool = u.arbitrary().expect("failed to generate bool");
            let bc_power_reference_value: f32 = {
                let mut selected: Option<f32> = None;
                loop {
                    let raw: u16 = u
                        .int_in_range(0..=65535)
                        .expect("failed to generate raw float-backed signal value");
                    let value: f32 = (raw as f32) * (0.125f32) + (-4016f32);
                    if value >= -4016f32 && value <= 4016f32 {
                        selected = Some(value);
                        break;
                    }
                }
                selected.expect("could not generate encodable float signal value")
            };
            let obc_enable_value: bool = u.arbitrary().expect("failed to generate bool");
            let dgs_enable_value: bool = u.arbitrary().expect("failed to generate bool");
            let fans_enable_value: bool = u
                .arbitrary()
                .expect("failed to generate bool");
            let can45_supply_value: bool = u
                .arbitrary()
                .expect("failed to generate bool");
            let can45_ignition_value: bool = u
                .arbitrary()
                .expect("failed to generate bool");
            let current_sensor_supply_value: bool = u
                .arbitrary()
                .expect("failed to generate bool");
            let hv_battery_supply_value: bool = u
                .arbitrary()
                .expect("failed to generate bool");
            let hv_battery_ignition_value: bool = u
                .arbitrary()
                .expect("failed to generate bool");
            let obcquux_solenoid_control_value: bool = u
                .arbitrary()
                .expect("failed to generate bool");
            let obc_supply_value: bool = u.arbitrary().expect("failed to generate bool");
            let obc_ignition_value: bool = u
                .arbitrary()
                .expect("failed to generate bool");
            let bc_ignition_value: bool = u
                .arbitrary()
                .expect("failed to generate bool");
            let bc_supply_value: bool = u.arbitrary().expect("failed to generate bool");
            let feed_pump_ctrl_value: bool = u
                .arbitrary()
                .expect("failed to generate bool");
            let mut msg = BazMaintenanceControlMsg::new(
                    hv_battery_enable_value,
                    bc_enable_value,
                    bc_power_reference_value,
                    obc_enable_value,
                    dgs_enable_value,
                    fans_enable_value,
                    can45_supply_value,
                    can45_ignition_value,
                    current_sensor_supply_value,
                    hv_battery_supply_value,
                    hv_battery_ignition_value,
                    obcquux_solenoid_control_value,
                    obc_supply_value,
                    obc_ignition_value,
                    bc_ignition_value,
                    bc_supply_value,
                    feed_pump_ctrl_value,
                )
                .expect("constructor should accept generated test values");
            assert_eq!(
                msg.hv_battery_enable(), hv_battery_enable_value,
                "getter `{}` returned unexpected value", stringify!(hv_battery_enable),
            );
            assert_eq!(
                msg.bc_enable(), bc_enable_value,
                "getter `{}` returned unexpected value", stringify!(bc_enable),
            );
            let actual = msg.bc_power_reference();
            let expected = bc_power_reference_value;
            let tolerance = (expected.abs() * 0.0001).max(0.0001);
            assert!(
                (actual - expected).abs() <= tolerance,
                "getter `{}` returned {:?}, expected {:?}",
                stringify!(bc_power_reference), actual, expected,
            );
            assert_eq!(
                msg.obc_enable(), obc_enable_value,
                "getter `{}` returned unexpected value", stringify!(obc_enable),
            );
            assert_eq!(
                msg.dgs_enable(), dgs_enable_value,
                "getter `{}` returned unexpected value", stringify!(dgs_enable),
            );
            assert_eq!(
                msg.fans_enable(), fans_enable_value,
                "getter `{}` returned unexpected value", stringify!(fans_enable),
            );
            assert_eq!(
                msg.can45_supply(), can45_supply_value,
                "getter `{}` returned unexpected value", stringify!(can45_supply),
            );
            assert_eq!(
                msg.can45_ignition(), can45_ignition_value,
                "getter `{}` returned unexpected value", stringify!(can45_ignition),
            );
            assert_eq!(
                msg.current_sensor_supply(), current_sensor_supply_value,
                "getter `{}` returned unexpected value",
                stringify!(current_sensor_supply),
            );
            assert_eq!(
                msg.hv_battery_supply(), hv_battery_supply_value,
                "getter `{}` returned unexpected value", stringify!(hv_battery_supply),
            );
            assert_eq!(
                msg.hv_battery_ignition(), hv_battery_ignition_value,
                "getter `{}` returned unexpected value", stringify!(hv_battery_ignition),
            );
            assert_eq!(
                msg.obcquux_solenoid_control(), obcquux_solenoid_control_value,
                "getter `{}` returned unexpected value",
                stringify!(obcquux_solenoid_control),
            );
            assert_eq!(
                msg.obc_supply(), obc_supply_value,
                "getter `{}` returned unexpected value", stringify!(obc_supply),
            );
            assert_eq!(
                msg.obc_ignition(), obc_ignition_value,
                "getter `{}` returned unexpected value", stringify!(obc_ignition),
            );
            assert_eq!(
                msg.bc_ignition(), bc_ignition_value,
                "getter `{}` returned unexpected value", stringify!(bc_ignition),
            );
            assert_eq!(
                msg.bc_supply(), bc_supply_value,
                "getter `{}` returned unexpected value", stringify!(bc_supply),
            );
            assert_eq!(
                msg.feed_pump_ctrl(), feed_pump_ctrl_value,
                "getter `{}` returned unexpected value", stringify!(feed_pump_ctrl),
            );
            let hv_battery_enable_next_value: bool = u
                .arbitrary()
                .expect("failed to generate bool");
            let bc_enable_next_value: bool = u
                .arbitrary()
                .expect("failed to generate bool");
            let bc_power_reference_next_value: f32 = {
                let mut selected: Option<f32> = None;
                loop {
                    let raw: u16 = u
                        .int_in_range(0..=65535)
                        .expect("failed to generate raw float-backed signal value");
                    let value: f32 = (raw as f32) * (0.125f32) + (-4016f32);
                    if value >= -4016f32 && value <= 4016f32 {
                        selected = Some(value);
                        break;
                    }
                }
                selected.expect("could not generate encodable float signal value")
            };
            let obc_enable_next_value: bool = u
                .arbitrary()
                .expect("failed to generate bool");
            let dgs_enable_next_value: bool = u
                .arbitrary()
                .expect("failed to generate bool");
            let fans_enable_next_value: bool = u
                .arbitrary()
                .expect("failed to generate bool");
            let can45_supply_next_value: bool = u
                .arbitrary()
                .expect("failed to generate bool");
            let can45_ignition_next_value: bool = u
                .arbitrary()
                .expect("failed to generate bool");
            let current_sensor_supply_next_value: bool = u
                .arbitrary()
                .expect("failed to generate bool");
            let hv_battery_supply_next_value: bool = u
                .arbitrary()
                .expect("failed to generate bool");
            let hv_battery_ignition_next_value: bool = u
                .arbitrary()
                .expect("failed to generate bool");
            let obcquux_solenoid_control_next_value: bool = u
                .arbitrary()
                .expect("failed to generate bool");
            let obc_supply_next_value: bool = u
                .arbitrary()
                .expect("failed to generate bool");
            let obc_ignition_next_value: bool = u
                .arbitrary()
                .expect("failed to generate bool");
            let bc_ignition_next_value: bool = u
                .arbitrary()
                .expect("failed to generate bool");
            let bc_supply_next_value: bool = u
                .arbitrary()
                .expect("failed to generate bool");
            let feed_pump_ctrl_next_value: bool = u
                .arbitrary()
                .expect("failed to generate bool");
            msg.set_hv_battery_enable(hv_battery_enable_next_value)
                .expect("setter should accept generated test value");
            msg.set_bc_enable(bc_enable_next_value)
                .expect("setter should accept generated test value");
            msg.set_bc_power_reference(bc_power_reference_next_value)
                .expect("setter should accept generated test value");
            msg.set_obc_enable(obc_enable_next_value)
                .expect("setter should accept generated test value");
            msg.set_dgs_enable(dgs_enable_next_value)
                .expect("setter should accept generated test value");
            msg.set_fans_enable(fans_enable_next_value)
                .expect("setter should accept generated test value");
            msg.set_can45_supply(can45_supply_next_value)
                .expect("setter should accept generated test value");
            msg.set_can45_ignition(can45_ignition_next_value)
                .expect("setter should accept generated test value");
            msg.set_current_sensor_supply(current_sensor_supply_next_value)
                .expect("setter should accept generated test value");
            msg.set_hv_battery_supply(hv_battery_supply_next_value)
                .expect("setter should accept generated test value");
            msg.set_hv_battery_ignition(hv_battery_ignition_next_value)
                .expect("setter should accept generated test value");
            msg.set_obcquux_solenoid_control(obcquux_solenoid_control_next_value)
                .expect("setter should accept generated test value");
            msg.set_obc_supply(obc_supply_next_value)
                .expect("setter should accept generated test value");
            msg.set_obc_ignition(obc_ignition_next_value)
                .expect("setter should accept generated test value");
            msg.set_bc_ignition(bc_ignition_next_value)
                .expect("setter should accept generated test value");
            msg.set_bc_supply(bc_supply_next_value)
                .expect("setter should accept generated test value");
            msg.set_feed_pump_ctrl(feed_pump_ctrl_next_value)
                .expect("setter should accept generated test value");
            assert_eq!(
                msg.hv_battery_enable(), hv_battery_enable_next_value,
                "getter `{}` returned unexpected value", stringify!(hv_battery_enable),
            );
            assert_eq!(
                msg.bc_enable(), bc_enable_next_value,
                "getter `{}` returned unexpected value", stringify!(bc_enable),
            );
            let actual = msg.bc_power_reference();
            let expected = bc_power_reference_next_value;
            let tolerance = (expected.abs() * 0.0001).max(0.0001);
            assert!(
                (actual - expected).abs() <= tolerance,
                "getter `{}` returned {:?}, expected {:?}",
                stringify!(bc_power_reference), actual, expected,
            );
            assert_eq!(
                msg.obc_enable(), obc_enable_next_value,
                "getter `{}` returned unexpected value", stringify!(obc_enable),
            );
            assert_eq!(
                msg.dgs_enable(), dgs_enable_next_value,
                "getter `{}` returned unexpected value", stringify!(dgs_enable),
            );
            assert_eq!(
                msg.fans_enable(), fans_enable_next_value,
                "getter `{}` returned unexpected value", stringify!(fans_enable),
            );
            assert_eq!(
                msg.can45_supply(), can45_supply_next_value,
                "getter `{}` returned unexpected value", stringify!(can45_supply),
            );
            assert_eq!(
                msg.can45_ignition(), can45_ignition_next_value,
                "getter `{}` returned unexpected value", stringify!(can45_ignition),
            );
            assert_eq!(
                msg.current_sensor_supply(), current_sensor_supply_next_value,
                "getter `{}` returned unexpected value",
                stringify!(current_sensor_supply),
            );
            assert_eq!(
                msg.hv_battery_supply(), hv_battery_supply_next_value,
                "getter `{}` returned unexpected value", stringify!(hv_battery_supply),
            );
            assert_eq!(
                msg.hv_battery_ignition(), hv_battery_ignition_next_value,
                "getter `{}` returned unexpected value", stringify!(hv_battery_ignition),
            );
            assert_eq!(
                msg.obcquux_solenoid_control(), obcquux_solenoid_control_next_value,
                "getter `{}` returned unexpected value",
                stringify!(obcquux_solenoid_control),
            );
            assert_eq!(
                msg.obc_supply(), obc_supply_next_value,
                "getter `{}` returned unexpected value", stringify!(obc_supply),
            );
            assert_eq!(
                msg.obc_ignition(), obc_ignition_next_value,
                "getter `{}` returned unexpected value", stringify!(obc_ignition),
            );
            assert_eq!(
                msg.bc_ignition(), bc_ignition_next_value,
                "getter `{}` returned unexpected value", stringify!(bc_ignition),
            );
            assert_eq!(
                msg.bc_supply(), bc_supply_next_value,
                "getter `{}` returned unexpected value", stringify!(bc_supply),
            );
            assert_eq!(
                msg.feed_pump_ctrl(), feed_pump_ctrl_next_value,
                "getter `{}` returned unexpected value", stringify!(feed_pump_ctrl),
            );
        }
    }
    #[test]
    fn test_quux_maintenance_control1_msg() {
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
        for seed in SEEDS {
            let mut u = Unstructured::new(seed);
            let fan_speed_setpoint_value: f32 = {
                let mut selected: Option<f32> = None;
                loop {
                    let raw: u16 = u
                        .int_in_range(0..=65535)
                        .expect("failed to generate raw float-backed signal value");
                    let value: f32 = (raw as f32) * (0.5f32) + (-16064f32);
                    if value >= -16064f32 && value <= 16063.5f32 {
                        selected = Some(value);
                        break;
                    }
                }
                selected.expect("could not generate encodable float signal value")
            };
            let fan_direct_control_enable_value: bool = u
                .arbitrary()
                .expect("failed to generate bool");
            let ac_enable_value: bool = u.arbitrary().expect("failed to generate bool");
            let ac_reset_value: bool = u.arbitrary().expect("failed to generate bool");
            let ac_setpoint_value: f32 = {
                let mut selected: Option<f32> = None;
                loop {
                    let raw: u8 = u
                        .int_in_range(0..=255)
                        .expect("failed to generate raw float-backed signal value");
                    let value: f32 = (raw as f32) * (0.1f32) + (10f32);
                    if value >= 15f32 && value <= 35f32 {
                        selected = Some(value);
                        break;
                    }
                }
                selected.expect("could not generate encodable float signal value")
            };
            let chiller_enable_value: bool = u
                .arbitrary()
                .expect("failed to generate bool");
            let chiller_reset_value: bool = u
                .arbitrary()
                .expect("failed to generate bool");
            let chiller_setpoint_value: f32 = {
                let mut selected: Option<f32> = None;
                loop {
                    let raw: u8 = u
                        .int_in_range(0..=255)
                        .expect("failed to generate raw float-backed signal value");
                    let value: f32 = (raw as f32) * (0.1f32) + (10f32);
                    if value >= 15f32 && value <= 35f32 {
                        selected = Some(value);
                        break;
                    }
                }
                selected.expect("could not generate encodable float signal value")
            };
            let ac_direct_control_enable_value: bool = u
                .arbitrary()
                .expect("failed to generate bool");
            let chiller_direct_control_enable_value: bool = u
                .arbitrary()
                .expect("failed to generate bool");
            let ac_power_limit_value: u8 = {
                let raw = u
                    .int_in_range(0u8..=100u8)
                    .expect("failed to generate physical interger value");
                raw
            };
            let chiller_power_limit_value: u8 = {
                let raw = u
                    .int_in_range(0u8..=100u8)
                    .expect("failed to generate physical interger value");
                raw
            };
            let liquid_chiller_pump_direct_control_en_value: bool = u
                .arbitrary()
                .expect("failed to generate bool");
            let liquid_chiller_pump_on_value: bool = u
                .arbitrary()
                .expect("failed to generate bool");
            let lt_bar_pump_direct_control_ena_value: bool = u
                .arbitrary()
                .expect("failed to generate bool");
            let lt_bar_pump_on_value: bool = u
                .arbitrary()
                .expect("failed to generate bool");
            let lt_foo_pump_direct_control_enable_value: bool = u
                .arbitrary()
                .expect("failed to generate bool");
            let lt_foo_pump_on_value: bool = u
                .arbitrary()
                .expect("failed to generate bool");
            let bc_cac_pump_direct_control_enable_value: bool = u
                .arbitrary()
                .expect("failed to generate bool");
            let bc_cac_pump_on_value: bool = u
                .arbitrary()
                .expect("failed to generate bool");
            let mut msg = QuuxMaintenanceControl1Msg::new(
                    fan_speed_setpoint_value,
                    fan_direct_control_enable_value,
                    ac_enable_value,
                    ac_reset_value,
                    ac_setpoint_value,
                    chiller_enable_value,
                    chiller_reset_value,
                    chiller_setpoint_value,
                    ac_direct_control_enable_value,
                    chiller_direct_control_enable_value,
                    ac_power_limit_value,
                    chiller_power_limit_value,
                    liquid_chiller_pump_direct_control_en_value,
                    liquid_chiller_pump_on_value,
                    lt_bar_pump_direct_control_ena_value,
                    lt_bar_pump_on_value,
                    lt_foo_pump_direct_control_enable_value,
                    lt_foo_pump_on_value,
                    bc_cac_pump_direct_control_enable_value,
                    bc_cac_pump_on_value,
                )
                .expect("constructor should accept generated test values");
            let actual = msg.fan_speed_setpoint();
            let expected = fan_speed_setpoint_value;
            let tolerance = (expected.abs() * 0.0001).max(0.0001);
            assert!(
                (actual - expected).abs() <= tolerance,
                "getter `{}` returned {:?}, expected {:?}",
                stringify!(fan_speed_setpoint), actual, expected,
            );
            assert_eq!(
                msg.fan_direct_control_enable(), fan_direct_control_enable_value,
                "getter `{}` returned unexpected value",
                stringify!(fan_direct_control_enable),
            );
            assert_eq!(
                msg.ac_enable(), ac_enable_value,
                "getter `{}` returned unexpected value", stringify!(ac_enable),
            );
            assert_eq!(
                msg.ac_reset(), ac_reset_value, "getter `{}` returned unexpected value",
                stringify!(ac_reset),
            );
            let actual = msg.ac_setpoint();
            let expected = ac_setpoint_value;
            let tolerance = (expected.abs() * 0.0001).max(0.0001);
            assert!(
                (actual - expected).abs() <= tolerance,
                "getter `{}` returned {:?}, expected {:?}", stringify!(ac_setpoint),
                actual, expected,
            );
            assert_eq!(
                msg.chiller_enable(), chiller_enable_value,
                "getter `{}` returned unexpected value", stringify!(chiller_enable),
            );
            assert_eq!(
                msg.chiller_reset(), chiller_reset_value,
                "getter `{}` returned unexpected value", stringify!(chiller_reset),
            );
            let actual = msg.chiller_setpoint();
            let expected = chiller_setpoint_value;
            let tolerance = (expected.abs() * 0.0001).max(0.0001);
            assert!(
                (actual - expected).abs() <= tolerance,
                "getter `{}` returned {:?}, expected {:?}", stringify!(chiller_setpoint),
                actual, expected,
            );
            assert_eq!(
                msg.ac_direct_control_enable(), ac_direct_control_enable_value,
                "getter `{}` returned unexpected value",
                stringify!(ac_direct_control_enable),
            );
            assert_eq!(
                msg.chiller_direct_control_enable(), chiller_direct_control_enable_value,
                "getter `{}` returned unexpected value",
                stringify!(chiller_direct_control_enable),
            );
            assert_eq!(
                msg.ac_power_limit(), ac_power_limit_value,
                "getter `{}` returned unexpected value", stringify!(ac_power_limit),
            );
            assert_eq!(
                msg.chiller_power_limit(), chiller_power_limit_value,
                "getter `{}` returned unexpected value", stringify!(chiller_power_limit),
            );
            assert_eq!(
                msg.liquid_chiller_pump_direct_control_en(),
                liquid_chiller_pump_direct_control_en_value,
                "getter `{}` returned unexpected value",
                stringify!(liquid_chiller_pump_direct_control_en),
            );
            assert_eq!(
                msg.liquid_chiller_pump_on(), liquid_chiller_pump_on_value,
                "getter `{}` returned unexpected value",
                stringify!(liquid_chiller_pump_on),
            );
            assert_eq!(
                msg.lt_bar_pump_direct_control_ena(),
                lt_bar_pump_direct_control_ena_value,
                "getter `{}` returned unexpected value",
                stringify!(lt_bar_pump_direct_control_ena),
            );
            assert_eq!(
                msg.lt_bar_pump_on(), lt_bar_pump_on_value,
                "getter `{}` returned unexpected value", stringify!(lt_bar_pump_on),
            );
            assert_eq!(
                msg.lt_foo_pump_direct_control_enable(),
                lt_foo_pump_direct_control_enable_value,
                "getter `{}` returned unexpected value",
                stringify!(lt_foo_pump_direct_control_enable),
            );
            assert_eq!(
                msg.lt_foo_pump_on(), lt_foo_pump_on_value,
                "getter `{}` returned unexpected value", stringify!(lt_foo_pump_on),
            );
            assert_eq!(
                msg.bc_cac_pump_direct_control_enable(),
                bc_cac_pump_direct_control_enable_value,
                "getter `{}` returned unexpected value",
                stringify!(bc_cac_pump_direct_control_enable),
            );
            assert_eq!(
                msg.bc_cac_pump_on(), bc_cac_pump_on_value,
                "getter `{}` returned unexpected value", stringify!(bc_cac_pump_on),
            );
            let fan_speed_setpoint_next_value: f32 = {
                let mut selected: Option<f32> = None;
                loop {
                    let raw: u16 = u
                        .int_in_range(0..=65535)
                        .expect("failed to generate raw float-backed signal value");
                    let value: f32 = (raw as f32) * (0.5f32) + (-16064f32);
                    if value >= -16064f32 && value <= 16063.5f32 {
                        selected = Some(value);
                        break;
                    }
                }
                selected.expect("could not generate encodable float signal value")
            };
            let fan_direct_control_enable_next_value: bool = u
                .arbitrary()
                .expect("failed to generate bool");
            let ac_enable_next_value: bool = u
                .arbitrary()
                .expect("failed to generate bool");
            let ac_reset_next_value: bool = u
                .arbitrary()
                .expect("failed to generate bool");
            let ac_setpoint_next_value: f32 = {
                let mut selected: Option<f32> = None;
                loop {
                    let raw: u8 = u
                        .int_in_range(0..=255)
                        .expect("failed to generate raw float-backed signal value");
                    let value: f32 = (raw as f32) * (0.1f32) + (10f32);
                    if value >= 15f32 && value <= 35f32 {
                        selected = Some(value);
                        break;
                    }
                }
                selected.expect("could not generate encodable float signal value")
            };
            let chiller_enable_next_value: bool = u
                .arbitrary()
                .expect("failed to generate bool");
            let chiller_reset_next_value: bool = u
                .arbitrary()
                .expect("failed to generate bool");
            let chiller_setpoint_next_value: f32 = {
                let mut selected: Option<f32> = None;
                loop {
                    let raw: u8 = u
                        .int_in_range(0..=255)
                        .expect("failed to generate raw float-backed signal value");
                    let value: f32 = (raw as f32) * (0.1f32) + (10f32);
                    if value >= 15f32 && value <= 35f32 {
                        selected = Some(value);
                        break;
                    }
                }
                selected.expect("could not generate encodable float signal value")
            };
            let ac_direct_control_enable_next_value: bool = u
                .arbitrary()
                .expect("failed to generate bool");
            let chiller_direct_control_enable_next_value: bool = u
                .arbitrary()
                .expect("failed to generate bool");
            let ac_power_limit_next_value: u8 = {
                let raw = u
                    .int_in_range(0u8..=100u8)
                    .expect("failed to generate physical interger value");
                raw
            };
            let chiller_power_limit_next_value: u8 = {
                let raw = u
                    .int_in_range(0u8..=100u8)
                    .expect("failed to generate physical interger value");
                raw
            };
            let liquid_chiller_pump_direct_control_en_next_value: bool = u
                .arbitrary()
                .expect("failed to generate bool");
            let liquid_chiller_pump_on_next_value: bool = u
                .arbitrary()
                .expect("failed to generate bool");
            let lt_bar_pump_direct_control_ena_next_value: bool = u
                .arbitrary()
                .expect("failed to generate bool");
            let lt_bar_pump_on_next_value: bool = u
                .arbitrary()
                .expect("failed to generate bool");
            let lt_foo_pump_direct_control_enable_next_value: bool = u
                .arbitrary()
                .expect("failed to generate bool");
            let lt_foo_pump_on_next_value: bool = u
                .arbitrary()
                .expect("failed to generate bool");
            let bc_cac_pump_direct_control_enable_next_value: bool = u
                .arbitrary()
                .expect("failed to generate bool");
            let bc_cac_pump_on_next_value: bool = u
                .arbitrary()
                .expect("failed to generate bool");
            msg.set_fan_speed_setpoint(fan_speed_setpoint_next_value)
                .expect("setter should accept generated test value");
            msg.set_fan_direct_control_enable(fan_direct_control_enable_next_value)
                .expect("setter should accept generated test value");
            msg.set_ac_enable(ac_enable_next_value)
                .expect("setter should accept generated test value");
            msg.set_ac_reset(ac_reset_next_value)
                .expect("setter should accept generated test value");
            msg.set_ac_setpoint(ac_setpoint_next_value)
                .expect("setter should accept generated test value");
            msg.set_chiller_enable(chiller_enable_next_value)
                .expect("setter should accept generated test value");
            msg.set_chiller_reset(chiller_reset_next_value)
                .expect("setter should accept generated test value");
            msg.set_chiller_setpoint(chiller_setpoint_next_value)
                .expect("setter should accept generated test value");
            msg.set_ac_direct_control_enable(ac_direct_control_enable_next_value)
                .expect("setter should accept generated test value");
            msg.set_chiller_direct_control_enable(
                    chiller_direct_control_enable_next_value,
                )
                .expect("setter should accept generated test value");
            msg.set_ac_power_limit(ac_power_limit_next_value)
                .expect("setter should accept generated test value");
            msg.set_chiller_power_limit(chiller_power_limit_next_value)
                .expect("setter should accept generated test value");
            msg.set_liquid_chiller_pump_direct_control_en(
                    liquid_chiller_pump_direct_control_en_next_value,
                )
                .expect("setter should accept generated test value");
            msg.set_liquid_chiller_pump_on(liquid_chiller_pump_on_next_value)
                .expect("setter should accept generated test value");
            msg.set_lt_bar_pump_direct_control_ena(
                    lt_bar_pump_direct_control_ena_next_value,
                )
                .expect("setter should accept generated test value");
            msg.set_lt_bar_pump_on(lt_bar_pump_on_next_value)
                .expect("setter should accept generated test value");
            msg.set_lt_foo_pump_direct_control_enable(
                    lt_foo_pump_direct_control_enable_next_value,
                )
                .expect("setter should accept generated test value");
            msg.set_lt_foo_pump_on(lt_foo_pump_on_next_value)
                .expect("setter should accept generated test value");
            msg.set_bc_cac_pump_direct_control_enable(
                    bc_cac_pump_direct_control_enable_next_value,
                )
                .expect("setter should accept generated test value");
            msg.set_bc_cac_pump_on(bc_cac_pump_on_next_value)
                .expect("setter should accept generated test value");
            let actual = msg.fan_speed_setpoint();
            let expected = fan_speed_setpoint_next_value;
            let tolerance = (expected.abs() * 0.0001).max(0.0001);
            assert!(
                (actual - expected).abs() <= tolerance,
                "getter `{}` returned {:?}, expected {:?}",
                stringify!(fan_speed_setpoint), actual, expected,
            );
            assert_eq!(
                msg.fan_direct_control_enable(), fan_direct_control_enable_next_value,
                "getter `{}` returned unexpected value",
                stringify!(fan_direct_control_enable),
            );
            assert_eq!(
                msg.ac_enable(), ac_enable_next_value,
                "getter `{}` returned unexpected value", stringify!(ac_enable),
            );
            assert_eq!(
                msg.ac_reset(), ac_reset_next_value,
                "getter `{}` returned unexpected value", stringify!(ac_reset),
            );
            let actual = msg.ac_setpoint();
            let expected = ac_setpoint_next_value;
            let tolerance = (expected.abs() * 0.0001).max(0.0001);
            assert!(
                (actual - expected).abs() <= tolerance,
                "getter `{}` returned {:?}, expected {:?}", stringify!(ac_setpoint),
                actual, expected,
            );
            assert_eq!(
                msg.chiller_enable(), chiller_enable_next_value,
                "getter `{}` returned unexpected value", stringify!(chiller_enable),
            );
            assert_eq!(
                msg.chiller_reset(), chiller_reset_next_value,
                "getter `{}` returned unexpected value", stringify!(chiller_reset),
            );
            let actual = msg.chiller_setpoint();
            let expected = chiller_setpoint_next_value;
            let tolerance = (expected.abs() * 0.0001).max(0.0001);
            assert!(
                (actual - expected).abs() <= tolerance,
                "getter `{}` returned {:?}, expected {:?}", stringify!(chiller_setpoint),
                actual, expected,
            );
            assert_eq!(
                msg.ac_direct_control_enable(), ac_direct_control_enable_next_value,
                "getter `{}` returned unexpected value",
                stringify!(ac_direct_control_enable),
            );
            assert_eq!(
                msg.chiller_direct_control_enable(),
                chiller_direct_control_enable_next_value,
                "getter `{}` returned unexpected value",
                stringify!(chiller_direct_control_enable),
            );
            assert_eq!(
                msg.ac_power_limit(), ac_power_limit_next_value,
                "getter `{}` returned unexpected value", stringify!(ac_power_limit),
            );
            assert_eq!(
                msg.chiller_power_limit(), chiller_power_limit_next_value,
                "getter `{}` returned unexpected value", stringify!(chiller_power_limit),
            );
            assert_eq!(
                msg.liquid_chiller_pump_direct_control_en(),
                liquid_chiller_pump_direct_control_en_next_value,
                "getter `{}` returned unexpected value",
                stringify!(liquid_chiller_pump_direct_control_en),
            );
            assert_eq!(
                msg.liquid_chiller_pump_on(), liquid_chiller_pump_on_next_value,
                "getter `{}` returned unexpected value",
                stringify!(liquid_chiller_pump_on),
            );
            assert_eq!(
                msg.lt_bar_pump_direct_control_ena(),
                lt_bar_pump_direct_control_ena_next_value,
                "getter `{}` returned unexpected value",
                stringify!(lt_bar_pump_direct_control_ena),
            );
            assert_eq!(
                msg.lt_bar_pump_on(), lt_bar_pump_on_next_value,
                "getter `{}` returned unexpected value", stringify!(lt_bar_pump_on),
            );
            assert_eq!(
                msg.lt_foo_pump_direct_control_enable(),
                lt_foo_pump_direct_control_enable_next_value,
                "getter `{}` returned unexpected value",
                stringify!(lt_foo_pump_direct_control_enable),
            );
            assert_eq!(
                msg.lt_foo_pump_on(), lt_foo_pump_on_next_value,
                "getter `{}` returned unexpected value", stringify!(lt_foo_pump_on),
            );
            assert_eq!(
                msg.bc_cac_pump_direct_control_enable(),
                bc_cac_pump_direct_control_enable_next_value,
                "getter `{}` returned unexpected value",
                stringify!(bc_cac_pump_direct_control_enable),
            );
            assert_eq!(
                msg.bc_cac_pump_on(), bc_cac_pump_on_next_value,
                "getter `{}` returned unexpected value", stringify!(bc_cac_pump_on),
            );
        }
    }
    #[test]
    fn test_contactor_update_response_msg() {
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
        for seed in SEEDS {
            let mut u = Unstructured::new(seed);
            let mut msg = ContactorUpdateResponseMsg::new()
                .expect("constructor should accept generated test values");
        }
    }
    #[test]
    fn test_contactor_update_request_msg() {
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
        for seed in SEEDS {
            let mut u = Unstructured::new(seed);
            let mut msg = ContactorUpdateRequestMsg::new()
                .expect("constructor should accept generated test values");
        }
    }
    #[test]
    fn test_foo_contactor_boot_request_msg() {
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
        for seed in SEEDS {
            let mut u = Unstructured::new(seed);
            let contactor_to_send_boot_value = ContactorToSendBootEnum::FirstContactor;
            let mut msg = FooContactorBootRequestMsg::new(contactor_to_send_boot_value)
                .expect("constructor should accept generated test values");
            assert_eq!(
                msg.contactor_to_send_boot(), contactor_to_send_boot_value,
                "getter `{}` returned unexpected value",
                stringify!(contactor_to_send_boot),
            );
            let contactor_to_send_boot_next_value = ContactorToSendBootEnum::FirstContactor;
            msg.set_contactor_to_send_boot(contactor_to_send_boot_next_value)
                .expect("setter should accept generated test value");
            assert_eq!(
                msg.contactor_to_send_boot(), contactor_to_send_boot_next_value,
                "getter `{}` returned unexpected value",
                stringify!(contactor_to_send_boot),
            );
        }
    }
    #[test]
    fn test_bar_contactor_boot_request_msg() {
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
        for seed in SEEDS {
            let mut u = Unstructured::new(seed);
            let contactor_to_send_boot_value = ContactorToSendBootEnum::FirstContactor;
            let mut msg = BarContactorBootRequestMsg::new(contactor_to_send_boot_value)
                .expect("constructor should accept generated test values");
            assert_eq!(
                msg.contactor_to_send_boot(), contactor_to_send_boot_value,
                "getter `{}` returned unexpected value",
                stringify!(contactor_to_send_boot),
            );
            let contactor_to_send_boot_next_value = ContactorToSendBootEnum::FirstContactor;
            msg.set_contactor_to_send_boot(contactor_to_send_boot_next_value)
                .expect("setter should accept generated test value");
            assert_eq!(
                msg.contactor_to_send_boot(), contactor_to_send_boot_next_value,
                "getter `{}` returned unexpected value",
                stringify!(contactor_to_send_boot),
            );
        }
    }
    #[test]
    fn test_quux_maintenance_control2_msg() {
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
        for seed in SEEDS {
            let mut u = Unstructured::new(seed);
            let liquid_chiller_pump_rpm_value: u16 = {
                let raw = u
                    .int_in_range(1000u16..=5500u16)
                    .expect("failed to generate physical interger value");
                raw
            };
            let lt_bar_pump_rpm_value: u16 = {
                let raw = u
                    .int_in_range(500u16..=5000u16)
                    .expect("failed to generate physical interger value");
                raw
            };
            let lt_foo_pump_rpm_value: u16 = {
                let raw = u
                    .int_in_range(500u16..=5000u16)
                    .expect("failed to generate physical interger value");
                raw
            };
            let bc_cac_pump_rpm_value: u16 = {
                let raw = u
                    .int_in_range(500u16..=5000u16)
                    .expect("failed to generate physical interger value");
                raw
            };
            let mut msg = QuuxMaintenanceControl2Msg::new(
                    liquid_chiller_pump_rpm_value,
                    lt_bar_pump_rpm_value,
                    lt_foo_pump_rpm_value,
                    bc_cac_pump_rpm_value,
                )
                .expect("constructor should accept generated test values");
            assert_eq!(
                msg.liquid_chiller_pump_rpm(), liquid_chiller_pump_rpm_value,
                "getter `{}` returned unexpected value",
                stringify!(liquid_chiller_pump_rpm),
            );
            assert_eq!(
                msg.lt_bar_pump_rpm(), lt_bar_pump_rpm_value,
                "getter `{}` returned unexpected value", stringify!(lt_bar_pump_rpm),
            );
            assert_eq!(
                msg.lt_foo_pump_rpm(), lt_foo_pump_rpm_value,
                "getter `{}` returned unexpected value", stringify!(lt_foo_pump_rpm),
            );
            assert_eq!(
                msg.bc_cac_pump_rpm(), bc_cac_pump_rpm_value,
                "getter `{}` returned unexpected value", stringify!(bc_cac_pump_rpm),
            );
            let liquid_chiller_pump_rpm_next_value: u16 = {
                let raw = u
                    .int_in_range(1000u16..=5500u16)
                    .expect("failed to generate physical interger value");
                raw
            };
            let lt_bar_pump_rpm_next_value: u16 = {
                let raw = u
                    .int_in_range(500u16..=5000u16)
                    .expect("failed to generate physical interger value");
                raw
            };
            let lt_foo_pump_rpm_next_value: u16 = {
                let raw = u
                    .int_in_range(500u16..=5000u16)
                    .expect("failed to generate physical interger value");
                raw
            };
            let bc_cac_pump_rpm_next_value: u16 = {
                let raw = u
                    .int_in_range(500u16..=5000u16)
                    .expect("failed to generate physical interger value");
                raw
            };
            msg.set_liquid_chiller_pump_rpm(liquid_chiller_pump_rpm_next_value)
                .expect("setter should accept generated test value");
            msg.set_lt_bar_pump_rpm(lt_bar_pump_rpm_next_value)
                .expect("setter should accept generated test value");
            msg.set_lt_foo_pump_rpm(lt_foo_pump_rpm_next_value)
                .expect("setter should accept generated test value");
            msg.set_bc_cac_pump_rpm(bc_cac_pump_rpm_next_value)
                .expect("setter should accept generated test value");
            assert_eq!(
                msg.liquid_chiller_pump_rpm(), liquid_chiller_pump_rpm_next_value,
                "getter `{}` returned unexpected value",
                stringify!(liquid_chiller_pump_rpm),
            );
            assert_eq!(
                msg.lt_bar_pump_rpm(), lt_bar_pump_rpm_next_value,
                "getter `{}` returned unexpected value", stringify!(lt_bar_pump_rpm),
            );
            assert_eq!(
                msg.lt_foo_pump_rpm(), lt_foo_pump_rpm_next_value,
                "getter `{}` returned unexpected value", stringify!(lt_foo_pump_rpm),
            );
            assert_eq!(
                msg.bc_cac_pump_rpm(), bc_cac_pump_rpm_next_value,
                "getter `{}` returned unexpected value", stringify!(bc_cac_pump_rpm),
            );
        }
    }
    #[test]
    fn test_quux_maintenance_control3_msg() {
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
        for seed in SEEDS {
            let mut u = Unstructured::new(seed);
            let lt_dr_div_valve_direct_control_enable_value: bool = u
                .arbitrary()
                .expect("failed to generate bool");
            let lt_foo_diverter_valve_control_value: bool = u
                .arbitrary()
                .expect("failed to generate bool");
            let sens_valves_direct_control_enable_value: bool = u
                .arbitrary()
                .expect("failed to generate bool");
            let sensorics_valves_control_value: bool = u
                .arbitrary()
                .expect("failed to generate bool");
            let hpc_valve_direct_control_enable_value: bool = u
                .arbitrary()
                .expect("failed to generate bool");
            let hpc_valve_control_value: bool = u
                .arbitrary()
                .expect("failed to generate bool");
            let heater_direct_control_enable_value: bool = u
                .arbitrary()
                .expect("failed to generate bool");
            let heater_on_control_value: bool = u
                .arbitrary()
                .expect("failed to generate bool");
            let heater_temp_setpoint_value: i8 = {
                let raw = u
                    .int_in_range(-39i8..=80i8)
                    .expect("failed to generate physical interger value");
                raw
            };
            let mut msg = QuuxMaintenanceControl3Msg::new(
                    lt_dr_div_valve_direct_control_enable_value,
                    lt_foo_diverter_valve_control_value,
                    sens_valves_direct_control_enable_value,
                    sensorics_valves_control_value,
                    hpc_valve_direct_control_enable_value,
                    hpc_valve_control_value,
                    heater_direct_control_enable_value,
                    heater_on_control_value,
                    heater_temp_setpoint_value,
                )
                .expect("constructor should accept generated test values");
            assert_eq!(
                msg.lt_dr_div_valve_direct_control_enable(),
                lt_dr_div_valve_direct_control_enable_value,
                "getter `{}` returned unexpected value",
                stringify!(lt_dr_div_valve_direct_control_enable),
            );
            assert_eq!(
                msg.lt_foo_diverter_valve_control(), lt_foo_diverter_valve_control_value,
                "getter `{}` returned unexpected value",
                stringify!(lt_foo_diverter_valve_control),
            );
            assert_eq!(
                msg.sens_valves_direct_control_enable(),
                sens_valves_direct_control_enable_value,
                "getter `{}` returned unexpected value",
                stringify!(sens_valves_direct_control_enable),
            );
            assert_eq!(
                msg.sensorics_valves_control(), sensorics_valves_control_value,
                "getter `{}` returned unexpected value",
                stringify!(sensorics_valves_control),
            );
            assert_eq!(
                msg.hpc_valve_direct_control_enable(),
                hpc_valve_direct_control_enable_value,
                "getter `{}` returned unexpected value",
                stringify!(hpc_valve_direct_control_enable),
            );
            assert_eq!(
                msg.hpc_valve_control(), hpc_valve_control_value,
                "getter `{}` returned unexpected value", stringify!(hpc_valve_control),
            );
            assert_eq!(
                msg.heater_direct_control_enable(), heater_direct_control_enable_value,
                "getter `{}` returned unexpected value",
                stringify!(heater_direct_control_enable),
            );
            assert_eq!(
                msg.heater_on_control(), heater_on_control_value,
                "getter `{}` returned unexpected value", stringify!(heater_on_control),
            );
            assert_eq!(
                msg.heater_temp_setpoint(), heater_temp_setpoint_value,
                "getter `{}` returned unexpected value",
                stringify!(heater_temp_setpoint),
            );
            let lt_dr_div_valve_direct_control_enable_next_value: bool = u
                .arbitrary()
                .expect("failed to generate bool");
            let lt_foo_diverter_valve_control_next_value: bool = u
                .arbitrary()
                .expect("failed to generate bool");
            let sens_valves_direct_control_enable_next_value: bool = u
                .arbitrary()
                .expect("failed to generate bool");
            let sensorics_valves_control_next_value: bool = u
                .arbitrary()
                .expect("failed to generate bool");
            let hpc_valve_direct_control_enable_next_value: bool = u
                .arbitrary()
                .expect("failed to generate bool");
            let hpc_valve_control_next_value: bool = u
                .arbitrary()
                .expect("failed to generate bool");
            let heater_direct_control_enable_next_value: bool = u
                .arbitrary()
                .expect("failed to generate bool");
            let heater_on_control_next_value: bool = u
                .arbitrary()
                .expect("failed to generate bool");
            let heater_temp_setpoint_next_value: i8 = {
                let raw = u
                    .int_in_range(-39i8..=80i8)
                    .expect("failed to generate physical interger value");
                raw
            };
            msg.set_lt_dr_div_valve_direct_control_enable(
                    lt_dr_div_valve_direct_control_enable_next_value,
                )
                .expect("setter should accept generated test value");
            msg.set_lt_foo_diverter_valve_control(
                    lt_foo_diverter_valve_control_next_value,
                )
                .expect("setter should accept generated test value");
            msg.set_sens_valves_direct_control_enable(
                    sens_valves_direct_control_enable_next_value,
                )
                .expect("setter should accept generated test value");
            msg.set_sensorics_valves_control(sensorics_valves_control_next_value)
                .expect("setter should accept generated test value");
            msg.set_hpc_valve_direct_control_enable(
                    hpc_valve_direct_control_enable_next_value,
                )
                .expect("setter should accept generated test value");
            msg.set_hpc_valve_control(hpc_valve_control_next_value)
                .expect("setter should accept generated test value");
            msg.set_heater_direct_control_enable(heater_direct_control_enable_next_value)
                .expect("setter should accept generated test value");
            msg.set_heater_on_control(heater_on_control_next_value)
                .expect("setter should accept generated test value");
            msg.set_heater_temp_setpoint(heater_temp_setpoint_next_value)
                .expect("setter should accept generated test value");
            assert_eq!(
                msg.lt_dr_div_valve_direct_control_enable(),
                lt_dr_div_valve_direct_control_enable_next_value,
                "getter `{}` returned unexpected value",
                stringify!(lt_dr_div_valve_direct_control_enable),
            );
            assert_eq!(
                msg.lt_foo_diverter_valve_control(),
                lt_foo_diverter_valve_control_next_value,
                "getter `{}` returned unexpected value",
                stringify!(lt_foo_diverter_valve_control),
            );
            assert_eq!(
                msg.sens_valves_direct_control_enable(),
                sens_valves_direct_control_enable_next_value,
                "getter `{}` returned unexpected value",
                stringify!(sens_valves_direct_control_enable),
            );
            assert_eq!(
                msg.sensorics_valves_control(), sensorics_valves_control_next_value,
                "getter `{}` returned unexpected value",
                stringify!(sensorics_valves_control),
            );
            assert_eq!(
                msg.hpc_valve_direct_control_enable(),
                hpc_valve_direct_control_enable_next_value,
                "getter `{}` returned unexpected value",
                stringify!(hpc_valve_direct_control_enable),
            );
            assert_eq!(
                msg.hpc_valve_control(), hpc_valve_control_next_value,
                "getter `{}` returned unexpected value", stringify!(hpc_valve_control),
            );
            assert_eq!(
                msg.heater_direct_control_enable(),
                heater_direct_control_enable_next_value,
                "getter `{}` returned unexpected value",
                stringify!(heater_direct_control_enable),
            );
            assert_eq!(
                msg.heater_on_control(), heater_on_control_next_value,
                "getter `{}` returned unexpected value", stringify!(heater_on_control),
            );
            assert_eq!(
                msg.heater_temp_setpoint(), heater_temp_setpoint_next_value,
                "getter `{}` returned unexpected value",
                stringify!(heater_temp_setpoint),
            );
        }
    }
}
