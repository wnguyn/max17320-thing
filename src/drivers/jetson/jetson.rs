// TODO: make this also work for rx? but like documentation
// only says the bms connects to rx thourgh? may need to recieve data later but
use stm32h7xx_hal::{block, hal::serial::Write};
// change uart later??
pub struct Jetson<T> {
    tx: T,
}

impl<T: Write<u8>> Jetson<T> {
    pub fn new(tx: T) -> Self {
        Self { tx }
    }

    pub fn send_byte(&mut self, byte: u8) -> Result<(), T::Error> {
        block!(self.tx.write(byte))
    }

    pub fn send_bytes(&mut self, bytes: &[u8]) -> Result<(), T::Error> {
        for &byte in bytes {
            self.send_byte(byte)?;
        }

        block!(self.tx.flush())
    }
}
