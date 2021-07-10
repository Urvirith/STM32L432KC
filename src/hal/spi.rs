/* Serial Peripheral Interface */
/* Manual Page 1304 */

use super::{common, pointer};

/*
    SPI registers
    The peripheral registers can be accessed 
    by half-words (16-bit) or words (32-bit). 
    SPI_DR in addition can be accessed by 8-bit access
*/

pub struct Spi {
    cr1:                *mut u32,           // Control Register 1
    cr2:                *mut u32,           // Control Register 2
    sr:                 *mut u32,           // Status Register
    dr:                 *mut u8,            // Data Register
    crcpr:              *mut u16,           // CRC Polynomial Register
    rxcrcr:             *mut u16,           // Rx CRC Register
    txcrcr:             *mut u16            // Tx CRC Register
}

/* Register Offset */
const CR1:      u32 = 0x00;
const CR2:      u32 = 0x04;
const SR:       u32 = 0x08;
const DR:       u32 = 0x0C;
const CRCPR:    u32 = 0x10;
const RXCRCR:   u32 = 0x14;
const TXCRCR:   u32 = 0x18;

/* Enumerations */
// CPHA = 0 READS THE FIRST BIT ON RISING EDGE OF CLOCK, CPHA = 1 READS THE FIRST BIT ON FALLING EDGE OF CLOCK
// CPOL IS HOW THE CLOCK BEHAVES, CPOL = TRUE, CLOCK IS HIGH UNTIL USED, FALSE CLOCK IS LOW UNTILL USED
pub enum ClockSetup {RisingEdgeClockLow, FallingEdgeClockLow, RisingEdgeClockHigh, FallingEdgeClockHigh}

// Least Significant Bit First, Most Significiant Bit First
pub enum BitFirst {Lsb, Msb}

// BR: Baud Rate Control 000: fPCLK/2 001: fPCLK/4 010: fPCLK/8 011: fPCLK/16 100: fPCLK/32 101: fPCLK/64 110: fPCLK/128 111: fPCLK/256
pub enum BaudRateDiv {Clk2, Clk4, Clk8, Clk16, Clk32, Clk64, Clk128, Clk256}

// These bits configure the data length for SPI transfers.
// 0000: Not used 0001: Not used 0010: Not used 
// 0011: 4-bit 0100: 5-bit 0101: 6-bit
// 0110: 7-bit 0111: 8-bit 1000: 9-bit
// 1001: 10-bit 1010: 11-bit 1011: 12-bit
// 1100: 13-bit 1101: 14-bit 1110: 15-bit
// 1111: 16-bit
// If software attempts to write one of the “Not used” values, 
// they are forced to the value “0111”(8-bit)
pub enum DataSize {
    Bits4   = 0x03,
    Bits5   = 0x04,
    Bits6   = 0x05,
    Bits7   = 0x06,
    Bits8   = 0x07,
    Bits9   = 0x08,
    Bits10  = 0x09,
    Bits11  = 0x0A,
    Bits12  = 0x0B,
    Bits13  = 0x0C,
    Bits14  = 0x0D,
    Bits15  = 0x0E,
    Bits16  = 0x0F
}

/* Register Bits */
/* CR1 */
const CPHA_BIT:         u32 = common::BIT_0;
const CPOL_BIT:         u32 = common::BIT_1;
const MSTR_BIT:         u32 = common::BIT_2;
const SPE_BIT:          u32 = common::BIT_6;
const LSBFIRST_BIT:     u32 = common::BIT_7;
const SSI_BIT:          u32 = common::BIT_8;
const SSM_BIT:          u32 = common::BIT_9;
const CRCL_BIT:         u32 = common::BIT_11;
const CRCNEXT_BIT:      u32 = common::BIT_12;
const CRCEN_BIT:        u32 = common::BIT_13;

