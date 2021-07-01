/* https://www.nxp.com/docs/en/data-sheet/MFRC522.pdf */
/* Pg. 9 Describes Pin Setup */
/* 
    NSS is SDA          - Starts High, Pulled Low To Start Communication
    MISO                - Master In Slave Out
    MOSI                - Master Out Slave In
    SCK                 - Serial Clock  - Starts Low - Rising Intersects MSB (RisingEdgeClockLow, MSB First)
    IRQ                 - Driven High By Slave To Indicate Request For Communication 
    RST                 - Driven High To Reset And Enable Module, Low To Shut The Module Down
*/

use crate::hal::{common, spi::Spi};

/* Masks */
const ADDRESS_MASK:         u8 = 0x3F;
const ADDRESS_BYTE_MASK:    u8 = 0xFE;

/* Offsets */
const ADDRESS_OFFSET:       u8 = 1;
const READ_OFFSET:          u8 = 7;

struct Rc522 {
    step: u8,
    bus: Spi
}

impl Rc522 {
    /* MIGHT NOT BE THE BEST METHOD FOR HANDLING THE SPI BUS, MAYBE PASS BY REFERENCE????? */
    pub fn init(bus: Spi) -> Rc522 {
        return Rc522 {
            step: 0,
            bus: bus
        };
    }


    pub fn read(&self, addr: u8) {

    }

    pub fn write(&self, addr: u8, data: &[u8]) {

    }

    fn form_address(addr: u8, read: bool) -> u8 {
        let mut address = (addr & ADDRESS_MASK) << ADDRESS_OFFSET;

        if read {
            address |= 1 << READ_OFFSET; 
        } else {
            address &= !(1 << READ_OFFSET);
        }

        return address;
    }
}