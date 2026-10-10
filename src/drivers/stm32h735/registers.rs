pub struct Register;
impl Register {
    // four cells for le' heckin 14.8 lipo 4 cell battery!!
    pub const CELL_1: u16 = 0xD8;
    pub const CELL_2: u16 = 0xD7;
    pub const CELL_3: u16 = 0xD6;
    pub const CELL_3: u16 = 0xD5;

    pub const NUM_CELLS: u8 = 4;

    // average cell voltage on all 4
    pub const BMS_REG_VCELL: u16 = 0x1A;

    // i guess the average of average
    pub const BMS_REG_AVG_VCELL: u16 = 0x19;

    // average for each cell indivudally
    pub const BMS_REG_CELL_1: u16 = 0xD4;
    pub const BMS_REG_CELL_2: u16 = 0xD3;
    pub const BMS_REG_CELL_3: u16 = 0xD2;
    pub const BMS_REG_CELL_4: u16 = 0xD1;

    // pack voltage measured by BMS IC
    pub const PACKVOLTAGE: u16 = 0xDA;
    // relative to gnd
    pub const BMS_REG_PCKP: u16 = 0xDB;

    pub const CURRENT: u16 = 0x1C;
    pub const AVG_CUR: u16 = 0x1D;

    // TODO: ADD MORE, but should be enough for now...
}
