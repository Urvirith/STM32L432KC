/* Inter-Integrated Circuit (I2C) */
/* Manual Page 1127 */

use super::{common, pointer};

pub struct I2c {
    cr1:        *mut u32,       // Control Register 1
    cr2:        *mut u32,       // Control Register 2
    oar1:       *mut u32,       // Own Address Register 1
    oar2:       *mut u32,       // Own Address Register 2
    timingr:    *mut u32,       // Timing Register
    timeoutr:   *mut u32,       // Timeout Register
    isr:        *mut u32,       // Interrupt And Status Register
    icr:        *mut u32,       // Interrupt Flag Clear Register
    pecr:       *mut u32,       // PEC Register
    rxdr:       *mut u8,        // Receive Data Register
    txdr:       *mut u8         // Transmit Data Register
}

/* Register Offset */
const CR1:      u32 = 0x00;
const CR2:      u32 = 0x04;
const OAR1:     u32 = 0x08;
const OAR2:     u32 = 0x0C;
const TIMINGR:  u32 = 0x10;
const TIMEOUTR: u32 = 0x14;
const ISR:      u32 = 0x18;
const ICR:      u32 = 0x1C;
const PECR:     u32 = 0x20;
const RXDR:     u32 = 0x24;
const TXDR:     u32 = 0x28;

/* Enumerations */
/* Speed of I2C bus, 10KHz, 100KHz 400KHz or Plus mode */
pub enum TimingMode {Sm10KHz, Sm100KHz, Fm400KHz, FmPlus}

    /* Register Masks */
/* CR2 */
const ADDR_10_MASK:     u32 = common::MASK_10_BIT;
const ADDR_7_MASK:      u32 = common::MASK_9_BIT;
const NBYTES_MASK:      u32 = common::MASK_8_BIT;
/* TIMINGR */
const SCL_MASK:         u32 = common::MASK_8_BIT;
const DEL_MASK:         u32 = common::MASK_4_BIT;

/* Register Bits */
/* CR1 */
const PE_BIT:           u32 = common::BIT_0;
/* CR2 */
const RD_WRN_BIT:       u32 = common::BIT_10;      // Transfer direction (master mode) 0: Master requests a write transfer. 1: Master requests a read transfer.
const ADDR_10_BIT:      u32 = common::BIT_11;     // 10-bit addressing mode (master mode) 0: The master operates in 7-bit addressing mode, 1: The master operates in 10-bit addressing mode
const HEAD_10_BIT:      u32 = common::BIT_12;      // 10-bit address header only read direction (master receiver mode) 0: The master sends the complete 10 bit slave address read sequence:
                                        // Start + 2 bytes 10bit address in write direction + Restart + 1st 7 bits of the 10 bit address in read direction.
                                        // 1: The master only sends the 1st 7 bits of the 10 bit address, followed by Read direction.
const START_BIT:        u32 = common::BIT_13;      // This bit is set by software, and cleared by hardware after the Start followed by the address sequence is sent, by an arbitration loss, by a timeout error detection, or when PE = 0.
                                        // It can also be cleared by software by writing ‘1’ to the ADDRCF bit in the I2C_ICR register. 0: No Start generation. 1: Restart/Start generation:
const STOP_BIT:         u32 = common::BIT_14;      // The bit is set by software, cleared by hardware when a STOP condition is detected, or when PE = 0. In Master Mode: 0: No Stop generation. 1: Stop generation after current byte transfer.
const NACK_BIT:         u32 = common::BIT_15;      // The bit is set by software, cleared by hardware when the NACK is sent, or when a STOP condition or an Address matched is received, or when PE=0. 0: an ACK is sent after current received byte. 1: a NACK is sent after current received byte.
const RELOAD_BIT:       u32 = common::BIT_24;      // 0: The transfer is completed after the NBYTES data transfer (STOP or RESTART follows). 1: The transfer is not completed after the NBYTES data transfer (NBYTES is reloaded). TCR flag is set when NBYTES data are transferred, stretching SCL low.
const AUTOEND_BIT:      u32 = common::BIT_25;      // 0: software end mode: TC flag is set when NBYTES data are transferred, stretching SCL low. 1: Automatic end mode: a STOP condition is automatically sent when NBYTES data are transferred.

