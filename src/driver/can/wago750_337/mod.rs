/* CAN (Controller Area Network) Wago I/O Device */
/* https://www.wago.com/us/controllers-bus-couplers-i-o/fieldbus-coupler-canopen/p/750-337#downloads */

/*
    750_337 Communication Module - CANOpen
    750_403 Discrete Module In PNP (24v Sensing)
    750_408 Discrete Module In NPN (24v Sourcing)
    750_504 Discrete Module Out (24v Sourcing)
    750_504 Discrete Module Out (24v Sourcing)
    750_454 Analogue Module In  (4-20mA Sinking)
    750_554 Analogue Module Out (4-20mA Sinking)
*/


use crate::hal::{can::CanMsg, can::Can, common};
use super::canopen::{CANOpen, sdo};
use super::canopen;

const GUARDTIME:        u16 = 0x100C; 
const LIFEFACTOR:       u16 = 0x100D;
const GTLTSI:           u8  = 0x00;
const BYTEMASK:         u8  = 0xFF;

const DIG8OUTPUTS:      u16 = 0x6200; 
const DIG8INPUTS:       u16 = 0x6000;
const SDOIOSI:          u8  = 0x01;

// Assuming if an NMT is not seen in 500 ms that Master is no longer active, and begin life guard event
const GUARDTIMEMS:      u16 = 1000;
const LIFEFACTORMUL:    u8  = 10;

// PDO MAPPING
/* RX Mapping PDO 1*/
const RPDO1:                u16 = 0x1600;
const RPDO1SINUMAPPED:      u8  = 0x03;     // 3 Mapped Mapped Objects (8 Bit Output, 1 Analogue, 2 Analogue)
const RPDO1DIGOUTPUTS:      u32 = 0x6200;   // Digital Outputs
const RPDO1DIGBITS:         u32 = 0x08;     // Number Of Bits In A Transfer - 8
const RPDO1DIG1SI:          u32 = 0x01;     // Sub Index For First 8 Digital Outputs
const RPDO1ANALOGOUT:       u32 = 0x6411;   // Analogue Outputs
const RPDO1ANALOGBITS:      u32 = 0x10;     // Number Of Bits In A Transfer - 16
const RPDO1ANALOGOUT1SI:    u32 = 0x01;     // Sub Index For First 1 Channel Analogue Output
const RPDO1ANALOGOUT2SI:    u32 = 0x02;     // Sub Index For First 2 Channel Analogue Output

/* TX Mapping PDO 1*/
const TPDO1:                u16 = 0x1A00;
const TPDO1SINUMAPPED:      u8  = 0x03;     // 3 Mapped Mapped Objects (8 Bit Output, 1 Analogue, 2 Analogue)
const TPDO1DIGINPUTS:       u32 = 0x6000;   // Digital Outputs
const TPDO1DIGBITS:         u32 = 0x08;     // Number Of Bits In A Transfer - 8
const TPDO1DIG1SI:          u32 = 0x01;     // Sub Index For First 8 Digital Outputs
const TPDO1ANALOGIN:        u32 = 0x6401;   // Analogue Outputs
const TPDO1ANALOGBITS:      u32 = 0x10;     // Number Of Bits In A Transfer - 16
const TPDO1ANALOGIN1SI:     u32 = 0x01;     // Sub Index For First 1 Channel Analogue Output
const TPDO1ANALOGIN2SI:     u32 = 0x02;     // Sub Index For First 1 Channel Analogue Output

pub struct Wago750 {
    step:       u8,
    setup:      bool, // Setup Complete
    co_node:    CANOpen
}

impl Wago750 {
    pub fn init(node: u32) -> Wago750 {
        return Wago750 {
            step:       0,
            setup:      false,
            co_node:    CANOpen::init(node)
        }
    }

    /* Handling Area For Incoming Messages For Wago */
    pub fn read_message(&mut self, msg: CanMsg) {
        match self.co_node.get_source_fc(msg.get_id()) {
            canopen::HEARTBEAT => {
                self.read_node_guarding(msg);
            } canopen::TPDO1 => {

            } canopen::TSDO => {

            } _ => {
                // Unknown Message, Do Not React
            }
        };
    }