/* SR */
const RXNE_BIT:         u32 = common::BIT_0;
const TXE_BIT:          u32 = common::BIT_1;
const CRCERR_BIT:       u32 = common::BIT_4;
const MODF_BIT:         u32 = common::BIT_5;
const OVR_BIT:          u32 = common::BIT_6;
const BSY_BIT:          u32 = common::BIT_7;
const FRE_BIT:          u32 = common::BIT_8;


/* Register Masks */
/* CR1 */
const BR_MASK:          u32 = common::MASK_3_BIT;

/* CR2 */
const DS_MASK:          u32 = common::MASK_4_BIT;

/* SR */
const FRLVL_MASK:       u32 = common::MASK_2_BIT;
const FTLVL_MASK:       u32 = common::MASK_2_BIT;

/* Register Offsets */
/* CR1 */
const BR_OFFSET:        u32 = 3;                        /* Mode is two bits wide, shift by an offset of 2 */

/* CR2 */
const DS_OFFSET:        u32 = 8;

/* SR */
const FRLVL_OFFSET:     u32 = 9;
const FTLVL_OFFSET:     u32 = 11;

const TIMEOUT:          u32 = common::WAIT100US;


impl Spi {
    /* Initialize The Structure */
    pub fn init(base: u32) -> Spi {
        return Spi {
            cr1:        (base + CR1)        as *mut u32,
            cr2:        (base + CR2)        as *mut u32,
            sr:         (base + SR)         as *mut u32,
            dr:         (base + DR)         as *mut u8,
            crcpr:      (base + CRCPR)      as *mut u16,
            rxcrcr:     (base + RXCRCR)     as *mut u16,
            txcrcr:     (base + TXCRCR)     as *mut u16
        };
    }
    /* SPI Setup */
    // 1.Write proper GPIO registers: Configure GPIO for MOSI, MISO and SCK pins.
    // 2.     Write to the SPI_CR1 register:
    //      a)    Configure the serial clock baud rate using the BR[2:0] bits (Note: 4).
    //      b)    Configure the CPOL and CPHA bits combination to define one of the 
    //            four relationships between the data transfer and the serial clock 
    //            (CPHA must be cleared in NSSP mode). 
    //            (Note: 2 - except the case when CRC is enabled at TI mode).
    //      c)    Select simplex or half-duplex mode by configuring RXONLY or BIDIMODE and BIDIOE 
    //            (RXONLY and BIDIMODE can't be set at the same time).
    //      d)    Configure the LSBFIRST bit to define the frame format (Note: 2).
    //      e)    Configure the CRCL and CRCEN bits if CRC is needed 
    //            (while SCK clock signal is at idle state).
    //      f)    Configure SSM and SSI (Notes: 2 & 3).
    //      g)    Configure the MSTR bit (in multimaster NSS configuration, avoid conflict state on 
    //            NSS if master is configured to prevent MODF error).
    // 3.     Write to SPI_CR2 register:
    //      a)    Configure the DS[3:0] bits to select the data length for the transfer.
    //      b)    Configure SSOE (Notes: 1 & 2 & 3).
    //      c)    Set the FRF bit if the TI protocol is required (keep NSSP bit cleared in TI mode).
    //      d)    Set the NSSP bit if the NSS pulse mode between two data units is required 
    //            (keep CHPA and TI bits cleared in NSSP mode).
    //      e)    Configure the FRXTH bit. The RXFIFO threshold must be aligned to the 
    //            read access size for the SPIx_DR register.
    //      f)    Initialize LDMA_TX and LDMA_RX bits if DMA is used in packed mode.
    // 4.     Write to SPI_CRCPR register: Configure the CRC polynomial if needed.
    pub fn open(&self, br: BaudRateDiv, cs: ClockSetup, bit: BitFirst, ds: DataSize) {
        pointer::set_ptr_vol_u32(self.cr1, BR_OFFSET, BR_MASK, br as u32);

        match cs {                 // WILL BE COVERED BY THE DEVICES (Example nRF8001 is SCK LOW -> CPOL IS FALSE)                 
            ClockSetup::RisingEdgeClockLow => {
                pointer::clr_ptr_vol_bit_u32(self.cr1, CPHA_BIT);
                pointer::clr_ptr_vol_bit_u32(self.cr1, CPOL_BIT);
            } ClockSetup::FallingEdgeClockLow => {
                pointer::set_ptr_vol_bit_u32(self.cr1, CPHA_BIT);
                pointer::clr_ptr_vol_bit_u32(self.cr1, CPOL_BIT);
            } ClockSetup::RisingEdgeClockHigh => {
                pointer::clr_ptr_vol_bit_u32(self.cr1, CPHA_BIT);
                pointer::set_ptr_vol_bit_u32(self.cr1, CPOL_BIT);
            } ClockSetup::FallingEdgeClockHigh => {
                pointer::set_ptr_vol_bit_u32(self.cr1, CPHA_BIT);
                pointer::set_ptr_vol_bit_u32(self.cr1, CPOL_BIT);
            }
        }

        // Flow control needed if Unidirection (Half Duplex Comms)
        //self.ioctl_set_cr1(BIDIOE); 
        //self.ioctl_clr_cr1(BIDIMODE);

        match bit {
            BitFirst::Lsb => pointer::set_ptr_vol_bit_u32(self.cr1, LSBFIRST_BIT),
            BitFirst::Msb => pointer::clr_ptr_vol_bit_u32(self.cr1, LSBFIRST_BIT)
        }

        // MIGHT REQIRE PROGRAMMING LATER CURRENLY SET UP FOR CRC 8 BIT
        pointer::set_ptr_vol_bit_u32(self.cr1, CRCL_BIT);
        pointer::set_ptr_vol_bit_u32(self.cr1, CRCEN_BIT);

        // MIGHT REQIRE PROGRAMMING LATER CURRENLY SET UP HARDWARE SLAVE MANAGEMENT / PG. 1078 is for the Master with multiple slaves
        pointer::set_ptr_vol_bit_u32(self.cr1, SSI_BIT);
        pointer::set_ptr_vol_bit_u32(self.cr1, SSM_BIT);

        // SET MASTER MODE
        pointer::set_ptr_vol_bit_u32(self.cr1, MSTR_BIT);

        // DATA SIZE
        pointer::set_ptr_vol_u32(self.cr2, DS_OFFSET, DS_MASK, ds as u32);

        /*
        // CURRENTLY SET UP FOR MULTI MASTER MODE CONFIGURATION
        self.ioctl_clr_cr2(SSOE);

        match mode {
            0 => {
                // NSSP MODE
                self.ioctl_clr_cr1(CPHA);
                self.ioctl_clr_cr2(FRF);
                self.ioctl_set_cr2(NSSP);
            }
            1 => {
                // TI MODE
                self.ioctl_clr_cr2(NSSP);
                self.ioctl_set_cr2(FRF);
            }
            _ => {
                // SOMETHING MIGHT NEED TO BE PLACED HERE???
            }
        }
        */

        //self.ioctl_set_crcpr(crc_poly);
    }

