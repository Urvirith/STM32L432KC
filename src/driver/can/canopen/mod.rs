/* CANOpen */
/* Standard Industrial Protocol */
/* Uses Include - Robotics, Remote I/O, Safety I/O */
/* General Functions */

mod sdo;
mod nmt;

const BOOTUP:           u8 = 0x00;      // Boot up (Initialising)
const STOPPED:          u8 = 0x04;      // Stopped State
const OPERATIONAL:      u8 = 0x05;      // Operationall State
const PREOPERATION:     u8 = 0x7F;      // Pre-Operational State
const UNKNOWN:          u8 = 0xFF;      // Pre-Operational State

struct CANOpen {
    node:   u32,            /* Internal Node Address Set By Program 1 - 127 */
    state:  CanOpenState,   /* Internal State Of The Node */
    toggle: bool            /* Internal Bit For NMT Node Guarding */
}

#[derive(Clone, Copy)]
pub enum CanOpenState {Bootup, Stopped, Operational, PreOperational, Unknown}

pub fn canopen_state_val(state: CanOpenState) -> u8 {
    return match state {
        CanOpenState::Bootup            => BOOTUP,
        CanOpenState::Stopped           => STOPPED,
        CanOpenState::Operational       => OPERATIONAL,
        CanOpenState::PreOperational    => PREOPERATION,
        CanOpenState::Unknown           => UNKNOWN
    };
}

pub fn canopen_state(state: u8) -> CanOpenState {
    return match state {
        BOOTUP                          =>  CanOpenState::Bootup,
        STOPPED                         =>  CanOpenState::Stopped,
        OPERATIONAL                     =>  CanOpenState::Operational,
        PREOPERATION                    =>  CanOpenState::PreOperational,
        _                               =>  CanOpenState::Unknown
    };
}

impl CANOpen {
    pub fn init(node: u32) -> CANOpen {
        return CANOpen {
            node:   node,
            state:  CanOpenState::Bootup,
            toggle: false
        };
    }

    pub fn get_node(&self) -> u32 {
        return self.node;
    }

    pub fn set_node(&mut self, node: u32) {
        self.node = node;
    }

    pub fn get_state(&self) -> CanOpenState {
        return self.state;
    }

    pub fn set_state(&mut self, state: CanOpenState) {
        self.state = state;
    }
}

