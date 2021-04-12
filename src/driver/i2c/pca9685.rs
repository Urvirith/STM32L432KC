use crate::hal::{i2c::I2c, timer::Timer};

// DEFAULT ADDRESS
pub const ADDR_PCA9685:         u32 = 0x40;

// AREA FOR THE PCA9685
const PCA_MODE_1:               u8 = 0x00;
const MODE_1_RESTART:           u8 = 0x80;
const MODE_1_SLEEP:             u8 = 0x10;
const PCA_RESET:                [u8; 2] = [PCA_MODE_1, MODE_1_RESTART]; // MODE 1 REGISTER
const PCA_PRESCALE:             u8 = 0xFE; // PRESCALE REGISTER
const LED0_ON_L:                u8 = 0x06;
//const MODE1_AI: u8 = 0x20;
const FREQ_OSSC:                u32 = 25000000;            // FREQ OF INTERNAL OSCILLATOR IN DATASHEET
const PRESCALE_MAX_PCA9685:     u32 = 255;
const PRESCALE_MIN_PCA9685:     u32 = 3;
const FREQ_MAX_PCA9685:         u32 = 3500;
const FREQ_MIN_PCA9685:         u32 = 1;

const SEVEN_BIT:                bool = false;

pub fn init(i2c: &I2c, addr: u32, tim: &Timer) {
    reset(i2c, addr, tim);
    set_pwm_freq(i2c, addr, 60, tim);
}

pub fn reset(i2c: &I2c, addr: u32, tim: &Timer) {
    i2c.std_write(addr, SEVEN_BIT, SEVEN_BIT, &PCA_RESET); // SET UP THE ACCELERATION SCALE
    tim.wait();
}

// PLEASE READ https://github.com/adafruit/Adafruit-PWM-Servo-Driver-Library/blob/master/Adafruit_PWMServoDriver.cpp 
// THIS WILL NEED TO BE LOOKED AT
pub fn set_pwm_freq(i2c: &I2c, addr: u32, freq: u32, tim: &Timer) {
    let freq_val;
    let prescale_val;
    
    if freq > FREQ_MAX_PCA9685 { 
        freq_val = FREQ_MAX_PCA9685;
    } else if freq < FREQ_MIN_PCA9685 {
        freq_val = FREQ_MIN_PCA9685;
    } else {
        freq_val = freq;
    };
    
    let prescale = ((((FREQ_OSSC / (freq_val * 4096) * 10)) + 5) - 10) / 10;
    
    if prescale > PRESCALE_MAX_PCA9685 {
        prescale_val = PRESCALE_MAX_PCA9685 as u8;
    } else if prescale < PRESCALE_MIN_PCA9685 {
        prescale_val = PRESCALE_MIN_PCA9685 as u8;
    } else {
        prescale_val = prescale as u8;
    };

    let old_mode = i2c.std_read_u8(addr, SEVEN_BIT, SEVEN_BIT, PCA_MODE_1);

    i2c.std_write(addr, SEVEN_BIT, SEVEN_BIT, &[PCA_MODE_1, ((old_mode & !MODE_1_RESTART) | MODE_1_SLEEP)]);
    i2c.std_write(addr, SEVEN_BIT, SEVEN_BIT, &[PCA_PRESCALE, prescale_val]);
    i2c.std_write(addr, SEVEN_BIT, SEVEN_BIT, &[PCA_MODE_1, old_mode]);
    tim.wait();
    i2c.std_write(addr, SEVEN_BIT, SEVEN_BIT, &[PCA_MODE_1, old_mode | 0xA1]);
}

pub fn set_pin(i2c: &I2c, addr: u32, num: u8, val: u16, invert: bool) {
    let eval_val;

    if val > 4095 {
        eval_val = 4095;
    } else {
        eval_val = val;
    }

    if invert == true {
        match eval_val {
            0 =>        set_pwm(i2c, addr, num, 4096, 0),
            4095 =>     set_pwm(i2c, addr, num, 0, 4096),
            _ =>        set_pwm(i2c, addr, num, 0, 4095 - eval_val),
        }
    } else {
        match eval_val {
            0 =>        set_pwm(i2c, addr, num, 0, 4096),
            4095 =>     set_pwm(i2c, addr, num, 4096, 0),
            _ =>        set_pwm(i2c, addr, num, 0, eval_val),
        }
    }
}

pub fn set_pwm(i2c: &I2c, addr: u32, num: u8, on: u16, off: u16) {
    let pwm = [
        LED0_ON_L + 4 * num,
        on as u8,
        (on >> 8) as u8,
        off as u8,
        (off >> 8) as u8
    ];
    i2c.std_write(addr, SEVEN_BIT, SEVEN_BIT, &pwm);
}
