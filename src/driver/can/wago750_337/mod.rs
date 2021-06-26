/* CAN (Controller Area Network) Wago I/O Device */
/* https://www.wago.com/us/controllers-bus-couplers-i-o/fieldbus-coupler-canopen/p/750-337#downloads */

use crate::hal::{can::CanMsg, can::Can};
use super::canopen::{CANOpen, sdo};

//sdo_init_download(driver::can::canopen::sdo::N::Bytes3, driver::can::canopen::sdo::E::Expedited, 0x6200, 0x01, [1 << ind, 0x00, 0x00, 0x00], &mut msg);

pub fn get_state(node: &CANOpen, msg: &CanMsg, bus: &Can) {

}

/* This is a pure custom implementation due to the nature of the flex I/O as the data packs itself dynamically */
pub fn setup(node: &CANOpen, msg: &CanMsg, bus: &Can) {
    // Set Up Node Guarding
    
}

/*Node Guarding is switched off by default because 0 is entered in the respective indices (0x100C = Guard-Time, 0x100D = Life Time Factor)*/
/* 0x100C – Guard Time 
            This object specifies the Guard Time in milliseconds. 
            An NMT master requests the state of the NMT slave in a cyclical manner. 
            The time between two requests is the Guard Time. 
*/
/* 0x100D – Life Time Factor
            The Life Time Factor is part of the Node Guarding Protocol. The NMT slave
            checks whether it was queried within the Node Life Time (guard time multiplied
            by the Life Time Factor). If not, the slave must assume that the NMT master is no
            longer in normal operation. It then initiates a Life Guarding Event.
            If the Node Life Time is zero, there is no monitoring.
*/
pub fn setup_node_guarding(node: &CANOpen, msg: &CanMsg, bus: &Can, guardtime: u16, lifefactortime: u8) {
    //node.sdo_init_download(sdo::N::Bytes2, sdo::E::Expedited, );
}