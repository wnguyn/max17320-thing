#[derive(Clone, Copy)]
pub enum BmsState {
    Standby,
    Charging,
    Discharging,
    Fault(FaultReason),
}

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