/* ISR */
// CONST FOR THE ISR AND ICR PG. 1234
const TXIS_BIT:         u32 = common::BIT_1;
const RXNE_BIT:         u32 = common::BIT_2;
const TC_BIT:           u32 = common::BIT_6;


/* Register Offsets */
/* CR2 */
const ADDR_10_OFFSET:   u32 =   0;
const ADDR_7_OFFSET:    u32 =   1;
const NBYTES_OFFSET:    u32 =   16;              // The number of bytes to be transmitted/received is programmed there. This field is don’t care in slave mode with SBC=0.

/* TIMINGR */
const SCLL_OFFSET:      u32 =   0;
const SCLH_OFFSET:      u32 =   8;
const SDADEL_OFFSET:    u32 =   16;
const SCLDEL_OFFSET:    u32 =   20;
const PRESC_OFFSET:     u32 =   28;

/* SETUP */
const READ:             bool = false;
const WRITE:            bool = true;
const LEN_1_BYTE:       u32 = 1;

const TIMEOUT:          u32 = common::WAIT100US;

impl I2c { 
    /* Initialize The Structure */
    pub fn init(base: u32) -> I2c {
        return I2c {
            cr1:        (base + CR1)        as *mut u32,
            cr2:        (base + CR2)        as *mut u32,
            oar1:       (base + OAR1)       as *mut u32,
            oar2:       (base + OAR2)       as *mut u32,
            timingr:    (base + TIMINGR)    as *mut u32,
            timeoutr:   (base + TIMEOUTR)   as *mut u32,
            isr:        (base + ISR)        as *mut u32,
            icr:        (base + ICR)        as *mut u32,
            pecr:       (base + PECR)       as *mut u32,
            rxdr:       (base + RXDR)       as *mut u8,
            txdr:       (base + TXDR)       as *mut u8
        };
    }
    
    // Initalization Flow ->
    // Clear PE bit in I2C_CR1
    // Configure ANFOFF and DNF[3:0] in I2C_CR1
    // Configure PRESC[3:0],
    // SDADEL[3:0], SCLDEL[3:0], SCLH[7:0], SCLL[7:0]  in I2C_TIMINGR
    // Configure NOSTRETCH in I2C_CR1
    // Set PE bit in I2C_CR1
    // End
    // INORDER TO RUN THE I2C SETUP YOU MUST FIRST I2C CLK AS APART OF THE CLOCK REGISTER AT PG. 163
    pub fn open(&self, sclk: common::MsiRange, mode: TimingMode) {
        pointer::clr_ptr_vol_bit_u32(self.cr1, PE_BIT);
        
        match sclk {
            common::MsiRange::Clk8MHz => {
                match mode {
                    TimingMode::Sm10KHz     => self.set_timing_register(0xC7, 0xC3, 0x02, 0x04, 0x01),
                    TimingMode::Sm100KHz    => self.set_timing_register(0x13, 0x0F, 0x02, 0x04, 0x01),
                    TimingMode::Fm400KHz    => self.set_timing_register(0x09, 0x03, 0x01, 0x03, 0x00),
                    TimingMode::FmPlus      => self.set_timing_register(0xC7, 0xC3, 0x02, 0x04, 0x01),
                }
            },
            common::MsiRange::Clk16MHz => {
                match mode {
                    TimingMode::Sm10KHz     => self.set_timing_register(0xC7, 0xC3, 0x02, 0x04, 0x03),
                    TimingMode::Sm100KHz    => self.set_timing_register(0x13, 0x0F, 0x02, 0x04, 0x03),
                    TimingMode::Fm400KHz    => self.set_timing_register(0x09, 0x03, 0x02, 0x03, 0x01),
                    TimingMode::FmPlus      => self.set_timing_register(0x04, 0x02, 0x00, 0x02, 0x00),
                }
            },
            common::MsiRange::Clk48MHz => {
                match mode {
                    TimingMode::Sm10KHz     => self.set_timing_register(0xC7, 0xC3, 0x02, 0x04, 0x0B),
                    TimingMode::Sm100KHz    => self.set_timing_register(0x13, 0x0F, 0x02, 0x04, 0x0B),
                    TimingMode::Fm400KHz    => self.set_timing_register(0x09, 0x03, 0x03, 0x03, 0x05),
                    TimingMode::FmPlus      => self.set_timing_register(0x03, 0x01, 0x00, 0x01, 0x05),
                }
            },
            _=> self.set_timing_register(0x19, 0x06, 0x00, 0x03, 0x00) // DEFAULT SPEED USED FOR TESTING
        }

        pointer::set_ptr_vol_bit_u32(self.cr1, PE_BIT);
    }

