use crate::drivers::stm32h735::error::BmsError;
use crate::drivers::stm32h735::max17320::{CellType, BMS};
use stm32h7xx_hal::i2c;

#[derive(Debug, Clone, Copy)]
pub struct BmsStatus {
    pub soc_percent: f32,

    pub pack_voltage_v: f32,
    pub current_a: f32,

    pub cell_voltage_v: [f32; 4],
    pub temperature_c: [f32; 4],
}

impl BmsStatus {
    pub fn status<I2C>(bms: &mut BMS<I2C>) -> Result<Self, BmsError<i2c::Error>>
    where
        I2C: stm32h7xx_hal::hal::blocking::i2c::WriteRead<Error = i2c::Error>,
    {
        Ok(Self {
            soc_percent: bms.remaining_soc()?,

            pack_voltage_v: bms.pack_volt()?,
            current_a: bms.reg_cur()?,

            cell_voltage_v: [
                bms.batcellv(CellType::BatI)?,
                bms.batcellv(CellType::BatII)?,
                bms.batcellv(CellType::BatIII)?,
                bms.batcellv(CellType::BatIV)?,
            ],

            temperature_c: [
                bms.batcelltemp(CellType::BatI)?,
                bms.batcelltemp(CellType::BatII)?,
                bms.batcelltemp(CellType::BatIII)?,
                bms.batcelltemp(CellType::BatIV)?,
            ],
        })
    }
}
