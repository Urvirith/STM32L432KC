/* General Purpose I/O */
/* Manual Page 235 */

use super::common;
use super::pointer;

pub struct Gpio {
    moder:              *mut u32,           // Mode Register
    otyper:             *mut u32,           // Output Type Register
    ospeedr:            *mut u32,           // Output Speed Register
    pupdr:              *mut u32,           // Pull up Pull Down Register
    idr:                *mut u32,           // Input Data Register
    odr:                *mut u32,           // Output Data Register
    bsrr:               *mut u32,           // Bit Set Reset Register
    lckr:               *mut u32,           // Configuration Lock Register
    afrl:               *mut u32,           // Alternate Function Low Register
    afrh:               *mut u32,           // Alternate Function High Register
    brr:                *mut u32,           // Bit Reset Register
}

/* Register Offset */
const MODER:    u32 = 0x00;
const OTYPER:   u32 = 0x04;
const OSPEEDR:  u32 = 0x08;
const PUPDR:    u32 = 0x0C;
const IDR:      u32 = 0x10;
const ODR:      u32 = 0x14;
const BSRR:     u32 = 0x18;
const LCKR:     u32 = 0x1C;
const AFRL:     u32 = 0x20;
const AFRH:     u32 = 0x24;
const BRR:      u32 = 0x28;

/* Enumerations */
// 00: Input mode     01: General purpose output mode     10: Alternate function mode     11: Analog mode (reset state)
pub enum Mode {In, Out, Alt, An}

// 0: Output Push Pull     1: Output Open Drain
pub enum OType {PushPull, OpenDrain}

// 00: Low speed     01: Medium speed     10: High speed     11: Very high speed
pub enum OSpeed {Low, Medium, High, VeryHigh}

// 00: No pull-up, pull-down     01: Pull-up     10: Pull-down     11: Reserved
pub enum Pupd {NoPuPd, Pu, Pd}

/* 0000: AF0     0001: AF1     0010: AF2     0011: AF3     0100: AF4     0101: AF5
   0110: AF6     0111: AF7     1000: AF8     1001: AF9     1010: AF10    1011: AF11
   1100: AF12    1101: AF13    1110: AF14    1111: AF15                             */
pub enum AltFunc {Af0, Af1, Af2, Af3, Af4, Af5, Af6, Af7, Af8, Af9, Af10, Af11, Af12, Af13, Af14, Af15}

/* Register Masks */
const MODER_MASK:       u32 = common::MASK_2_BIT;       /* Mode is mask required, here we set the mask to two bit 11 */
const OSPEED_MASK:      u32 = common::MASK_2_BIT;       /* Mode is mask required, here we set the mask to two bit 11 */
const PUPD_MASK:        u32 = common::MASK_2_BIT;       /* Mode is mask required, here we set the mask to two bit 11 */
const AF_MASK:          u32 = common::MASK_4_BIT;       /* Mode is mask required, here we set the mask to four bit 1111 */

/* Register Offsets */
const MODER_OFFSET:     u32 = 2;                        /* Mode is two bits wide, shift by an offset of 2 */
const OSPEED_OFFSET:    u32 = 2;                        /* Output Speed is two bits wide, shift by an offset of 2 */
const PUPD_OFFSET:      u32 = 2;                        /* Pu Speed is two bits wide, shift by an offset of 2 */
const AF_OFFSET:        u32 = 4;                        /* Alternate Function is four bits wide, shift by an offset of 4 */

impl Gpio {
    /* Initialize The Structure */
    pub fn init(base: u32) -> Gpio {
        return Gpio {
            moder:      (base + MODER)      as *mut u32,
            otyper:     (base + OTYPER)     as *mut u32,
            ospeedr:    (base + OSPEEDR)    as *mut u32,
            pupdr:      (base + PUPDR)      as *mut u32,
            idr:        (base + IDR)        as *mut u32,
            odr:        (base + ODR)        as *mut u32,
            bsrr:       (base + BSRR)       as *mut u32,
            lckr:       (base + LCKR)       as *mut u32,
            afrl:       (base + AFRL)       as *mut u32,
            afrh:       (base + AFRH)       as *mut u32,
            brr:        (base + BRR)        as *mut u32
        };
    }

    /* Get The Pin */
    pub fn get_pin(&self, val: u32) -> bool {
        return pointer::get_ptr_vol_bit_u32(self.idr, val);
    }

    /* Set The Pin */
    pub fn set_pin(&self, val: u32) {
        pointer::set_ptr_vol_bit_u32(self.odr, val);
    }

    /* Clear The Pin */
    pub fn clr_pin(&self, val: u32) {
        pointer::clr_ptr_vol_bit_u32(self.odr, val);
    }

    /* Set Lock */
    pub fn set_lock(&self, val: u32){
        pointer::set_ptr_vol_bit_u32(self.lckr, val);
    }
    
    /* Clear Lock */
    pub fn clr_lock(&self, val: u32){
        pointer::clr_ptr_vol_bit_u32(self.lckr, val);
    }

    /* Set Pin Type And Alternate Function */
    pub fn otype(&self, bit: u32, mode: Mode, otype: OType, alt_func: AltFunc) {
        let alt;
        match mode {
            Mode::Alt => alt = true,
            _ =>         alt = false
        }

        pointer::set_ptr_vol_u32(self.moder, bit * MODER_OFFSET, MODER_MASK, mode as u32);
        
        match otype {
            OType::OpenDrain    =>      pointer::set_ptr_vol_bit_u32(self.otyper, 1 << bit),
            OType::PushPull     =>      pointer::clr_ptr_vol_bit_u32(self.otyper, 1 << bit)
        }

        /* If Alternate Function */
        if alt {
            if bit <= 7 {
                pointer::set_ptr_vol_u32(self.afrl, bit * AF_OFFSET, AF_MASK, alt_func as u32);
            } else {
                pointer::set_ptr_vol_u32(self.afrh, (bit - 8) * AF_OFFSET, AF_MASK, alt_func as u32);
            } 
        }
    }
    
    /* Set Pin Speed */
    pub fn ospeed(&self, bit: u32, speed: OSpeed) {
        pointer::set_ptr_vol_u32(self.ospeedr, bit * OSPEED_OFFSET, OSPEED_MASK, speed as u32);
    }
    
    /* Set Pull Up Pull Down Mode */
    pub fn pupd(&self, bit: u32, mode: Pupd) {
        pointer::set_ptr_vol_u32(self.pupdr, bit * PUPD_OFFSET, PUPD_MASK, mode as u32);
    }
}
