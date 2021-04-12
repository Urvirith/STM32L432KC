/* Serial Peripheral Interface */
/* Manual Page 1304 */

use super::common;
use super::pointer;
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
// 00: Input mode     01: General purpose output mode     10: Alternate function mode     11: Analog mode (reset state)
pub enum Mode {In, Out, Alt, An}

/* Register Bits */
/* CR1 */
const SPE_BIT:          u32 = common::BIT_6;
const CRCNEXT_BIT:      u32 = common::BIT_12;

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

/* SR */
const FRLVL_MASK:       u32 = common::MASK_2_BIT;
const FTLVL_MASK:       u32 = common::MASK_2_BIT;

/* Register Offsets */
/* CR1 */
const BR_OFFSET:        u32 = 3;                        /* Mode is two bits wide, shift by an offset of 2 */

/* SR */
const FRLVL_OFFSET:     u32 = 9;
const FTLVL_OFFSET:     u32 = 11;


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
    pub fn open(&self) {

    }

    //  The master at full-duplex (or in any transmit-only mode) starts to communicate when the SPI is enabled and TXFIFO is not empty,
    //  or with the next write to TXFIFO. In any master receive only mode (RXONLY=1 or BIDIMODE=1 & BIDIOE=0),
    //  master starts to communicate and the clock starts running immediately after SPI is enabled. For handling DMA,
    //  follow the dedicated section.
    pub fn enable(&self) {
        pointer::set_ptr_vol_bit_u32(self.cr1, SPE_BIT);
    }

    pub fn read(&self, buf: &mut [u8]) -> usize {     // Return true if error occured
        let mut index = 0;

        while pointer::get_ptr_vol_u32(self.sr, FRLVL_OFFSET, FRLVL_MASK) != 0 {
            if index < buf.len() {
                buf[index] = pointer::get_ptr_vol_raw_u8(self.dr); // Will need to be changed if handling 16 bit words etc
                index += 1;
            } else {
                return 0x77;
            }

            if self.error() {
                //return (self.error_byte() + 0x10) as usize;
            }
        }
        return index;
    }

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
            if self.error() {
                pointer::clr_ptr_vol_bit_u32(self.cr1, SPE_BIT);
                return false;
            }
        }

        while pointer::get_ptr_vol_bit_u32(self.sr, BSY_BIT) {
            if self.error() {
                pointer::clr_ptr_vol_bit_u32(self.cr1, SPE_BIT);
                return false;
            }
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