    /* Check The State And Determine If It Needs To Be Set */
    pub fn set_state(&self, bus: &Can) {
        let mut msg = CanMsg::init();
        let send;

        match self.co_node.get_state() {
            canopen::CANOpenState::Bootup => {
                send = false;
            } canopen::CANOpenState::Stopped => {
                //self.co_node.nmt_write_preop(&mut msg);
                send = false;
            } canopen::CANOpenState::Operational => {
                if self.setup {
                    send = false;
                } else {
                    self.co_node.nmt_write_preop(&mut msg);
                    send = true;
                }
            } canopen::CANOpenState::PreOperational => {
                if self.setup {
                    self.co_node.nmt_write_start(&mut msg);
                    send = true;
                } else {
                    send = false;
                }
            } canopen::CANOpenState::Unknown => {
                send = false;
            }
        };
    
        if send {
            bus.write(msg);
        }
    }

    pub fn test_get_state(&self) -> u8 {
        return self.co_node.get_state_u8();
    }

    /* Start The Node */
    pub fn start_node(&self, bus: &Can) {
        let mut msg = CanMsg::init();
        /* NMT Write Start */
        self.co_node.nmt_write_start(&mut msg);
        // Write Heartbeat Interval
        bus.write(msg);
    }

    /* Set The Node To Preoperational */
    pub fn preop_node(&self, bus: &Can) {
        let mut msg = CanMsg::init();
        /* NMT Write Start */
        self.co_node.nmt_write_preop(&mut msg);
        // Write Heartbeat Interval
        bus.write(msg);
    }

    /* Start The Node */
    pub fn reset_node(&self, bus: &Can) {
        let mut msg = CanMsg::init();
        /* NMT Write Start */
        self.co_node.nmt_write_comms(&mut msg);
        // Write Heartbeat Interval
        bus.write(msg);
    }
    
    /* This is a pure custom implementation due to the nature of the flex I/O as the data packs itself dynamically */
    pub fn setup_wago(&mut self, bus: &Can) {
        // Set Up Node Guarding / PDOs
        match self.co_node.get_state() {
            canopen::CANOpenState::PreOperational => {
                if bus.write_free() {
                    match self.step {
                        0 => {      // Initialize The Guard Time
                            self.setup_guardtime(bus, GUARDTIMEMS);
                        } 1 => {    // Initialize The Life Factor Time
                            self.setup_lifefactor(bus, LIFEFACTORMUL);
                        } 2 => {    // Initialize The PDO Rx (Client Recieve) For Current Setup
                            self.setup_pdo_map_number(bus, RPDO1, 0, RPDO1SINUMAPPED);
                        } 3 => {    // Initialize The PDO Rx (Client Recieve) For Current Setup
                            self.setup_pdo_map(bus, RPDO1, 1, RPDO1DIGOUTPUTS, RPDO1DIG1SI, RPDO1DIGBITS);
                        } 4 => {    // Initialize The PDO Rx (Client Recieve) For Current Setup
                            self.setup_pdo_map(bus, RPDO1, 2, RPDO1ANALOGOUT, RPDO1ANALOGOUT1SI, RPDO1ANALOGBITS);
                        } 5 => {    // Initialize The PDO Rx (Client Recieve) For Current Setup
                            self.setup_pdo_map(bus, RPDO1, 3, RPDO1ANALOGOUT, RPDO1ANALOGOUT2SI, RPDO1ANALOGBITS);
                        } _ => {;
                            self.setup = true;
                        }
                    };
                }
            } _ => {
                if !self.setup {
                    self.preop_node(bus);
                } 
            }
        }
    }
    