    pub fn start_bus(&self) { 
        pointer::set_ptr_vol_bit_u32(self.cr1, PE_BIT);
    }

    pub fn stop_bus(&self) { 
        pointer::clr_ptr_vol_bit_u32(self.cr1, PE_BIT);

        let mut i = 0; // CONVERT TO FAULT TIMER, VOLITILE WILL PREVENT OPTIMIZATION

        while i < TIMEOUT {
            i+=1;
        }
    }

    // • Addressing mode (7-bit or 10-bit): ADD10 • Slave address to be sent: SADD[9:0]
        // • Transfer direction: RD_WRN
        // • In case of 10-bit address read: HEAD10R bit. HEAD10R must be configure to indicate if the complete address sequence must be sent, or only the header in case of a direction change.
        // • The number of bytes to be transferred: NBYTES[7:0]. If the number of bytes is equal to or greater than 255 bytes, NBYTES[7:0] must initially be filled with 0xFF.
    pub fn setup(&self, slave_addr: u32, addr_10bit: bool, req_10bit: bool, byte_cnt: u32, write: bool) {
        if addr_10bit {
            pointer::set_ptr_vol_bit_u32(self.cr2, ADDR_10_BIT);
            pointer::set_ptr_vol_u32(self.cr2, ADDR_10_OFFSET, ADDR_10_MASK, slave_addr);

            if req_10bit {
                pointer::set_ptr_vol_bit_u32(self.cr2, HEAD_10_BIT);
            } else {
                pointer::clr_ptr_vol_bit_u32(self.cr2, HEAD_10_BIT);
            }
        } else {
            pointer::clr_ptr_vol_bit_u32(self.cr2, ADDR_10_BIT);
            pointer::set_ptr_vol_u32(self.cr2, ADDR_7_OFFSET, ADDR_7_MASK, slave_addr);
        }

        if write {
            pointer::clr_ptr_vol_bit_u32(self.cr2, RD_WRN_BIT);
        } else {
            pointer::set_ptr_vol_bit_u32(self.cr2, RD_WRN_BIT);
        }

        pointer::set_ptr_vol_u32(self.cr2, NBYTES_OFFSET, NBYTES_MASK, byte_cnt);

        // TO BE CHANGED TO BINARY SWITCH IF BYTE CNT >= 255;
        pointer::clr_ptr_vol_bit_u32(self.cr2, RELOAD_BIT);
        pointer::clr_ptr_vol_bit_u32(self.cr2, AUTOEND_BIT);
    }

    pub fn start(&self) -> bool {
        pointer::set_ptr_vol_bit_u32(self.cr2, START_BIT);

        let mut i = 0; // CONVERT TO FAULT TIMER

        while pointer::get_ptr_vol_bit_u32(self.cr2, START_BIT) {
            if i > TIMEOUT {
                return false;
            }
            i+=1;
        }
        return true;
    }

