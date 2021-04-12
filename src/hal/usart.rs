/* USART (Universal Synchronous and Asynchronous Receiver Transmitter) */
/* Manual Page 1194 */

use super::{common, pointer};

// Any USART bidirectional communication requires a minimum of two pins: Receive data In (RX) and Transmit data Out (TX):
// • RX: Receive data Input. This is the serial data input.
//   Oversampling techniques are used for data recovery by discriminating between valid incoming data and noise.
// • TX: Transmit data Output. When the transmitter is disabled, the output pin returns to its I/O port configuration.
//   When the transmitter is enabled and nothing is to be transmitted, the TX pin is at high level.
//   In Single-wire and Smartcard modes, this I/O is used to transmit and receive the data

// Serial data are transmitted and received through these pins in normal USART mode. The frames are comprised of:
// • An Idle Line prior to transmission or reception
// • A start bit • A data word (7, 8 or 9 bits) least significant bit first
// • 0.5, 1, 1.5, 2 stop bits indicating that the frame is complete
// • The USART interface uses a baud rate generator • A status register (USART_ISR)
// • Receive and transmit data registers (USART_RDR, USART_TDR)
// • A baud rate register (USART_BRR)
// • A guard-time register (USART_GTPR) in case of Smartcard mode.

// Refer to Section 38.8: USART registers on page 1238 for the definitions of each bit.
// The following pin is required to interface in synchronous mode and Smartcard mode:
// • CK: Clock output. This pin outputs the transmitter data clock for synchronous transmission corresponding to SPI master mode
//  (no clock pulses on start bit and stop bit, and a software option to send a clock pulse on the last data bit).
//  In parallel, data can be received synchronously on RX. This can be used to control peripherals that have shift registers.
//  The clock phase and polarity are software programmable. In Smartcard mode, CK output can provide the clock to the smartcard.

// The following pins are required in RS232 Hardware flow control mode:
// • CTS: Clear To Send blocks the data transmission at the end of the current transfer when high
// • RTS: Request to send indicates that the USART is ready to receive data (when low).
// The following pin is required in RS485 Hardware control mode:
// • DE: Driver Enable activates the transmission mode of the external transceiver.

pub struct Usart {
    cr1:        *mut u32,       // Control Register 1
    cr2:        *mut u32,       // Control Register 2
    cr3:        *mut u32,       // Control Register 3
    brr:        *mut u32,       // Baud Rate Register Register
    gtpr:       *mut u32,       // Guard Time / Prescaler Register
    rtor:       *mut u32,       // Receiver Timeout Register
    rqr:        *mut u32,       // Request Register
    isr:        *mut u32,       // Interrupt And Status Register
    icr:        *mut u32,       // Interrupt Flag Clear Register
    rdr:        *mut u8,        // Receive Data Register
    tdr:        *mut u8,        // Transmit Data Register
}

/* Register Offset */
const CR1:      u32 = 0x00;
const CR2:      u32 = 0x04;
const CR3:      u32 = 0x08;
const BRR:      u32 = 0x0C;
const GTPR:     u32 = 0x10;
const RTOR:     u32 = 0x14;
const RQR:      u32 = 0x18;
const ISR:      u32 = 0x1C;
const ICR:      u32 = 0x20;
const RDR:      u32 = 0x24;
const TDR:      u32 = 0x28;

/* Enumerations */
/* Oversample Size, 16-Bits Oversample, 8-Bits Oversample */
pub enum OverSample {Oversample16, Oversample8}

/* Word Length 0 = 8-Bits, 1 = 9-Bits 2 = 7-Bits */
pub enum WordLen {Bits8, Bits9, Bits7}

/* Stop Bit Length, 0 = 1-Bit, 1 = 0.5 Bit, 2 = 2 Bit, 3 = 1.5 Bit */
pub enum StopLen {StopBit1, StopBit05, StopBit2, StopBit15}

/* Baud Rates */
pub enum BaudRate {
    Baud1200    = 1200, 
    Baud1800    = 1800,
    Baud2400    = 2400, 
    Baud4800    = 4800,
    Baud9600    = 9600, 
    Baud19200   = 19200,
    Baud28800   = 28800, 
    Baud38400   = 38400,
    Baud57600   = 57600, 
    Baud76800   = 76800,
    Baud115200  = 115200, 
    Baud230400  = 230400,
    Baud460800  = 460800, 
    Baud576000  = 576000,
    Baud921600  = 921600 
}

const SCLK_HZ:          u32 = 1000;
const RTO_TIMEO_VALUE:  u32 = 10;

/* Register Masks */
/* CR1 */
const OVER8_MASK:       u32 = common::MASK_4_BIT;
/* CR2 */
const STOP_BIT_MASK:    u32 = common::MASK_2_BIT;
/* BRR */
const BAUD_RATE_MASK:   u32 = common::MASK_16_BIT;
/* RTOR */
const RTO_TIMEO_MASK:   u32 = common::MASK_24_BIT;

