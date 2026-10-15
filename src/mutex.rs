// shared mutex lock between everything important?

use embassy_sync::{
    blocking_mutex::raw::CriticalSectionRawMutex,
    mutex::Mutex,
};

pub enum SafetyState {
    Normal,
    Warn,
    Limit,
    Critical,
}

pub static SAFETY_STATE: Mutex<CriticalSectionRawMutex, SafetyState> =
    Mutex::new(SafetyState::Normal);
