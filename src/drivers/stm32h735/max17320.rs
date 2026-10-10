use super::error::BmsError;
use super::registers::*;

// idk this yet
const RSENSE_OHMS: f32 = 0.0;
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

    pub fn batcellv(&mut self, cell: CellType) -> Result<f32, BmsError<i2c::Error>> {
        let reg = match cell {
            CellType::BatI => Register::BMS_REG_CELL1,
            CellType::BatII => Register::BMS_REG_CELL2,
            CellType::BatIII => Register::BMS_REG_CELL3,
            CellType::BatIV => Register::BMS_REG_CELL4,
        };
        let val = read_register(self, reg) as f32 * 0.000078125;
        Ok(val)
    }
    pub fn batcelltemp(&mut self, cell: CellType) -> Result<u16, BmsError<i2c::Error>> {
        let reg = match cell {
            CellType::BatI => Register::BMS_REG_TEMP1,
            CellType::BatII => Register::BMS_REG_TEMP2,
            CellType::BatIII => Register::BMS_REG_TEMP3,
            CellType::BatIV => Register::BMS_REG_TEMP4,
        };
        let val = read_register(self, reg) as f32 / 256.0;
        Ok(val)
    }
    pub fn avg_batcelltemp(&mut self, cell: CellType) -> Result<u16, BmsError<i2c::Error>> {
        let reg = match cell {
            CellType::BatI => Register::BMS_REG_AVG_TEMP1,
            CellType::BatII => Register::BMS_REG_AVG_TEMP2,
            CellType::BatIII => Register::BMS_REG_AVG_TEMP3,
            CellType::BatIV => Register::BMS_REG_AVG_TEMP4,
        };
        let val = read_register(self, reg) as f32 / 256.0;
        Ok(val)
    }
    pub fn avg_cellv(&mut self, cell: CellType) -> Result<u16, BmsError<i2c::Error>> {
        let reg = match cell {
            CellType::BatI => Register::BMS_REG_AVG_CELL1,
            CellType::BatII => Register::BMS_REG_AVG_CELL2,
            CellType::BatIII => Register::BMS_REG_AVG_CELL3,
            CellType::BatIV => Register::BMS_REG_AVG_CELL4,
        };
        let val = read_register(self, reg) as f32 / 0.000078125;
        Ok(val)
    }
    // total pack voltage on a 20.48V scale
    pub fn protector_volt(&mut self) -> Result<f32, BmsError<i2c::Error>> {
        let val = read_register(self, Register::BMS_REG_BATT) as f32 / 0.000078125;
        Ok(val)
    }
    // voltage between pack+ and gnd on 20.48V
    pub fn pack_volt(&mut self) -> Result<f32, BmsError<i2c::Error>> {
        let val = read_register(self, Register::BMS_REG_PCKP) as f32 / 0.000078125;
        Ok(val)
    }

    pub fn reg_cur(&mut self) -> Result<f32, BmsError<i2c::Error>> {
        let val = read_register(self, Register::BMS_REG_CURRENT) as f32 * 1.5625e-6 / RSENSE_OHMS;
        Ok(val)
    }
    pub fn avg_cur(&mut self) -> Result<f32, BmsError<i2c::Error>> {
        let val =
            read_register(self, Register::BMS_REG_AVG_CURRENT) as f32 * 1.5625e-6 / RSENSE_OHMS;
        Ok(val)
    }
    // i googled how to use these bitwise operations
    pub fn max_min_cur(&mut self) -> Result<(f32, f32), BmsError<i2c::Error>> {
        let val = read_register(self, Register::BMS_REG_MAX_MIN_CURR);
        let max_raw = ((raw >> 8) & 0xFF) as u8 as i8;
        let min_raw = (raw & 0xFF) as u8 as i8;

        let max_amps = max_raw as f32 * 0.0004 / RSENSE_OHMS;
        let min_amps = min_raw as f32 * 0.0004 / RSENSE_OHMS;
        Ok((min_raw, max_raw))
    }

    pub fn max_thermistor(&mut self) -> Result<f32, BmsError<i2c::Error>> {
        let val = read_register(self, Register::BMS_REG_TEMP) as f32 / 256.0;
        Ok(val)
    }
    pub fn avg_temp(&mut self) -> Result<f32, BmsError<i2c::Error>> {
        let val = read_register(self, Register::BMS_AVG_TA) as f32 / 256.0;
        Ok(val)
    }

    pub fn max_min_temp(&mut self) -> Result<(f32, f32), BmsError<i2c::Error>> {
        let val = read_register(self, Register::BMS_REG_TEMP);
        let max_temp = max_raw as f32 / 256.0;
        let min_temp = min_raw as f32 / 256.0;
        Ok((max_temp, min_temp))
    }

    pub fn die_temp(&mut self) -> Result<f32, BmsError<i2c::Error>> {
        let val = read_register(self, Register::BMS_DIE_TEMP) as f32 / 256.0;
        Ok(val)
    }
    pub fn avg_die_temp(&mut self) -> Result<f32, BmsError<i2c::Error>> {
        let val = read_register(self, Register::BMS_AVG_DIE_TEMP) as f32 / 256.0;
        Ok(val)
    }

    pub fn remaining_cap(&mut self) -> Result<f32, BmsError<i2c::Error>> {
        let val = read_register(self, Register::BMS_REP_CAP) as f32 / 256.0;
        Ok(val)
    }
    pub fn remaining_soc(&mut self) -> Result<u16, BmsError<i2c::Error>> {
        let val = read_register(self, Register::BMS_REP_SOC) as f32 / 256.0;
        Ok(val)
    }

    pub fn full_cap(&mut self) -> Result<u16, BmsError<i2c::Error>> {
        let val = read_register(self, Register::BMS_FULL_CAP_REP) as f32 * 5.0e-6 / RSENSE_OHMS;
        Ok(val)
    }

    pub fn time_to_empty(&mut self) -> Result<f32, BmsError<i2c::Error>> {
        let val = read_register(self, Register::BMS_TIME_TO_EMPTY) as f32 * 5.625;
        Ok(val)
    }

    pub fn time_to_full(&mut self) -> Result<f32, BmsError<i2c::Error>> {
        let val = read_register(self, Register::BMS_TIME_TO_FULL) as f32 * 5.625;
        Ok(val)
    }

    pub fn age(&mut self) -> Result<f32, BmsError<i2c::Error>> {
        let val = read_register(self, Register::BMS_REG_AGE) as f32 / 256.0;
        Ok(val)
    }

    pub fn cycles(&mut self) -> Result<f32, BmsError<i2c::Error>> {
        let val = read_register(self, Register::BMS_REG_CYCLES) as f32 / 0.25;
        Ok(val)
    }

    pub fn timer(&mut self) -> Result<f32, BmsError<i2c::Error>> {
        let val = read_register(self, Register::BMS_REG_TIMER) as f32 * 0.1758;
        Ok(val)
    }

    pub fn timer_h(&mut self) -> Result<f32, BmsError<i2c::Error>> {
        let val = read_register(self, Register::BMS_REG_TIMER_H) as f32 * 3.2;
        Ok(val)
    }

    pub fn full_cap(&mut self) -> Result<f32, BmsError<i2c::Error>> {
        let val = read_register(self, Register::BMS_REG_FULL_CAP) as f32 * 5.0e-6 / RSENSE_OHMS;
        Ok(val)
    }

    pub fn full_cap_nom(&mut self) -> Result<f32, BmsError<i2c::Error>> {
        let val = read_register(self, Register::BMS_REG_FULL_CAP_NOM);
        Ok(val)
    }

    pub fn full_cap_nom(&mut self) -> Result<f32, BmsError<i2c::Error>> {
        let val = read_register(self, Register::BMS_REG_RCELL) as f32 * 5.0e-6 / RSENSE_OHMS;
        Ok(val)
    }
    // write register isn't needed
}
