/* Service Data Object */
/* SDO Is A Client / Server Type */
use super::CANOpen;
use crate::hal::{can::CanMsg, common};

const SDO_TX:           u32 = 0x580;    // Server Rx Client Tx
const SDO_RX:           u32 = 0x600;    // Server TX Client RX

const IDE:              bool = false;   // CANOpen only uses normal ID

const S_OFFSET:         u8 = 0; 
const E_OFFSET:         u8 = 1;
const N_OFFSET:         u8 = 2;
const CCS_OFFSET:       u8 = 5;

const OD_MASK:          u16 = 0xFF;

// CCS is the client command specifier of the SDO transfer
// 0 for SDO segment download,
// 1 for initiating download,
// 2 for initiating upload,
// 3 for SDO segment upload,
// 4 for aborting an SDO transfer,
// 5 for SDO block upload,
// 6 for SDO block download
pub enum Ccs {SegDl, InitDl, SegUl, AbortTrans, BlkUl, BlkDl}

// N is the number of bytes in the data part of the message 
// which do not contain data, only valid if e and s are set
pub enum N {Bytes0, Bytes1, Bytes2, Bytes3} 

// E if set, 
// indicates an expedited transfer, 
// i.e. all data exchanged are contained within the message. 
// If this bit is cleared then the message is a segmented transfer 
// where the data does not fit into one message and multiple messages 
// are used
pub enum E {Segmented, Expedited}

// S if set, 
// indicates that the data size is specified in n (if e is set) 
// or in the data part of the message
pub enum S {Unset, DataSizeN}

pub struct CANOpenSdo {
    cmd_byte:   u8,         // Combination of CCS, N, E, S
    od_ind:     u16,        // Index (16 bits) reflect the OD address to be accessed
    od_sub:     u8          // Subindex (8 bits) reflect the OD address to be accessed
}

impl CANOpen {
    /* SDO Segment Download */
    pub fn sdo_write(&self, cod_id: u32, node_id: u32, sdo: &CANOpenSdo, msg_data: [u8; 4] ) -> CanMsg { 
        let mut msg = CanMsg::init();
        let mut data = [0; 8];

        data[0] = sdo.cmd_byte;
        data[1] = ((sdo.od_ind >> 0) & OD_MASK) as u8;
        data[2] = ((sdo.od_ind >> 8) & OD_MASK) as u8;
        data[3] = sdo.od_sub;
        data[4] = msg_data[0];
        data[5] = msg_data[1];
        data[6] = msg_data[2];
        data[7] = msg_data[3];


        msg.set_id(cod_id + node_id, false);
        

        return msg;
    }

    pub fn sdo_read(&self, msg: CanMsg) {

    }
}

impl CANOpenSdo {
    pub fn init(ccs: Ccs, n: N, e: E, s: S, od_ind: u16, od_sub: u8) -> CANOpenSdo {
        let mut cmd_byte = 0;

        cmd_byte |= (s as u8) << S_OFFSET;
        cmd_byte |= (e as u8) << E_OFFSET;
        cmd_byte |= (n as u8) << N_OFFSET;
        cmd_byte |= (ccs as u8) << CCS_OFFSET;
        
        return CANOpenSdo {
            cmd_byte:   cmd_byte,
            od_ind:     od_ind,
            od_sub:     od_sub
        };
    }

    pub fn get_cmd_byte(&self) -> u8 {
        return self.cmd_byte;
    }

    pub fn get_od_ind(&self) -> u16 {
        return self.od_ind;
    }

    pub fn get_od_sub(&self) -> u8 {
        return self.od_sub;
    }
}