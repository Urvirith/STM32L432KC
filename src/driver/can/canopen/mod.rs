/* CANOpen Master*/
/* Standard Industrial Protocol */
/* Uses Include - Robotics, Remote I/O, Safety I/O */
mod sdo;
mod nmt;

struct CANOpen {
    node:   u32
}

impl CANOpen {
    pub fn init(node: u32) -> CANOpen {
        return CANOpen {
            node: node
        }
    }
}

