pub struct Register;

impl Register {
    pub const MAX17320_ADDR: u8 = 0x36;
    pub const MAX17320_NV_ADDR: u8 = 0x0B;

    pub const BMS_REG_VCELL: u16 = 0x1A;
    pub const BMS_REG_AVG_VCELL: u16 = 0x19;
    pub const BMS_REG_CELL1: u16 = 0xD8;
    pub const BMS_REG_CELL2: u16 = 0xD7;
    pub const BMS_REG_CELL3: u16 = 0xD6;
    pub const BMS_REG_CELL4: u16 = 0xD5;

    pub const BMS_REG_AVG_CELL1: u16 = 0xD4;
    pub const BMS_REG_AVG_CELL2: u16 = 0xD3;
    pub const BMS_REG_AVG_CELL3: u16 = 0xD2;
    pub const BMS_REG_AVG_CELL4: u16 = 0xD1;

    pub const BMS_REG_BATT: u16 = 0xDA;
    pub const BMS_REG_PCKP: u16 = 0xDB;
    pub const BMS_REG_MAX_MIN_VOLT: u16 = 0x0008;

    pub const BMS_REG_CURRENT: u16 = 0x1C;
    pub const BMS_REG_AVG_CURRENT: u16 = 0x1D;
    pub const BMS_REG_MAX_MIN_CURR: u16 = 0x00A;
    pub const BMS_REG_nCGAIN: u16 = 0x1C8;
    pub const BMS_REG_nRSense: u16 = 0x1CF;

    pub const BMS_REG_TEMP: u16 = 0x1B;
    pub const BMS_REG_AVG_TA: u16 = 0x16;
    pub const BMS_REG_MAX_MIN_TEMP: u16 = 0x009;
    pub const BMS_REG_nThermCFG: u16 = 0x1CA;
    pub const BMS_REG_DIE_TEMP: u16 = 0x34;
    pub const BMS_REG_AVG_DIE_TEMP: u16 = 0x40;

    pub const BMS_REG_TEMP1: u16 = 0x13A;
    pub const BMS_REG_TEMP2: u16 = 0x139;
    pub const BMS_REG_TEMP3: u16 = 0x138;
    pub const BMS_REG_TEMP4: u16 = 0x137;
    pub const BMS_REG_AVG_TEMP1: u16 = 0x136;
    pub const BMS_REG_AVG_TEMP2: u16 = 0x135;
    pub const BMS_REG_AVG_TEMP3: u16 = 0x134;
    pub const BMS_REG_AVG_TEMP4: u16 = 0x133;

    pub const BMS_REG_REP_CAP: u16 = 0x005;
    pub const BMS_REG_REP_SOC: u16 = 0x006;
    pub const BMS_REG_FULL_CAP_REP: u16 = 0x10;
    pub const BMS_REG_TIME_TO_EMPTY: u16 = 0x11;
    pub const BMS_REG_TIME_TO_FULL: u16 = 0x20;
    pub const BMS_REG_AGE: u16 = 0x007;
    pub const BMS_REG_CYCLES: u16 = 0x017;
    pub const BMS_REG_NCycles: u16 = 0x1A4;

    pub const BMS_REG_TIMER: u16 = 0x3E;
    pub const BMS_REG_TIMER_H: u16 = 0xBE;
    pub const BMS_REG_FULL_CAP: u16 = 0x35;
    pub const BMS_REG_FULL_CAP_NOM: u16 = 0x1A5;
    pub const BMS_REG_RCELL: u16 = 0x14;

}
