// Library for use of the MPL3115A2
// This will set to high accuracy altmeter
// Temperature will read in degrees Celcius

use crate::hal::i2c::I2c;

const INDEX_BREAK:              usize = 50;

// AREA FOR THE MPL3115A2
const ADDR_MPL3115A2:           u32 = 0x60;

// READ VALUES
//const IDENT_SELF: u8 = 0x0C; // NOT USED CURRENTLY

const MPL_CR1_REG:              u8 = 0x26;
const MPL_CR1_SYSB:             u8 = 1 << 0; // SET FOR ACTIVE POLLING EVERY SECOND
const MPL_CR1_OST:              u8 = 1 << 1; // CALL FOR A ONS CALL
const MPL_CR1_RST:              u8 = 1 << 2;
const MPL_CR1_ALT:              u8 = 1 << 7;

const MPL_PT_DATA_CFG:          u8 = 0x13;
const MPL_PT_DATA_CFG_TDEFE:    u8 = 1 << 0;
const MPL_PT_DATA_CFG_PDEFE:    u8 = 1 << 1;
const MPL_PT_DATA_CFG_DREM:     u8 = 1 << 2;

const MPL_STATUS_REG:           u8 = 0x00;

const MPL_REG_STATUS_TDR:       u8 = 1 << 1;
const MPL_REG_STATUS_PDR:       u8 = 1 << 2;
const MPL_REG_STATUS_DREM:      u8 = 1 << 3;

const MPL_PRESS_REG:            u8 = 0x1;
const MPL_TEMP_REG:             u8 = 0x4;

const BAR_IN_REG:               u8 = 0x14;

const SEVEN_BIT:                bool = false;

pub fn init(i2c: &I2c, pres: u32) -> bool {

    let mut i = 0;
    i2c.std_write(ADDR_MPL3115A2, SEVEN_BIT, SEVEN_BIT, &[MPL_CR1_REG, MPL_CR1_RST]);

    while (i2c.std_read_u8(ADDR_MPL3115A2, SEVEN_BIT, SEVEN_BIT, MPL_CR1_REG) & MPL_CR1_RST) == MPL_CR1_RST {
        if i > INDEX_BREAK {
            return false;
        }
        i += 1;
    }

    let mpl_cr1_os128 = 7 << 3; // SET TO 0 FOR A SCAN OF 6ms fastest sampling // 7 for 512ms high resolution
    let mut cntr_reg1 = MPL_CR1_ALT | mpl_cr1_os128;
    let data_config = MPL_PT_DATA_CFG_TDEFE | MPL_PT_DATA_CFG_PDEFE | MPL_PT_DATA_CFG_DREM;
    let mut array: [u8; 2] = [0, 2];

    convert_u8_array_u16((pres / 2) as u16, &mut array, 0);

    i2c.std_write(ADDR_MPL3115A2, SEVEN_BIT, SEVEN_BIT, &[MPL_CR1_REG, cntr_reg1]);
    i2c.std_write(ADDR_MPL3115A2, SEVEN_BIT, SEVEN_BIT, &[MPL_PT_DATA_CFG, data_config]);
    i2c.std_write(ADDR_MPL3115A2, SEVEN_BIT, SEVEN_BIT, &[BAR_IN_REG, array[0], array[1]]);

    cntr_reg1 |= MPL_CR1_SYSB;
    i2c.std_write(ADDR_MPL3115A2, SEVEN_BIT, SEVEN_BIT, &[MPL_CR1_REG, cntr_reg1]);

    return true;
}

pub fn get_altitude(i2c: &I2c) -> i32 {
    let mut altitude;
    let mut i = 0;
    let mut alt_bytes = [0, 0, 0];

    //toggle_ons(i2c);

    while (i2c.std_read_u8(ADDR_MPL3115A2, SEVEN_BIT, SEVEN_BIT, MPL_STATUS_REG) & MPL_REG_STATUS_PDR) != MPL_REG_STATUS_PDR {
        if i > INDEX_BREAK {
            return 0;
        }
        i += 1;
    }

    i2c.std_read(ADDR_MPL3115A2, SEVEN_BIT, SEVEN_BIT, &[MPL_PRESS_REG], &mut alt_bytes);

    altitude = (alt_bytes[0] as i32) << 8;
    altitude |= (alt_bytes[1] as i32) << 0;
    altitude = altitude * 100;
    altitude = altitude + ((((alt_bytes[2] as i32) >> 4) * 100) / 16);

    return altitude;
}

pub fn get_temperature(i2c: &I2c) -> i32 {
    let mut temperature;
    let mut i = 0;
    let mut temp_bytes = [0, 0];

    //toggle_ons(i2c);

    while (i2c.std_read_u8(ADDR_MPL3115A2, SEVEN_BIT, SEVEN_BIT, MPL_STATUS_REG) & MPL_REG_STATUS_TDR) != MPL_REG_STATUS_TDR {
        if i > INDEX_BREAK {
            return 0;
        }
        i += 1;
    }

    i2c.std_read(ADDR_MPL3115A2, SEVEN_BIT, SEVEN_BIT, &[MPL_TEMP_REG], &mut temp_bytes);

    temperature = (temp_bytes[0] as i32) * 100;
    temperature = temperature + ((((temp_bytes[1] as i32) >> 4) * 100) / 16);

    return temperature;
}

