/* CANOpen */
/* Standard Industrial Protocol */
/* Uses Include - Robotics, Remote I/O, Safety I/O */
/* General Functions */

pub mod sdo;
pub mod pdo;
pub mod nmt;

use crate::hal::{can::CanMsg};

/* State Commands For CANOpen */
pub const BOOTUP:       u8 = 0x00;      // Boot up (Initialising)
pub const STOPPED:      u8 = 0x04;      // Stopped State
pub const OPERATIONAL:  u8 = 0x05;      // Operationall State
pub const PREOPERATION: u8 = 0x7F;      // Pre-Operational State
pub const UNKNOWN:      u8 = 0xFF;      // Unknown State

/* Function Codes For CANOpen */
pub const NMT:          u32 = 0x0000;   // Network Management 
pub const SYNC:         u32 = 0x0080;   // Synchronization
pub const EMCY:         u32 = 0x0080;   // Emergency
pub const TIME:         u32 = 0x0100;   // Timestamp
pub const TPDO1:        u32 = 0x0180;   // Process Data Object
pub const RPDO1:        u32 = 0x0200;   // Process Data Object
pub const TPDO2:        u32 = 0x0280;   // Process Data Object
pub const RPDO2:        u32 = 0x0300;   // Process Data Object
pub const TPDO3:        u32 = 0x0380;   // Process Data Object
pub const RPDO3:        u32 = 0x0400;   // Process Data Object
pub const TPDO4:        u32 = 0x0480;   // Process Data Object
pub const RPDO4:        u32 = 0x0500;   // Process Data Object
pub const TSDO:         u32 = 0x0580;   // Service Data Object
pub const RSDO:         u32 = 0x0600;   // Service Data Object
pub const HEARTBEAT:    u32 = 0x0700;   // Node Monitoring (Heartbeat)

/* Function Code And Node Masks */
const NODE_MASK:        u32 = 0x007F;   // Standard ID, Node Mask (Not Extended)
const FC_MASK:          u32 = 0x0780;   // Standard ID, Function Code Mask

pub struct CANOpen {
    node:   u32,                        /* Internal Node Address Set By Program 1 - 127 */
    state:  CANOpenState,               /* Internal State Of The Node */
    toggle: bool                        /* Internal Bit For NMT Node Guarding */
}

#[derive(Clone, Copy)]
pub enum CANOpenState {
    Bootup          = 0x00, 
    Stopped         = 0x04, 
    Operational     = 0x05, 
    PreOperational  = 0x7F, 
    Unknown         = 0xFF
}

pub fn canopen_state_val(state: CANOpenState) -> u8 {
    return state as u8;
}

pub fn canopen_state(state: u8) -> CANOpenState {
    return match state {
        BOOTUP                          =>  CANOpenState::Bootup,
        STOPPED                         =>  CANOpenState::Stopped,
        OPERATIONAL                     =>  CANOpenState::Operational,
        PREOPERATION                    =>  CANOpenState::PreOperational,
        _                               =>  CANOpenState::Unknown
    };
}

impl CANOpen {
    pub fn init(node: u32) -> CANOpen {
        return CANOpen {
            node:   node,
            state:  CANOpenState::Bootup,
            toggle: false
        };
    }

    /* Get The Node Of The Remote Message - Standard ID Only */
    /* Used Externally To Get The Node Number, Or A Mask Can Applied At Higher Logic */
    pub fn get_ext_node(cob_id: u32) -> u32 {
        return cob_id & NODE_MASK;
    }

    /* Get The Node Of The Remote Message - Standard ID Only */
    /* Used Against The Master To Get The Node Number, Or A Mask Can Applied At Higher Logic */
    pub fn get_source_node(&self, cob_id: u32) -> u32 {
        return cob_id & NODE_MASK;
    }

    /* Get The Function Code Of The Remote Message - Standard ID Only */
    /* Used Against The Master Or Slave To Get The Function Code, Or A Mask Can Applied At Higher Logic */
    pub fn get_source_fc(&self, cob_id: u32) -> u32 {
        return cob_id & FC_MASK;
    }

    /* Obtain Own Node ID */
    pub fn get_node(&self) -> u32 {
        return self.node;
    }

    /* Obtain Own State */
    pub fn get_state(&self) -> CANOpenState {
        return self.state;
    }

    pub fn get_state_u8(&self) -> u8 {
        return canopen_state_val(self.state);
    }

    /* Set Internal State */
    pub fn set_state(&mut self, state: CANOpenState) {
        self.state = state;
    }

    /* Decision Tree For Messages */
    pub fn msg_handler(&self, msg: &mut CanMsg) {
        let fc = msg.get_id();


    }

    
    /* FOR USE IN DOWN STEAM LOGIC WITHIN*/
    /* Client Tx, Server Rx - Used In SDO Creation*/
    pub fn get_rsdo(&self) -> u32 {
        return RSDO;
    }

    pub fn get_tsdo(&self) -> u32 {
        return TSDO;
    }
}

