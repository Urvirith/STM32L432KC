/* Network Management */
/* NMT Can Only Be Used As A Master */

use super::CANOpen;
use super::CANOpenState;
use crate::hal::{can::CanMsg, common};
use crate::driver::can::canopen;

/* Local Const To Form The Message */
const START:            u8 = 0x01;      // Start Remote Node
const STOP:             u8 = 0x02;      // Stop Remote Node
const PREOP:            u8 = 0x80;      // Pre-Operational Remote Node
const RESET:            u8 = 0x81;      // Reset Remote Node
const COMMS:            u8 = 0x82;      // Reset Communication Remote Node
const DLC_NMT:          u32 = 0x02;     // NMT Standard DLC
const DLC_HB:           u32 = 0x01;     // NMT Heartbeat DLC 
const CO_IDE:           bool = false;   // CANOpen supports 1 -127 nodes

const MASK:             u32 = common::MASK_7_BIT;
const SHIFT:            u32 = 7;

pub const HB:           u32 = 0x0700;   // Heartbeat / Node Guarding COB-ID

impl CANOpen {
    /* Start Remote Node */
    pub fn nmt_write_start(&self, node_id: u8, msg: &mut CanMsg) {
        msg.set_dlc(DLC_NMT);
        msg.set_data([START, node_id, 0, 0, 0, 0, 0, 0]);
    }

    /* Stop Remote Node */
    pub fn nmt_write_stop(&self, node_id: u8, msg: &mut CanMsg) {
        msg.set_dlc(DLC_NMT);
        msg.set_data([STOP, node_id, 0, 0, 0, 0, 0, 0]);
    }

    /* Pre-Operational Remote Node */
    pub fn nmt_write_preop(&self, node_id: u8, msg: &mut CanMsg) {
        msg.set_dlc(DLC_NMT);
        msg.set_data([PREOP, node_id, 0, 0, 0, 0, 0, 0]);
    }

    /* Reset Remote Node */
    pub fn nmt_write_reset(&self, node_id: u8, msg: &mut CanMsg) {
        msg.set_dlc(DLC_NMT);
        msg.set_data([RESET, node_id, 0, 0, 0, 0, 0, 0]);
    }
    
    /* Reset Communication Remote Node */
    pub fn nmt_write_comms(&self, node_id: u8, msg: &mut CanMsg) {
        msg.set_dlc(DLC_NMT);
        msg.set_data([COMMS, node_id, 0, 0, 0, 0, 0, 0]);
    }

    /* Heartbeat Consumer */
    pub fn nmt_read_heartbeat(&self, msg: &CanMsg) -> CANOpenState {
        return canopen::canopen_state(msg.get_data()[0]);
    }

    /* Heartbeat Producer */
    pub fn nmt_write_heartbeat(&self, node_id: u32, msg: &mut CanMsg) {
        msg.set_id(HB + node_id, CO_IDE);
        msg.set_dlc(DLC_HB);
        msg.set_data([canopen::canopen_state_val(self.state), 0, 0, 0, 0, 0, 0, 0]);
    }

    pub fn nmt_request_guarding(&mut self, node_id: u32, msg: &mut CanMsg) {       
        msg.set_id(HB + node_id, CO_IDE);
        msg.set_rtr();
    }

    pub fn nmt_write_guarding(&mut self, node_id: u32, msg: &mut CanMsg) {
        let mut byte = 0;

        if self.toggle {
            byte = 1 << SHIFT;
            self.toggle = false;
        } else {
            byte = 0;
            self.toggle = true;
        }
        
        msg.set_id(HB + node_id, CO_IDE);
        msg.set_data([canopen::canopen_state_val(self.state) | byte, 0, 0, 0, 0, 0, 0, 0]);
    }
}