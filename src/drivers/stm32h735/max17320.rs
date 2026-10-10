use super::registers::Register;
use stm32_7xx_hal::*;

// TODO: switch to generics???
pub struct BMS {
    i2c: I2c,
}

impl BMS {
    // get I2c from Peripherals::take()
    pub fn new(i2c: I2c) -> Self {
        Self { i2c }
    }
    pub fn soc(&mut self) -> Result<u16, Error> {
        let raw = self.read_register(Register::REP_SOC);
    }
}