    //  The master at full-duplex (or in any transmit-only mode) starts to communicate when the SPI is enabled and TXFIFO is not empty,
    //  or with the next write to TXFIFO. In any master receive only mode (RXONLY=1 or BIDIMODE=1 & BIDIOE=0),
    //  master starts to communicate and the clock starts running immediately after SPI is enabled. For handling DMA,
    //  follow the dedicated section.
    pub fn enable(&self) {
        pointer::set_ptr_vol_bit_u32(self.cr1, SPE_BIT);
    }

    pub fn read(&self, buf: &mut [u8], len: usize) -> usize {     // Return true if error occured
        let mut i = 0;
        let mut f = 0; // CONVERT TO FAULT TIMER

        while pointer::get_ptr_vol_bit_u32(self.sr, RXNE_BIT) {
            if i < buf.len() {
                buf[i] = pointer::get_ptr_vol_raw_u8(self.dr); // Will need to be changed if handling 16 bit words etc
                i += 1;
            } // Possible Need To Put Else Return here....

            // Return if i = len and len is set, possible to return terminating char if needed or pin pointer
            if ((i >= len) && (len > 0)) || i >= buf.len() { 
                return i;
            }

            if self.error() {
                //return (self.error_byte() + 0x10) as usize;
            }

            if (len > 0) && (i < len) {
                while !pointer::get_ptr_vol_bit_u32(self.sr, RXNE_BIT) {
                    // SPIN WHILE THE RXNE IS NOT EMPTY, DUMP OUT IF ISSUE COMES UP
                    if f > TIMEOUT {
                        return 0;
                    }
                    f+=1;
                }
            }
        }
        return i;
    }

