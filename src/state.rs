use crate::drivers::stm32h735::error::BmsError;
use crate::drivers::stm32h735::max17320::{CellType, BMS};
use stm32h7xx_hal::i2c;

#[derive(Clone, Copy)]
pub enum BmsState {
    Standby,
    Charging,
    Discharging,
    Fault(FaultReason),
}

// I actually don't know if this is gonna be used
#[derive(Clone, Copy)]
pub enum FaultReason {
    OverCurrent,
    CellOverVoltage,
    CellUnderVoltage,
    HighTemperature,
    LowTemperature,
    CellVoltageSensorFault,
    CommsFault,
}

pub enum JetsonMessage {
    Status(BmsState),
    Event(FaultReason),
    Fault(FaultReason),
    Tick,
}
