// Library for use of the fxas21002C
// This will set to high accuracy gyrometer
use crate::hal::i2c::I2c;

const INDEX_BREAK:                  usize = 50;

// AREA FOR THE FXOS8700
const ADDR_FXAS21002C:              u32 = 0x21;

const GYRO_REG_STATUS:              u8 = 0x00; // CURRENTLY USED
const _GYRO_REG_X_MSB:              u8 = 0x01;

const CTRL_REG0:                    u8 = 0x0D;
const CTRL_REG1:                    u8 = 0x13;

const CTRL_REG1_STANDBY:            u8 = 0x00;
const CTRL_REG1_ACTIVE:             u8 = 0x0E;
const CTRL_REG1_RESET:              u8 = 1 << 6;

const GYRO_SENSITIVITY_250DPS:      i16 = 128;   // 0.0078125;
const GYRO_SENSITIVITY_500DPS:      i16 = 64;    // 0.015625;
const GYRO_SENSITIVITY_1000DPS:     i16 = 32;   // 0.03125;
const GYRO_SENSITIVITY_2000DPS:     i16 = 16;   // 0.0625;

const SEVEN_BIT:                    bool = false;

/* Enumerations */
/* Gyro Sensitivity */
pub enum GyroSens {Dps2000 = 0x00, Dps1000 = 0x01, Dps500 = 0x02, Dps250 = 0x03}

// const DPS_TO_RADS: f32 = 0.0175;

pub fn init(i2c: &I2c, rng: GyroSens) -> bool {
    let mut index = 0;

    i2c.std_write(ADDR_FXAS21002C, SEVEN_BIT, SEVEN_BIT, &[CTRL_REG1, CTRL_REG1_STANDBY]);
    i2c.std_write(ADDR_FXAS21002C, SEVEN_BIT, SEVEN_BIT, &[CTRL_REG1, CTRL_REG1_RESET]);

    while (i2c.std_read_u8(ADDR_FXAS21002C, SEVEN_BIT, SEVEN_BIT, CTRL_REG1) & CTRL_REG1_RESET) == CTRL_REG1_RESET {
        if index > INDEX_BREAK {
            return false;
        }
        index = index + 1;
    }


    i2c.std_write(ADDR_FXAS21002C, SEVEN_BIT, SEVEN_BIT, &[CTRL_REG0, rng as u8]);
    i2c.std_write(ADDR_FXAS21002C, SEVEN_BIT, SEVEN_BIT, &[CTRL_REG1, CTRL_REG1_ACTIVE]);

    return true;
}

// RETURNS [X, Y, Z] in [F32]
pub fn get_gyro(i2c: &I2c, rng: GyroSens, data: &mut [i16]) {
    let mut raw_gyro: [u8; 7] = [0; 7];
    let x_raw: i16;
    let y_raw: i16;
    let z_raw: i16;

    i2c.std_read(ADDR_FXAS21002C, SEVEN_BIT, SEVEN_BIT, &[GYRO_REG_STATUS], &mut raw_gyro); // MIGHT HAVE TO READ 6 bytes starting at 0x01;
    x_raw = ((raw_gyro[1] as i16) << 8) | ((raw_gyro[2] as i16) << 0);
    y_raw = ((raw_gyro[3] as i16) << 8) | ((raw_gyro[4] as i16) << 0);
    z_raw = ((raw_gyro[5] as i16) << 8) | ((raw_gyro[6] as i16) << 0);

    match rng {
        GyroSens::Dps2000 => {
            data[0] = x_raw / GYRO_SENSITIVITY_2000DPS;
            data[1] = y_raw / GYRO_SENSITIVITY_2000DPS;
            data[2] = z_raw / GYRO_SENSITIVITY_2000DPS;
        }
        GyroSens::Dps1000 => {
            data[0] = x_raw / GYRO_SENSITIVITY_1000DPS;
            data[1] = y_raw / GYRO_SENSITIVITY_1000DPS;
            data[2] = z_raw / GYRO_SENSITIVITY_1000DPS;
        }
        GyroSens::Dps500 => {
            data[0] = x_raw / GYRO_SENSITIVITY_500DPS;
            data[1] = y_raw / GYRO_SENSITIVITY_500DPS;
            data[2] = z_raw / GYRO_SENSITIVITY_500DPS;
        }
        GyroSens::Dps250 => {
            data[0] = x_raw / GYRO_SENSITIVITY_250DPS;
            data[1] = y_raw / GYRO_SENSITIVITY_250DPS;
            data[2] = z_raw / GYRO_SENSITIVITY_250DPS;
        }
    };
}

pub fn check_fail(buf: &[i16]) -> bool {
    let mut i = 0;

    while i < buf.len() {
        if buf[i] != 771 {
            return false;
        }
        i += 1;
    }
    return true;
}