    /* Write Function */
    pub fn write(&self, buf: &[u8]) -> u8 {  // Return true if error occured
        let mut i = 0;

        while i < buf.len() {
            if pointer::get_ptr_vol_u32(self.sr, FTLVL_OFFSET, FTLVL_MASK) != 3 {
                pointer::set_ptr_vol_raw_u8(self.dr, buf[i]);
                i += 1;
            }

            if self.error_byte() > 0 {
                //return self.error_byte();
            }

        }

        pointer::set_ptr_vol_bit_u32(self.cr1, CRCNEXT_BIT);

        return 0;
    }

    /* Write A Single Byte Of Data */
    pub fn write_byte(&self, buf: u8) -> bool {
        let mut i = 0;
        // Wait For FIFO To Free Before Writing Data To The Buffer
        while !pointer::get_ptr_vol_bit_u32(self.sr, TXE_BIT) {
            if i > TIMEOUT {
                return false;
            }
            i+=1;
        }

        pointer::set_ptr_vol_raw_u8(self.dr, buf);

        return true;
    }

    //  The correct disable procedure is (except when receive only mode is used):
    //      1. Wait until FTLVL[1:0] = 00 (no more data to transmit).
    //      2. Wait until BSY=0 (the last data frame is processed).
    //      3. Disable the SPI (SPE=0).
    //      4. Read data until FRLVL[1:0] = 00 (read all the received data).
    //  The correct disable procedure for certain receive only modes is:
    //      1. Interrupt the receive flow by disabling SPI (SPE=0) in the specific time window while the last data frame is ongoing.
    //      2. Wait until BSY=0 (the last data frame is processed).
    //      3. Read data until FRLVL[1:0] = 00 (read all the received data
    pub fn disable(&self) -> bool {     // Return true if error occured
        while pointer::get_ptr_vol_u32(self.sr, FTLVL_OFFSET, FTLVL_MASK) != 0 {
            /*
            if self.error() {
                pointer::clr_ptr_vol_bit_u32(self.cr1, SPE_BIT);
                return false;
            }
            */
        }

        while pointer::get_ptr_vol_bit_u32(self.sr, BSY_BIT) {
            /*
            if self.error() {
                pointer::clr_ptr_vol_bit_u32(self.cr1, SPE_BIT);
                return false;
            }
            */
        }

        pointer::clr_ptr_vol_bit_u32(self.cr1, SPE_BIT);

        return true; // IMPLEMENTATION OF TIMEOUT MIGHT BE NESSICARY
    }

    fn error(&self) -> bool { // RETURN ONE OF THE THREE ERRORS
        if pointer::get_ptr_vol_bit_u32(self.sr, CRCERR_BIT) || pointer::get_ptr_vol_bit_u32(self.sr, MODF_BIT) || pointer::get_ptr_vol_bit_u32(self.sr, OVR_BIT) {
            return true;
        } else {
            return false;  
        }
    }

    fn error_byte(&self) -> u8 { // RETURN ONE OF THE THREE ERRORS
        if pointer::get_ptr_vol_bit_u32(self.sr, CRCERR_BIT) { 
            return 1;
        } else if pointer::get_ptr_vol_bit_u32(self.sr, MODF_BIT) {
            return 2;
        } else if pointer::get_ptr_vol_bit_u32(self.sr, OVR_BIT) {
            return 3;
        } else {
            return 0;  
        }
    }
}
