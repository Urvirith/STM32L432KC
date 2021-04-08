// Library for use of the fxos8700
// This will set to high accuracy accel and magnotometer
use crate::hal::i2c::I2c;

const INDEX_BREAK:          usize = 50;

// AREA FOR THE FXOS8700
const ADDR_FXOS8700C:       u32 = 0x1F;

const ACCEL_REG_STATUS:     u8 = 0x00; // CURRENTLY USED
const _ACCEL_REG_X_MSB:     u8 = 0x01;

const CTRL_XYZ:             u8 = 0x0E;
const CTRL_REG1:            u8 = 0x2A;
const CTRL_REG2:            u8 = 0x2B;
const MCTRL_REG1:           u8 = 0x5B;
const MCTRL_REG2:           u8 = 0x5C;

const CTRL_REG1_ACT:        u8 = 1 << 0;
// const CTRL_REG1_FR: u8 = 1 << 1;
const CTRL_REG1_LOW_NOISE:  u8 = 1 << 2;
const CTRL_REG1_DR400:      u8 = 1 << 3;

const CTRL_REG2_HIGH_RES:   u8 = 1 << 1;
const CTRL_REG2_RESET:      u8 = 1 << 6;

const CTRL_MREG1_HMS:       u8 = 3 << 0; // HYBRID MODE
const CTRL_MREG1_OSR:       u8 = 7 << 2; // OVER SAMPLE SET TO MAX
const CTRL_MREG1_ACAL:      u8 = 1 << 7; // OVER SAMPLE SET TO MAX

const CTRL_MREG2_HYB:       u8 = 1 << 5; // HYBRID MODE

const ACCEL_MG_RANGE_2G:    i16 = 4;       // 4.0984 or 0.244
const ACCEL_MG_RANGE_4G:    i16 = 2;       // 2.0492 or 0.488
const ACCEL_MG_RANGE_8G:    i16 = 1;       // 1.0246 or 0.976
const _MAG_UT_LSB:          i16 = 10;            // 10 or .1 uT

const SEVEN_BIT:            bool = false;

/* Enumerations */
/* Accelerometer Range */
pub enum Acc {Rng2G = 0x00, Rng4G = 0x01, Rng8G = 0x02}

pub fn init(i2c: &I2c, rng: Acc) -> bool {
    let mut i = 0;
    let mut cr1;
    i2c.std_write(ADDR_FXOS8700C, SEVEN_BIT, SEVEN_BIT, &[CTRL_REG2, CTRL_REG2_RESET]);

    while (i2c.std_read_u8(ADDR_FXOS8700C, SEVEN_BIT, SEVEN_BIT, CTRL_REG2) & CTRL_REG2_RESET) == CTRL_REG2_RESET {
        if i > INDEX_BREAK {
            return false;
        }
        i += 1;
    }

    cr1 = CTRL_REG1_LOW_NOISE | CTRL_REG1_DR400;

    i2c.std_write(ADDR_FXOS8700C, SEVEN_BIT, SEVEN_BIT, &[CTRL_XYZ, rng as u8]);
    i2c.std_write(ADDR_FXOS8700C, SEVEN_BIT, SEVEN_BIT, &[CTRL_REG2, CTRL_REG2_HIGH_RES]); /* High resolution */
    i2c.std_write(ADDR_FXOS8700C, SEVEN_BIT, SEVEN_BIT, &[CTRL_REG1, cr1]); /* Active, Normal Mode, Low Noise, 400Hz in Hybrid Mode */
    i2c.std_write(ADDR_FXOS8700C, SEVEN_BIT, SEVEN_BIT, &[MCTRL_REG1, (CTRL_MREG1_ACAL | CTRL_MREG1_HMS | CTRL_MREG1_OSR)]); /* Hybrid Mode, Over Sampling Rate = 16 */
    i2c.std_write(ADDR_FXOS8700C, SEVEN_BIT, SEVEN_BIT, &[MCTRL_REG2, CTRL_MREG2_HYB]); /* Jump to reg 0x33 after reading 0x06 */ // Might need a 1 << 5

    cr1 |= CTRL_REG1_ACT;
    i2c.std_write(ADDR_FXOS8700C, SEVEN_BIT, SEVEN_BIT, &[CTRL_REG1, cr1]); /* Active, Normal Mode, Low Noise, 400Hz in Hybrid Mode */

    return true;
}

// RETURNS [X, Y, Z] in [F32] FOR BOTH ACCEL AND MAG
pub fn get_acc_mag(i2c: &I2c, rng: Acc, accel_mag_data: &mut [i16]) {
    let mut raw_data: [u8; 13] = [0; 13]; // READ THE STATUS REGISTER +
    let accel_x_raw: i16;
    let accel_y_raw: i16;
    let accel_z_raw: i16;
    let mag_x_raw: i16;
    let mag_y_raw: i16;
    let mag_z_raw: i16;
    let mut scaled_accel: [i16; 3] = [0; 3];

    i2c.std_read(ADDR_FXOS8700C, SEVEN_BIT, SEVEN_BIT, &[ACCEL_REG_STATUS], &mut raw_data); // MIGHT HAVE TO READ 6 bytes starting at 0x01;
    accel_x_raw = (((raw_data[1] as i16) << 8) | ((raw_data[2] as i16) << 0)) >> 2;
    accel_y_raw = (((raw_data[3] as i16) << 8) | ((raw_data[4] as i16) << 0)) >> 2;
    accel_z_raw = (((raw_data[5] as i16) << 8) | ((raw_data[6] as i16) << 0)) >> 2;
    mag_x_raw = ((raw_data[7] as i16) << 8) | ((raw_data[8] as i16) << 0);
    mag_y_raw = ((raw_data[9] as i16) << 8) | ((raw_data[10] as i16) << 0);
    mag_z_raw = ((raw_data[11] as i16) << 8) | ((raw_data[12] as i16) << 0);

    match rng {
        Acc::Rng2G => {
            scaled_accel[0] = ((accel_x_raw as i16) * 10) / ACCEL_MG_RANGE_2G;
            scaled_accel[1] = ((accel_y_raw as i16) * 10) / ACCEL_MG_RANGE_2G;
            scaled_accel[2] = ((accel_z_raw as i16) * 10) / ACCEL_MG_RANGE_2G;
        }
        Acc::Rng4G => {
            scaled_accel[0] = ((accel_x_raw as i16) * 10) / ACCEL_MG_RANGE_4G;
            scaled_accel[1] = ((accel_y_raw as i16) * 10) / ACCEL_MG_RANGE_4G;
            scaled_accel[2] = ((accel_z_raw as i16) * 10) / ACCEL_MG_RANGE_4G;
        }
        Acc::Rng8G => {
            scaled_accel[0] = ((accel_x_raw as i16) * 10) / ACCEL_MG_RANGE_8G;
            scaled_accel[1] = ((accel_y_raw as i16) * 10) / ACCEL_MG_RANGE_8G;
            scaled_accel[2] = ((accel_z_raw as i16) * 10) / ACCEL_MG_RANGE_8G;
        }
    };

    accel_mag_data[0] = scaled_accel[0];
    accel_mag_data[1] = scaled_accel[1];
    accel_mag_data[2] = scaled_accel[2];
    accel_mag_data[3] = mag_x_raw;
    accel_mag_data[4] = mag_y_raw;
    accel_mag_data[5] = mag_z_raw;
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