    pub fn stop(&self) -> bool {
        pointer::set_ptr_vol_bit_u32(self.cr2, STOP_BIT);

        let mut i = 0; 

        while pointer::get_ptr_vol_bit_u32(self.cr2, STOP_BIT){
            if i > TIMEOUT {
                return false;
            }
            i+=1;
        }
        return true;
    }

    pub fn tc(&self) -> bool {
        let mut i = 0; // CONVERT TO FAULT TIMER

        while !pointer::get_ptr_vol_bit_u32(self.isr, TC_BIT) {
            if i > TIMEOUT {
                return false;
            }

            i+=1;
        }
        return true;
    }

    // p. 1202
    // In the case of a read transfer, the RXNE flag is set after each byte reception, after the 8th SCL pulse.
    // An RXNE event generates an interrupt if the RXIE bit is set in the I2C_CR1 register. The flag is cleared when I2C_RXDR is read.
    // If the total number of data bytes to be received is greater than 255, reload mode must be selected by setting the RELOAD bit in the I2C_CR2 register.
    // In this case, when NBYTES[7:0] data have been transferred, the TCR flag is set and the SCL line is stretched low until NBYTES[7:0] is written to a non-zero value.
    // • When RELOAD=0 and NBYTES[7:0] data have been transferred:
    //      – In automatic end mode (AUTOEND=1), a NACK and a STOP are automatically sent after the last received byte.
    //      – In software end mode (AUTOEND=0), a NACK is automatically sent after the last received byte,
    //        the TC flag is set and the SCL line is stretched low in order to allow software actions:
    //          A RESTART condition can be requested by setting the START bit in the I2C_CR2 register with the proper slave address configuration, and number of bytes to be transferred.
    //          Setting the START bit clears the TC flag and the START condition, followed by slave address, are sent on the bus.
    //          A STOP condition can be requested by setting the STOP bit in the I2C_CR2 register. Setting the STOP bit clears the TC flag and the STOP condition is sent on the bus. 
    pub fn read(&self, buf: &mut [u8]) -> bool {
        let mut i = 0;
        let mut t = 0;
        while i < buf.len() {
            if pointer::get_ptr_vol_bit_u32(self.isr, RXNE_BIT) {
                buf[i] = pointer::get_ptr_vol_raw_u8(self.rxdr);
                i+=1;
                t=0;
            } else {
                if t > TIMEOUT { // Convert to fault timer rather than else statement
                    return false;
                }
                t+=1;
            }
        }
        return true;
    }

    pub fn read_u8(&self) -> u8 {
        let mut i = 0; 

        while !pointer::get_ptr_vol_bit_u32(self.isr, RXNE_BIT) {
            if i > TIMEOUT {
                return 0;
            }
            i+=1;
        }
        return pointer::get_ptr_vol_raw_u8(self.rxdr);
    }

    pub fn std_read(&self, slave_addr: u32, addr_10bit: bool, req_10bit: bool, buf_write: &[u8], buf_read: &mut [u8]) {
        self.setup(slave_addr, addr_10bit, req_10bit, buf_write.len() as u32, WRITE);
        self.start();
        self.write(buf_write);
        self.tc();
        self.setup(slave_addr, addr_10bit, req_10bit, buf_read.len() as u32, READ);
        self.start();
        self.read(buf_read);
        self.tc();
        self.stop();

        //return 0; // CAN BE USED LATER FOR ALARMING OR TURN TO VOID
    }

    pub fn std_read_u8(&self, slave_addr: u32, addr_10bit: bool, req_10bit: bool, byte_write: u8) -> u8 {
        self.setup(slave_addr, addr_10bit, req_10bit, LEN_1_BYTE, WRITE);
        self.start();
        self.write_u8(byte_write);
        self.tc();
        self.setup(slave_addr, addr_10bit, req_10bit, LEN_1_BYTE, READ);
        self.start();
        let byte = self.read_u8();
        self.tc();
        self.stop();

        return byte; // CAN BE USED LATER FOR ALARMING OR TURN TO VOID
    }

