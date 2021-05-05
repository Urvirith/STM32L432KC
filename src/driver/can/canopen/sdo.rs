/* Service Data Object */
use super::CANOpen;

const SDO_MST_RX:       u32 = 0x580; // Master Rx Slave Tx
const SDO_MST_TX:       u32 = 0x600; // Master TX Slave RX


// CCS is the client command specifier of the SDO transfer
// 0 for SDO segment download,
// 1 for initiating download,
// 2 for initiating upload,
// 3 for SDO segment upload,
// 4 for aborting an SDO transfer,
// 5 for SDO block upload,
// 6 for SDO block download
pub enum Css {SegDl, InitDl, SegUl, AbortTrans, BlkUl, BlkDl}

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

impl CANOpen {
    /* SDO Segment Download */
    pub fn sdo_send (&self) { 

    }
}