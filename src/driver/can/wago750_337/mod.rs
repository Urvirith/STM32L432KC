/* CAN (Controller Area Network) Wago I/O Device */
/* https://www.wago.com/us/controllers-bus-couplers-i-o/fieldbus-coupler-canopen/p/750-337#downloads */

/*
    750_337 Communication Module - CANOpen
    750_    Discrete Module PNP (24 Sensing)
    750_    Discrete Module NPN (24 Sourcing)
    750_    Analogue Module In  (4-20mA Sinking)
    750_    Analogue Module Out (4-20mA Sinking)
*/


use crate::hal::{can::CanMsg, can::Can};
use super::canopen::{CANOpen, sdo};

const GUARDTIME:        u16 = 0x100C; 
const LIFEFACTOR:       u16 = 0x100D;
const GTLTSI:           u8  = 0x00;
const BYTEMASK:         u8  = 0xFF;

const DIG8OUTPUTS:      u16 = 0x6200; 
const DIG8INPUTS:       u16 = 0x6000;
const SDOIOSI:          u8  = 0x01;

// Assuming if an NMT is not seen in 500 ms that Master is no longer active, and begin life guard event
const GUARDTIMEMS:      u16 = 50;
const LIFEFACTORMUL:    u8  = 10;

pub struct Wago750 {
    step:       u8,
    co_node: CANOpen
}

impl Wago750 {
    pub fn init(node: u32) -> Wago750 {
        return Wago750 {
            step:       0,
            co_node:    CANOpen::init(node)
        }
    }
    
    /* This is a pure custom implementation due to the nature of the flex I/O as the data packs itself dynamically */
    pub fn setup(&mut self, msg: &mut CanMsg, bus: &Can) {
        // Set Up Node Guarding / PDOs
        if bus.write_pend() {
            match self.step {
                0 => {      // Initialize The Guard Time
                    self.setup_guardtime(msg, bus, GUARDTIMEMS);
                } 1 => {    // Initialize The Life Factor Time
                    self.setup_lifefactor(msg, bus, LIFEFACTORMUL);
                } 2 => {    // Initialize The PDO Rx (Client Recieve) For Current Setup
    
                } _ => {
    
                }
            };
        }
    }
    
    /*Node Guarding is switched off by default because 0 is entered in the respective indices (0x100C = Guard-Time, 0x100D = Life Time Factor)*/
    /* 0x100C – Guard Time 
                This object specifies the Guard Time in milliseconds. 
                An NMT master requests the state of the NMT slave in a cyclical manner. 
                The time between two requests is the Guard Time. 
    */
    pub fn setup_guardtime(&mut self, msg: &mut CanMsg, bus: &Can, guardtime: u16) {
        // Set The Heartbeat Interval
        self.co_node.sdo_init_download(sdo::N::Bytes2, sdo::E::Expedited, GUARDTIME, GTLTSI, [((guardtime >> 0) as u8 & BYTEMASK), ((guardtime >> 8) as u8 & BYTEMASK), 0, 0], msg);
        // Write Heartbeat Interval
        bus.write(msg);
        // Increment Step
        self.step += 1;
    }

    /* 0x100D – Life Time Factor
                The Life Time Factor is part of the Node Guarding Protocol. The NMT slave
                checks whether it was queried within the Node Life Time (guard time multiplied
                by the Life Time Factor). If not, the slave must assume that the NMT master is no
                longer in normal operation. It then initiates a Life Guarding Event.
                If the Node Life Time is zero, there is no monitoring.
    */
    pub fn setup_lifefactor(&mut self, msg: &mut CanMsg, bus: &Can, lifefactortime: u8) {
        // Set The Heartbeat Interval
        self.co_node.sdo_init_download(sdo::N::Bytes3, sdo::E::Expedited, LIFEFACTOR, GTLTSI, [lifefactortime, 0, 0, 0], msg);
        // Write Heartbeat Interval
        bus.write(msg);
        // Increment Step
        self.step += 1;
    }

    /* P.100 Describes The Transmission Of A PDO */
    /* P.117 Describes The Default Mapping For PDO */
    /* P.117 Describes The Default Mapping For PDO */
    pub fn setup_pdo(&self, msg: &mut CanMsg, bus: &Can) {

    }

    pub fn read_node_guarding(&mut self, msg: &mut CanMsg) {
        // Generate The Node Guarding Request
        self.co_node.nmt_read_heartbeat(msg);
    }

    pub fn write_node_guarding(&mut self, msg: &mut CanMsg, bus: &Can) {
        // Generate The Node Guarding Request
        self.co_node.nmt_request_guarding(msg);
        // Write Node Guarding Request
        bus.write(msg);
    }
    
    pub fn test_outputs(&self, msg: &mut CanMsg, bus: &Can, ind: &isize) {
        self.co_node.sdo_init_download(sdo::N::Bytes3, sdo::E::Expedited, DIG8OUTPUTS, SDOIOSI, [1 << ind, 0, 0, 0], msg);
        bus.write(&msg);
    }

    pub fn test_request_inputs(&self, msg: &mut CanMsg, bus: &Can) {
        self.co_node.sdo_init_upload(DIG8INPUTS, SDOIOSI, msg);
        bus.write(&msg);
    }

    pub fn test_read_sdo(&self, msg: &mut CanMsg) -> [u8; 4] {
        let dogmeat = [0, 0, 0, 0]; // PLACEHOLDER
        //self.node.

        return dogmeat;
    }
}