/* Register Bits */
/* CR1 */
const UE_BIT:           u32 = common::BIT_0;
const RE_BIT:           u32 = common::BIT_2;
const TE_BIT:           u32 = common::BIT_3;
const OVER_8_BIT:       u32 = common::BIT_15;
/* CR2 */
const RTOEN_BIT:        u32 = common::BIT_23;
/* ICR */
const FECF_BIT:         u32 = common::BIT_1;
const ORECR_BIT:        u32 = common::BIT_3;
const IDLECF_BIT:       u32 = common::BIT_4;
const RTOCF_BIT:        u32 = common::BIT_11;
/* ISR */
const FE_BIT:           u32 = common::BIT_1;
const ORE_BIT:          u32 = common::BIT_3;
const IDLE_BIT:         u32 = common::BIT_4;
const RXNE_BIT:         u32 = common::BIT_5;
const TC_BIT:           u32 = common::BIT_6;
const TXE_BIT:          u32 = common::BIT_7;

/* Register Offsets */
/* CR2 */
const STOP_BIT_OFFSET:  u32 = 12;
/* BRR */
const BAUD_RATE_OFFSET: u32 = 0;
/* RTOR */
const RTO_TIMEO_OFFSET: u32 = 0;

/* Register Shifts */
/* CR1 */
const OVER8_SHIFT:      u32 = 1;

impl Usart {
    /* Initialize The Structure */
    pub fn init(base: u32) -> Usart {
        return Usart {
            cr1:    (base + CR1)    as *mut u32,
            cr2:    (base + CR2)    as *mut u32,
            cr3:    (base + CR3)    as *mut u32,
            brr:    (base + BRR)    as *mut u32,
            gtpr:   (base + GTPR)   as *mut u32,
            rtor:   (base + RTOR)   as *mut u32,
            rqr:    (base + RQR)    as *mut u32,
            isr:    (base + ISR)    as *mut u32,
            icr:    (base + ICR)    as *mut u32,
            rdr:    (base + RDR)    as *mut u8,
            tdr:    (base + TDR)    as *mut u8
        }
    }

    /* Open The USART Driver, Set Word Length, Baud Rate, Oversample */
    pub fn open(&self, word_len: WordLen, stop: StopLen, baud: BaudRate, sclk_khz: u32, samp: OverSample) { 
        match word_len {
            WordLen::Bits8 => {
                pointer::clr_ptr_vol_bit_u32(self.cr1, common::BIT_12);
                pointer::clr_ptr_vol_bit_u32(self.cr1, common::BIT_24);
            },
            WordLen::Bits9 => {
                pointer::set_ptr_vol_bit_u32(self.cr1, common::BIT_12);
                pointer::clr_ptr_vol_bit_u32(self.cr1, common::BIT_24);
            },
            WordLen::Bits7 => {
                pointer::clr_ptr_vol_bit_u32(self.cr1, common::BIT_12);
                pointer::set_ptr_vol_bit_u32(self.cr1, common::BIT_24);
            }
        }

        match samp {
            OverSample::Oversample8     =>  pointer::set_ptr_vol_bit_u32(self.cr1, OVER_8_BIT),
            OverSample::Oversample16    =>  pointer::clr_ptr_vol_bit_u32(self.cr1, OVER_8_BIT)
        }
        
        pointer::set_ptr_vol_bit_u32(self.cr2, RTOEN_BIT);
        pointer::set_ptr_vol_u32(self.brr, BAUD_RATE_OFFSET, BAUD_RATE_MASK, self.clock_setup(baud, sclk_khz, samp));
        pointer::set_ptr_vol_u32(self.cr2, STOP_BIT_OFFSET, STOP_BIT_MASK, stop as u32);
        pointer::set_ptr_vol_bit_u32(self.cr1, UE_BIT);
        pointer::set_ptr_vol_bit_u32(self.cr1, RE_BIT);
        pointer::set_ptr_vol_u32(self.rtor, RTO_TIMEO_OFFSET, RTO_TIMEO_MASK, RTO_TIMEO_VALUE);
    }
    
    
    // p. 1205
    // Character reception procedure
    // 1. Program the M bits in USART_CR1 to define the word length.
    // 2. Select the desired baud rate using the baud rate register USART_BRR
    // 3. Program the number of stop bits in USART_CR2.
    // 4. Enable the USART by writing the UE bit in USART_CR1 register to 1.
    // 5. Select DMA enable (DMAR) in USART_CR3 if multibuffer communication is to take place.
    //    Configure the DMA register as explained in multibuffer communication.
    // 6. Set the RE bit USART_CR1.
    //    This enables the receiver which begins searching for a start bit.
    
