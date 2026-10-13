use crate::drivers::stm32h735::error::BmsError;
use crate::drivers::stm32h735::max17320::{CellType, BMS};
use embedded_hal::i2c::I2c;

#[derive(Debug, Clone, Copy)]
pub struct BmsStatus {
    pub soc_percent: f32,

    pub pack_voltage_v: f32,
    pub current_a: f32,

    pub cell_voltage_v: [f32; 4],
    pub temperature_c: [f32; 4],
}

// pull struct data to make a nice little thing to send over i2c.
// current implementation is like it returns a generic error (again this is just a generic library)
// because it's just easier and being more conservative is fine imo.
impl BmsStatus {
    pub fn status<T: I2c>(bms: &mut BMS<T>) -> Result<Self, BmsError<T::Error>> {
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
    // encode data to be sent over UART for the jetson, kinda crude but whatever lol
    pub fn encode(&self) -> [u8; 44] {
        let mut buf = [0u8; 44];
        let values = [
            self.soc_percent,
            self.pack_voltage_v,
            self.current_a,
            self.cell_voltage_v[0],
            self.cell_voltage_v[1],
            self.cell_voltage_v[2],
            self.cell_voltage_v[3],
            self.temperature_c[0],
            self.temperature_c[1],
            self.temperature_c[2],
            self.temperature_c[3],
        ];

        for (i, value) in values.iter().enumerate() {
            let bytes = value.to_le_bytes();
            let start = i * 4;
            buf[start..start + 4].copy_from_slice(&bytes);
        }

        buf
    }
}
