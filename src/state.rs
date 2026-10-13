#[derive(Clone, Copy)]
pub enum StateLoop {
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

// plan out implmentation??/
impl StateLoop {
    // main function to find faults put it in StateLoop::Something
    // i think?
    pub fn eval_state() {}
    //
    //
    pub 
}
