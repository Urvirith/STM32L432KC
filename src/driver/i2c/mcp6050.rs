/* MCP6050 Gyroscope and Accelerometer */
use crate::hal::i2c::I2c;

// AREA FOR THE GY-521 / MCP6050
const ADDR_MCP:         u32 = 0x68;

const MCP_WAKEUP:       [u8; 2] = [0x6B, 0x00]; // HW LW in that order BE transfer
const MCP_GYRO:         [u8; 2] = [0x1B, 0x00]; // HW LW in that order BE transfer
const MCP_ACCEL:        [u8; 2] = [0x1C, 0x00]; // HW LW in that order BE transfer

const MCP_GYRO_REG:     [u8; 1] = [0x43];       // HW LW in that order BE transfer
const MCP_ACCEL_REG:    [u8; 1] = [0x3B];       // HW LW in that order BE transfer
const MCP_TEMP_REG:     [u8; 1] = [0x41];       // HW LW in that order BE transfer

const SEVEN_BIT:        bool = false;

pub fn init(i2c: &I2c) {
    i2c.std_write(ADDR_MCP, SEVEN_BIT, SEVEN_BIT, &MCP_WAKEUP);    // WAKE UP THE MCP 6050
    i2c.std_write(ADDR_MCP, SEVEN_BIT, SEVEN_BIT, &MCP_GYRO);      // SET UP THE GYRO SCALE
    i2c.std_write(ADDR_MCP, SEVEN_BIT, SEVEN_BIT, &MCP_ACCEL);     // SET UP THE ACCELERATION SCALE
}

pub fn read_gyro(i2c: &I2c, buffer: &mut [u8]) {
    i2c.std_read(ADDR_MCP, SEVEN_BIT, SEVEN_BIT, &MCP_GYRO_REG, buffer);
}

pub fn read_accel(i2c: &I2c, buffer: &mut [u8]) {
    i2c.std_read(ADDR_MCP, SEVEN_BIT, SEVEN_BIT, &MCP_ACCEL_REG, buffer);
}

pub fn read_temp(i2c: &I2c, buffer: &mut [u8]) {
    i2c.std_read(ADDR_MCP, SEVEN_BIT, SEVEN_BIT, &MCP_TEMP_REG, buffer);
}

pub fn check_fail(i2c: &I2c, buffer: &[u8]) -> bool {
    let mut i = 0;

    while i < buffer.len() {
        if buffer[i] != 0 {
            return false;
        }
        i += 1;
    }
    return true;
}