    // When a character is received:
    // • The RXNE bit is set to indicate that the content of the shift register is transferred to the RDR.
    //   In other words, data has been received and can be read (as well as its associated error flags).
    // • An interrupt is generated if the RXNEIE bit is set.
    // • The error flags can be set if a frame error, noise or an overrun error has been detected during reception. PE flag can also be set with RXNE.
    // • In multibuffer, RXNE is set after every byte received and is cleared by the DMA read of the Receive data Register.
    // • In single buffer mode, clearing the RXNE bit is performed by a software read to the USART_RDR register.
    //   The RXNE flag can also be cleared by writing 1 to the RXFRQ in the USART_RQR register.
    //   The RXNE bit must be cleared before the end of the reception of the next character to avoid an overrun error.
    pub fn get_read(&self) -> bool{
        return pointer::get_ptr_vol_bit_u32(self.isr, RXNE_BIT);
    }
    
    /* Read From The RX Register */
    pub fn read(&self, buf: &mut [u8], term: u8) -> i32{
        pointer::set_ptr_vol_raw_u32(self.icr, RTOCF_BIT);
        pointer::set_ptr_vol_raw_u32(self.icr, FECF_BIT);
        pointer::set_ptr_vol_raw_u32(self.icr, IDLECF_BIT);
        pointer::set_ptr_vol_raw_u32(self.icr, ORECR_BIT);
    
        let mut i = 0; // Index based on len
        let mut t: u32 = 0;  // Index for loop trap, if line goes idle, prevent being trapped by dead line. Convert to fail timer for more accurate usage.
    
        while i < buf.len(){
            if self.get_read() {
                buf[i] = pointer::get_ptr_vol_raw_u8(self.rdr);
                if (term != 0x00) && (buf[i] == term){
                    return (i + 1) as i32;
                }
                i+=1;
                t=0;
            } else {
                if t > 100000 { // IDLED OUT
                    return -2;
                }
                t+=1;
            }
    
            if pointer::get_ptr_vol_bit_u32(self.isr, FE_BIT) | pointer::get_ptr_vol_bit_u32(self.isr, IDLE_BIT) | pointer::get_ptr_vol_bit_u32(self.isr, ORE_BIT) {
                return -1;
            }
        }
        return i as i32;
    }
    
    /*
        p. 1202
        Character transmission procedure
        1. Program the M bits in USART_CR1 to define the word length.
        2. Select the desired baud rate using the USART_BRR register.
        3. Program the number of stop bits in USART_CR2.
        4. Enable the USART by writing the UE bit in USART_CR1 register to 1.
        5. Select DMA enable (DMAT) in USART_CR3 if multibuffer communication is to take place.
           Configure the DMA register as explained in multibuffer communication.
        6. Set the TE bit in USART_CR1 to send an idle frame as first transmission.
        7. Write the data to send in the USART_TDR register (this clears the TXE bit).
           Repeat this for each data to be transmitted in case of single buffer.
        8. After writing the last data into the USART_TDR register, wait until TC=1.
           This indicates that the transmission of the last frame is complete.
           This is required for instance when the USART is disabled or enters the Halt mode to avoid corrupting the last transmission.
    */
    /* Write To The TX Register */
    pub fn write(&self, buf: &[u8]) {
        let mut i = 0;
        pointer::set_ptr_vol_bit_u32(self.cr1, TE_BIT);
        
        while i < buf.len(){
            if pointer::get_ptr_vol_bit_u32(self.isr, TXE_BIT) {
                pointer::set_ptr_vol_raw_u8(self.tdr, buf[i]);
                i+=1;
            }
        }
    
        while pointer::get_ptr_vol_bit_u32(self.isr, TC_BIT) == false {
            // SPIN TO WAIT UNTILL THE BIT IS COMPLETE#
        }
    
        pointer::clr_ptr_vol_bit_u32(self.cr1, TE_BIT);
    }
    
    /*
        USARTDIV is an unsigned fixed point number that is coded on the USART_BRR register.
        • When OVER8 = 0, BRR = USARTDIV.
        • When OVER8 = 1 – BRR[2:0] = USARTDIV[3:0] shifted 1 bit to the right.
        – BRR[3] must be kept cleared.
        – BRR[15:4] = USARTDIV[15:4]
    */
    fn clock_setup(&self, baud: BaudRate, sclk_khz: u32, samp: OverSample) -> u32 {
        match samp {
            OverSample::Oversample8     =>  {
                let baud_div = ((sclk_khz * SCLK_HZ) * 2) / baud as u32;

                return (baud_div & (!OVER8_MASK)) & ((baud_div & OVER8_MASK) >> OVER8_SHIFT)
            },
            OverSample::Oversample16    =>  return (sclk_khz * SCLK_HZ) / baud as u32
        };
    }
}
