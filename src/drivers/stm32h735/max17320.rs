use super::error::BmsError;
use super::registers::*;

use stm32_7xx_hal::*;

enum CellType {
    BatI,
    BatII,
    BatIII,
    BatIV,
}
pub struct BMS {
    i2c: I2c,
}

impl BMS {
    // get I2c from Peripherals::take()
    pub fn new(i2c: I2c) -> Self {
        Self { i2c }
    }

    fn read_register(&mut self, reg: u16) -> Result<u16, BmsError<i2c::Error>> {
        let mut buf = [0u8; 2];
        self.i2c
            .write_read(Register::MAX17320_ADRR, &[reg as u8], &mut buf)
            .map_err(BmsError::I2c)?;
        Ok(u16::from_le_bytes(buf))
    }

    pub fn read_batcell(&mut self, cell: CellType) -> Result<u16, BmsError<i2c::Error>> {
        let reg = match cell {
            CellType::BatI => Register::BMS_REG_CELL1,
            CellType::BatII => Register::BMS_REG_CELL2,
            CellType::BatIII => Register::BMS_REG_CELL3,
            CellType::BatIV => Register::BMS_REG_CELL4,
        };
        let val = read_register(self, reg);
        val
    }
    pub fn read_avg_batcell(&mut self, cell: CellType) -> Result<u16, BmsError<i2c::Error>> {
        let reg = match cell {
            CellType::BatI => Register::BMS_REG_AVG_CELL1,
            CellType::BatII => Register::BMS_REG_AVG_CELL2,
            CellType::BatIII => Register::BMS_REG_AVG_CELL3,
            CellType::BatIV => Register::BMS_REG_AVG_CELL4,
        };
        let val = read_register(self, reg);
        val
    }
    // total pack voltage on a 20.48V scale
    pub fn protector_volt(&mut self) {
        let val = read_register(self, Register::BMS_REG_BATT);
        val
    }
    // voltage between pack+ and gnd on 20.48V
    pub fn pack_volt(&mut self) {
        let val = read_register(self, Register::BMS_REG_PCKP);
        val
    }

    pub fn reg_cur(&mut self) {
        let val = read_register(self, Register::BMS_REG_CURRENT);
        val
    }
    pub fn avg_cur(&mut self) {
        let val = read_register(self, Register::BMS_REG_AVG_CURRENT);
        val
    }
    // idk what the hell this does
    pub fn max_min_cur(&mut self) {
        let val = read_register(self, Register::BMS_REG_MAX_MIN_CURR);
        val
    }

    pub fn max_thermistor(&mut self) {
        let val = read_register(self, Register::BMS_REG_TEMP);
        val
    }
    pub fn avg_temp(&mut self) {
        let val = read_register(self, Register::BMS_AVG_TA);
        val
    }

    pub fn max_min_temp(&mut self) -> Result<(f32, f32), BmsError<i2c::Error>> {
        let val = read_register(self, Register::BMS_REG_TEMP);
        Ok(val)
    }

    pub fn die_temp(&mut self) -> Result<u16, BmsError<i2c::Error>> {
        let val = read_register(self, Register::BMS_DIE_TEMP);
        Ok(val)
    }
    pub fn avg_die_temp(&mut self) -> Result<u16, BmsError<i2c::Error>> {
        let val = read_register(self, Register::BMS_AVG_DIE_TEMP);
        Ok(val)
    }
    // write register isn't needed
}