pub fn get_alt_temp(i2c: &I2c, buf: &mut [i16]) {
    let mut altitude;
    let mut temperature;
    let mut alt_temp_bytes: [u8; 5] = [0; 5];

    i2c.std_read(ADDR_MPL3115A2, SEVEN_BIT, SEVEN_BIT, &[MPL_PRESS_REG], &mut alt_temp_bytes);

    altitude = (alt_temp_bytes[0] as i16) << 8;
    altitude |= (alt_temp_bytes[1] as i16) << 0;
    altitude = altitude * 10;
    altitude = altitude + ((((alt_temp_bytes[2] as i16) >> 4) * 10) / 16);
    buf[0] = altitude;

    temperature = (alt_temp_bytes[3] as i16) * 10;
    temperature = temperature + ((((alt_temp_bytes[4] as i16) >> 4) * 10) / 16);
    buf[1] = temperature;
}

pub fn get_alt_read_flag(i2c: &I2c) -> bool {
    if (i2c.std_read_u8(ADDR_MPL3115A2, SEVEN_BIT, SEVEN_BIT, MPL_STATUS_REG) & MPL_REG_STATUS_PDR) == MPL_REG_STATUS_PDR {
        return true;
    }
    else {
        return false;
    }
}

pub fn get_temp_read_flag(i2c: &I2c) -> bool {
    if (i2c.std_read_u8(ADDR_MPL3115A2, SEVEN_BIT, SEVEN_BIT, MPL_STATUS_REG) & MPL_REG_STATUS_TDR) == MPL_REG_STATUS_TDR {
        return true;
    } else {
        return false;
    }
}

pub fn get_any_read_flag(i2c: &I2c) -> bool {
    if (i2c.std_read_u8(ADDR_MPL3115A2, SEVEN_BIT, SEVEN_BIT, MPL_STATUS_REG) & MPL_REG_STATUS_DREM) == MPL_REG_STATUS_DREM {
        return true;
    } else {
        return false;
    }
}

pub fn set_mode_altimeter(i2c: &I2c) {
    let mut cr1 = i2c.std_read_u8(ADDR_MPL3115A2, SEVEN_BIT, SEVEN_BIT, MPL_CR1_REG);
    cr1 |= MPL_CR1_ALT;
    i2c.std_write(ADDR_MPL3115A2, SEVEN_BIT, SEVEN_BIT, &[MPL_CR1_REG, cr1]);
}

pub fn set_mode_barometer(i2c: &I2c) {
    let mut cr1 = i2c.std_read_u8(ADDR_MPL3115A2, SEVEN_BIT, SEVEN_BIT, MPL_CR1_REG);
    cr1 &= !MPL_CR1_ALT;
    i2c.std_write(ADDR_MPL3115A2, SEVEN_BIT, SEVEN_BIT, &[MPL_CR1_REG, cr1]);
}

pub fn set_mode_active(i2c: &I2c) {
    let mut cr1 = i2c.std_read_u8(ADDR_MPL3115A2, SEVEN_BIT, SEVEN_BIT, MPL_CR1_REG);
    cr1 |= MPL_CR1_SYSB;
    i2c.std_write(ADDR_MPL3115A2, SEVEN_BIT, SEVEN_BIT, &[MPL_CR1_REG, cr1]);
}

pub fn set_mode_standby(i2c: &I2c) {
    let mut cr1 = i2c.std_read_u8(ADDR_MPL3115A2, SEVEN_BIT, SEVEN_BIT, MPL_CR1_REG);
    cr1 &= !MPL_CR1_SYSB;
    i2c.std_write(ADDR_MPL3115A2, SEVEN_BIT, SEVEN_BIT, &[MPL_CR1_REG, cr1]);
}

pub fn toggle_ons(i2c: &I2c) {
    let mut cr1 = i2c.std_read_u8(ADDR_MPL3115A2, SEVEN_BIT, SEVEN_BIT, MPL_CR1_REG);
    cr1 |= MPL_CR1_OST;
    i2c.std_write(ADDR_MPL3115A2, SEVEN_BIT, SEVEN_BIT, &[MPL_CR1_REG, cr1]);
}

pub fn check_fail(buf: &[i16]) -> bool {
    let mut i = 0;

    while i < buf.len() {
        if buf[i] != 0 {
            return false;
        }
        i += 1;
    }
    return true;
}

fn convert_u8_array_u16(int: u16, array: &mut [u8], offset: usize) {
    array[offset + 0] = ((int >> 8) & 0xFF) as u8;
    array[offset + 1] = ((int >> 0) & 0xFF) as u8;
}
