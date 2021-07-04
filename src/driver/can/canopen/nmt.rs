/* Network Management */
/* NMT Can Only Be Used As A Master */

use super::CANOpen;
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
const DLC_RTR:          u32 = 0x00;     // NMT Heartbeat RTR  
const CO_IDE:           bool = false;   // CANOpen supports 1 -127 nodes

const HB_MASK:          u32 = common::MASK_1_BIT;
const MASK:             u32 = common::MASK_7_BIT;
const SHIFT:            u32 = 7;

pub const HB:           u32 = 0x0700;   // Heartbeat / Node Guarding COB-ID
pub const NMT_NODE:     u32 = 0x0000;   // All NMT Command Messages Will Not Have A Node In ID -> COB-ID = 0

impl CANOpen {
    /* Start Remote Node */
    pub fn nmt_write_start(&self, msg: &mut CanMsg) {
        msg.set_id(NMT_NODE, CO_IDE);
        msg.clr_rtr();
        msg.set_dlc(DLC_NMT);
        msg.set_data([START, self.node as u8 , 0, 0, 0, 0, 0, 0]);
    }

    /* Stop Remote Node */
    pub fn nmt_write_stop(&self, msg: &mut CanMsg) {
        msg.set_id(NMT_NODE, CO_IDE);
        msg.clr_rtr();
        msg.set_dlc(DLC_NMT);
        msg.set_data([STOP, self.node as u8, 0, 0, 0, 0, 0, 0]);
    }

    /* Pre-Operational Remote Node */
    pub fn nmt_write_preop(&self, msg: &mut CanMsg) {
        msg.set_id(NMT_NODE, CO_IDE);
        msg.clr_rtr();
        msg.set_dlc(DLC_NMT);
        msg.set_data([PREOP, self.node as u8, 0, 0, 0, 0, 0, 0]);
    }

    /* Reset Remote Node */
    pub fn nmt_write_reset(&self, msg: &mut CanMsg) {
        msg.set_id(NMT_NODE, CO_IDE);
        msg.clr_rtr();
        msg.set_dlc(DLC_NMT);
        msg.set_data([RESET, self.node as u8, 0, 0, 0, 0, 0, 0]);
    }
    
    /* Reset Communication Remote Node */
    pub fn nmt_write_comms(&self, msg: &mut CanMsg) {
        msg.set_id(NMT_NODE, CO_IDE);
        msg.clr_rtr();
        msg.set_dlc(DLC_NMT);
        msg.set_data([COMMS, self.node as u8, 0, 0, 0, 0, 0, 0]);
    }

    /* Heartbeat / Guarding Consumer */
    pub fn nmt_read_heartbeat(&mut self, msg: &CanMsg) -> u8 {
        let data = msg.get_data()[0];
        
        if ((data  >> SHIFT) & HB_MASK as u8) == 1 {
            self.toggle = true;
        } else {
            self.toggle = false;
        }

        let state = data & MASK as u8;
        self.state = canopen::canopen_state(state);
        return data & MASK as u8;
    }

    /* Heartbeat Producer */
    pub fn nmt_write_heartbeat(&self, msg: &mut CanMsg) {
        msg.set_id(HB + self.node, CO_IDE);
        msg.clr_rtr();
        msg.set_dlc(DLC_HB);
        msg.set_data([canopen::canopen_state_val(self.state), 0, 0, 0, 0, 0, 0, 0]);
    }

    /* Guarding */
    /* Is A Client Server - Request Response Of Heartbeat, Can Be Used To read The State */
    pub fn nmt_request_guarding(&self, msg: &mut CanMsg) {       
        msg.set_id(HB + self.node, CO_IDE);
        msg.set_rtr();
        msg.set_dlc(DLC_RTR);
        msg.set_data([0, 0, 0, 0, 0, 0, 0, 0]);
    }

    /* Guarding */
    /* Is a Client Server - This Is The Server Response Of Its Own State */
    pub fn nmt_response_guarding(&mut self, msg: &mut CanMsg) {
        let hb;

        if self.toggle { // Generate a heartbeat signal for the 7 bit in the first byte of data
            hb = 1 << SHIFT;
            self.toggle = false;
        } else {
            hb = 0 << SHIFT;
            self.toggle = true;
        }
        
        msg.set_id(HB + self.node, CO_IDE);
        msg.clr_rtr();
        msg.set_dlc(DLC_HB);
        msg.set_data([canopen::canopen_state_val(self.state) | hb, 0, 0, 0, 0, 0, 0, 0]);
    }
}