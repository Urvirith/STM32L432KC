/* Process Data Object */

use super::CANOpen;
use crate::hal::{common, can::CanMsg};


impl CANOpen {
    pub fn pdo_write(&self, cod_id: u32, dlc: u32, data: [u8; 8], msg: &mut CanMsg) {
        msg.set_id(cod_id + self.node, false);
        msg.clr_rtr();
        msg.set_dlc(dlc);
        msg.set_data(data);
    }
}