    pub fn setup_complete(&self) -> bool {
        return self.setup;
    }
    /*Node Guarding is switched off by default because 0 is entered in the respective indices (0x100C = Guard-Time, 0x100D = Life Time Factor)*/
    /* 0x100C – Guard Time 
                This object specifies the Guard Time in milliseconds. 
                An NMT master requests the state of the NMT slave in a cyclical manner. 
                The time between two requests is the Guard Time. 
    */
    pub fn setup_guardtime(&mut self, bus: &Can, guardtime: u16) {
        let mut msg = CanMsg::init();
        // Set The Heartbeat Interval
        self.co_node.sdo_init_download(sdo::N::Bytes2, sdo::E::Expedited, GUARDTIME, GTLTSI, [((guardtime >> 0) as u8 & BYTEMASK), ((guardtime >> 8) as u8 & BYTEMASK), 0, 0], &mut msg);
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
    pub fn setup_lifefactor(&mut self, bus: &Can, lifefactortime: u8) {
        let mut msg = CanMsg::init();
        // Set The Heartbeat Interval
        self.co_node.sdo_init_download(sdo::N::Bytes3, sdo::E::Expedited, LIFEFACTOR, GTLTSI, [lifefactortime, 0, 0, 0], &mut msg);
        // Write Heartbeat Interval
        bus.write(msg);
        // Increment Step
        self.step += 1;
    }

    /* P.100 Describes The Transmission Of A PDO */
    /* P.117 Describes The Default Mapping For PDO */
    /* P.129 Describes The Default Mapping For PDO */
    /* P.168 Describes Transmit/ Recieve Of PDO */
    pub fn setup_pdo_map_number(&mut self, bus: &Can, index: u16, subindex: u8, map_num: u8) {
        let mut msg = CanMsg::init();
        // Set The Heartbeat Interval
        self.co_node.sdo_init_download(sdo::N::Bytes3, sdo::E::Expedited, index, subindex, [map_num, 0, 0, 0], &mut msg);
        // Write Heartbeat Interval
        bus.write(msg);
        // Increment Step
        self.step += 1;

    }
    
    pub fn setup_pdo_map(&mut self, bus: &Can, index: u16, subindex: u8, map_index: u32, map_subindex: u32, map_bits: u32) {
        let mut msg = CanMsg::init();
        // Set The Heartbeat Interval
        self.co_node.sdo_init_download(sdo::N::Bytes0, sdo::E::Expedited, index, subindex, self.pdo_mapping_structure(map_index, map_subindex, map_bits), &mut msg);
        // Write Heartbeat Interval
        bus.write(msg);
        // Increment Step
        self.step += 1;

    }

    pub fn read_node_guarding(&mut self, msg: CanMsg) -> u8 {
        // Generate The Node Guarding Request
        return self.co_node.nmt_read_heartbeat(&msg);
    }

    pub fn write_node_guarding(&self, bus: &Can) {
        let mut msg = CanMsg::init();
        // Generate The Node Guarding Request
        self.co_node.nmt_request_guarding(&mut msg);
        // Write Node Guarding Request
        bus.write(msg);
    }
    
    pub fn write_mapped_outputs(&self, data: [u8; 8], bus: &Can) {
        let mut msg = CanMsg::init();
        // This is highly subjective based on the data going in and out
        self.co_node.pdo_write(canopen::RPDO1, 5, data, &mut msg);
        bus.write(msg);
    }

    pub fn test_outputs(&self, bus: &Can, ind: &isize) {
        let mut msg = CanMsg::init();
        self.co_node.sdo_init_download(sdo::N::Bytes3, sdo::E::Expedited, DIG8OUTPUTS, SDOIOSI, [1 << ind, 0, 0, 0], &mut msg);
        bus.write(msg);
    }

    pub fn test_request_inputs(&self, bus: &Can) {
        let mut msg = CanMsg::init();
        self.co_node.sdo_init_upload(DIG8INPUTS, SDOIOSI, &mut msg);
        bus.write(msg);
    }

    pub fn test_read_sdo(&self, msg: CanMsg) -> [u8; 4] {
        let dogmeat = [0, 0, 0, 0]; // PLACEHOLDER
        //self.node.

        return dogmeat;
    }

    fn pdo_mapping_structure(&self, index: u32, subindex: u32, bit_len: u32) -> [u8; 4] {
        let mask = common::MASK_8_BIT as u8;
        let mut data = [0; 4];
        // For Mapping Parameter Found On p.130, consider distilling for quicker usage
        let map_para = ((index & common::MASK_16_BIT) << 16) | ((subindex & common::MASK_8_BIT) << 8) | ((bit_len & common::MASK_8_BIT) << 0);

        data[0] = (map_para >> 0) as u8 & mask;
        data[1] = (map_para >> 8) as u8 & mask;
        data[2] = (map_para >> 16) as u8 & mask;
        data[3] = (map_para >> 24) as u8 & mask;

        return data;
    }
}