    // p. 1198
    // In the case of a write transfer,
    // the TXIS flag is set after each byte transmission, after the 9th SCL pulse when an ACK is received.
    // A TXIS event generates an interrupt if the TXIE bit is set in the I2C_CR1 register.
    // The flag is cleared when the I2C_TXDR register is written with the next data byte to be transmitted.
    // The number of TXIS events during the transfer corresponds to the value programmed in NBYTES[7:0].
    // If the total number of data bytes to be sent is greater than 255, reload mode must be selected by setting the RELOAD bit in the I2C_CR2 register.
    // In this case, when NBYTES data have been transferred, the TCR flag is set and the SCL line is stretched low until NBYTES[7:0] is written to a non-zero value.
    // The TXIS flag is not set when a NACK is received.
    //  • When RELOAD=0 and NBYTES data have been transferred:
    //      – In automatic end mode (AUTOEND=1), a STOP is automatically sent.
    //      – In software end mode (AUTOEND=0), the TC flag is set and the SCL line is stretched low in order to perform software actions:
    //          A RESTART condition can be requested by setting the START bit in the I2C_CR2 register with the proper slave address configuration, and number of bytes to be transferred.
    //          Setting the START bit clears the TC flag and the START condition is sent on the bus. A STOP condition can be requested by setting the STOP bit in the I2C_CR2 register.
    //          Setting the STOP bit clears the TC flag and the STOP condition is sent on the bus.
    //  • If a NACK is received: the TXIS flag is not set, and a STOP condition is automatically sent after the NACK reception.
    //    the NACKF flag is set in the I2C_ISR register, and an interrupt is generated if the NACKIE bit is set.
    pub fn write(&self, buf: &[u8]) -> bool {
        let mut i = 0; 
        let mut t = 0; // CONVERT TO FAULT TIMER

        while i < buf.len() {
            if pointer::get_ptr_vol_bit_u32(self.isr, TXIS_BIT) {
                pointer::set_ptr_vol_raw_u8(self.txdr, buf[i]);
                i+=1;
            } else {
                if t > TIMEOUT { // Convert to fault timer rather than else statement
                    return false;
                }
                t+=1;
            }
        }
        return true;
    }

    pub fn write_u8(&self, byte: u8) -> bool {
        let mut i = 0; 

        while !pointer::get_ptr_vol_bit_u32(self.isr, TXIS_BIT) {
            if i > TIMEOUT {
                return false;
            }
            i+=1;
        }
        pointer::set_ptr_vol_raw_u8(self.txdr, byte);
        return true;
    }

    pub fn std_write(&self, slave_addr: u32, addr_10bit: bool, req_10bit: bool, buf: &[u8]) {
        self.setup(slave_addr, addr_10bit, req_10bit, buf.len() as u32, WRITE);
        self.start();
        self.write(buf);
        self.tc();
        self.stop();
    }

    // PG. 1522-1523 (MATH TREE)
    fn set_timing_register(&self, scll: u32, sclh: u32, sdadel: u32, scldel: u32, presc: u32) {
        pointer::set_ptr_vol_u32(self.timingr, SCLL_OFFSET, SCL_MASK, scll);
        pointer::set_ptr_vol_u32(self.timingr, SCLH_OFFSET, SCL_MASK, sclh);
        pointer::set_ptr_vol_u32(self.timingr, SDADEL_OFFSET, DEL_MASK, sdadel);
        pointer::set_ptr_vol_u32(self.timingr, SCLDEL_OFFSET, DEL_MASK, scldel);  
        pointer::set_ptr_vol_u32(self.timingr, PRESC_OFFSET, DEL_MASK, presc);
    }
}
