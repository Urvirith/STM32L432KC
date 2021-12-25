/* CAN (Controller Area Network) Wago I/O Device */
/* https://www.wago.com/us/controllers-bus-couplers-i-o/fieldbus-coupler-canopen/p/750-337#downloads */

/*
    750_337 Communication Module - CANOpen
    750_403 Discrete Module In PNP (24v Sensing)
    750_408 Discrete Module In NPN (24v Sourcing)
    750_504 Discrete Module Out (24v Sourcing)
    750_504 Discrete Module Out (24v Sourcing)
    750_554 Analogue Module Out (4-20mA Sinking)
    750_454 Analogue Module In  (4-20mA Sinking)
*/


use crate::stm32hal::{can::CanMsg, can::Can, common};
use super::canopen::{CANOpen};
use super::canopen;
pub mod setup;

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
    
    pub fn read_node_guarding(&mut self, msg: CanMsg) -> u8 {
        // Generate The Node Guarding Request
        return self.co_node.nmt_read_heartbeat(&msg);
    }

    pub fn write_node_guarding(&self, bus: &Can) {
        let mut msg = CanMsg::init();
        self.co_node.nmt_request_guarding(&mut msg);
        bus.write(msg);
    }
    
    pub fn write_mapped_outputs(&self, data: [u8; 8], bus: &Can) {
        // This is highly subjective based on the data going in and out
        let mut msg = CanMsg::init();
        self.co_node.pdo_write(canopen::RPDO1, 5, data, &mut msg);
        bus.write(msg);
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
