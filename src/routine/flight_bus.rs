// Contain and utilize the bus logic
use crate::hal::{timer::Timer, usart::Usart, i2c::I2c};
use crate::driver::i2c;


const ALT_BEGIN:    u8 = 0x01;
const MAG_BEGIN:    u8 = 0x02;
const GYRO_BEGIN:   u8 = 0x03;
const TERM_CHAR:    u8 = 0x0D;
const ALT_FAILED:   [u8; 3] = [0, 0, TERM_CHAR];
const MAG_FAILED:   [u8; 3] = [0, 1, TERM_CHAR];
const GYRO_FAILED:  [u8; 3] = [0, 2, TERM_CHAR];
const I2C_FAILED:   [u8; 3] = [0, 3, TERM_CHAR];

const RNG:          i2c::fxos8700::Acc = i2c::fxos8700::Acc::Rng2G;
const GYRO:         i2c::fxas21002c::GyroSens = i2c::fxas21002c::GyroSens::Dps250;

pub fn init(i2c: &I2c, usart: &Usart, tim: &Timer) {
    let pressure = 101600; // MUST BE MADE DYNAMIC, AS PRESSURE IS DETERMINATE OF ELEVATION// GPS IMPLEMENTATION WILL BE NEEDED IN THE FUTURE

    if i2c::mpl3115a2::init(i2c, pressure) == false { // REMOVE WHEN DONE TESTING
        usart.write(&ALT_FAILED);
    }

    if i2c::fxos8700::init(i2c, RNG) == false { // REMOVE WHEN DONE TESTING
        usart.write(&MAG_FAILED);
    }

    if i2c::fxas21002c::init(i2c, GYRO) == false { // REMOVE WHEN DONE TESTING
        usart.write(&GYRO_FAILED);
    }

    tim.wait();
}

pub fn read(i2c: &I2c, usart: &Usart) {
    let mut full_bus_fail_i2c1 = false;
    
    // AREA FOR THE ALTIMETER AND TEMPERATURE
    let mut mpl3115a2: [i16; 2] = [0; 2];
    let mpl3115a2_fail = alt_temp_run(i2c, usart, &mut mpl3115a2);

    if mpl3115a2_fail == true {
        alt_temp_reset(i2c, usart);
    }

    let mut fxos8700: [i16; 6] = [0; 6];
    let fxos8700_fail = acc_mag_run(i2c, usart, &mut fxos8700);

    if fxos8700_fail == true {
        acc_mag_reset(i2c, usart);
    }

    // GYROSCOPE STANDARD ERROR CANNOT BE USED
    let mut fxas21002c: [i16; 3] = [0; 3];
    let fxas21002c_fail = gyro_run(i2c, usart, &mut fxas21002c);

    if fxas21002c_fail == true {
        gyro_reset(i2c, usart);
    }

    if (mpl3115a2_fail == true) && (fxos8700_fail == true) && (fxas21002c_fail == true) {
        full_bus_fail_i2c1 = true;
    }

    if full_bus_fail_i2c1 == true {
        usart.write(&I2C_FAILED);
        i2c.stop_bus();
        i2c.start_bus();
    }
}

fn alt_temp_run(i2c: &I2c, usart: &Usart, data: &mut [i16]) -> bool {
    let mut data_array: [u8; 6] = [0; 6];

    i2c::mpl3115a2::get_alt_temp(i2c, data);

    data_array[0] = ALT_BEGIN;
    convert_u8_array_i16(data[0], &mut data_array, 1);
    convert_u8_array_i16(data[1], &mut data_array, 3);
    data_array[5] = TERM_CHAR;
    usart.write(&data_array);

    return i2c::mpl3115a2::check_fail(&data);
}

fn alt_temp_reset(i2c: &I2c, usart: &Usart) {
    let pressure = 101600;
    usart.write(&ALT_FAILED);

    i2c::mpl3115a2::set_mode_standby(i2c);
    i2c::mpl3115a2::init(i2c, pressure);
}

fn acc_mag_run(i2c: &I2c, usart: &Usart, data: &mut [i16]) -> bool {
    let mut data_array: [u8; 14] = [0; 14];
    // SENSITIVE INSTRUMENT MIGHT REQUIRE MULTIPLE RESTARTS, CURRENT I2C IMPLEMENTATION WILL RESTART AFTER MULTIPLE TIMES
    // ACCEROMETER AND MAGNOMETER
    i2c::fxos8700::get_acc_mag(i2c, RNG, data);

    data_array[0] = MAG_BEGIN;
    convert_u8_array_i16(data[0], &mut data_array, 1);
    convert_u8_array_i16(data[1], &mut data_array, 3);
    convert_u8_array_i16(data[2], &mut data_array, 5);
    convert_u8_array_i16(data[3], &mut data_array, 7);
    convert_u8_array_i16(data[4], &mut data_array, 9);
    convert_u8_array_i16(data[5], &mut data_array, 11);
    data_array[13] = TERM_CHAR;
    usart.write(&data_array);
    return i2c::fxos8700::check_fail(&data);
}

fn acc_mag_reset(i2c: &I2c, usart: &Usart) {
    usart.write(&MAG_FAILED);

    i2c::fxos8700::init(i2c, RNG);
}

fn gyro_run(i2c: &I2c, usart: &Usart, data: &mut [i16]) -> bool {
    let mut data_array: [u8; 8] = [3; 8];
    
    i2c::fxas21002c::get_gyro(i2c, GYRO, data);

    data_array[0] = GYRO_BEGIN;
    convert_u8_array_i16(data[0], &mut data_array, 1);
    convert_u8_array_i16(data[1], &mut data_array, 3);
    convert_u8_array_i16(data[2], &mut data_array, 5);
    data_array[7] = TERM_CHAR;
    usart.write(&data_array);
    
    return i2c::fxas21002c::check_fail(&data);
    //returns device_library::i2c::fxas21002c::check_fail(&fxas21002c);
}

fn gyro_reset(i2c: &I2c, usart: &Usart) {
    usart.write(&GYRO_FAILED);

    i2c::fxas21002c::init(i2c, GYRO);
}

fn convert_u8_array_i16(int: i16, array: &mut [u8], offset: usize) {
    array[offset + 0] = ((int >> 8) & 0xFF) as u8;
    array[offset + 1] = ((int >> 0) & 0xFF) as u8;
}
