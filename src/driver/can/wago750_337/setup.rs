use super::Wago750;
use crate::hal::{can::CanMsg, can::Can};
use super::canopen::{sdo};
use super::canopen;

const GUARDTIME:        u16 = 0x100C; 
const LIFEFACTOR:       u16 = 0x100D;
const GTLTSI:           u8  = 0x00;
const BYTEMASK:         u8  = 0xFF;

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
const TPDO1ANALOGIN2SI:     u32 = 0x02;     // Sub Index For First 2 Channel Analogue Output

const TXPDO1:               u16 = 0x1800;
const TXPDO1DISABLECOBID:   u8 =  0x80;     // ALIGN TO THE LAST BYTE TO DISABLE THE TXPDO FOR WRITING
const TXPDO1SI2:            u8 =  0xFF;     // Set To Change Of State
const TXPDO1SI3:            u16 = 0x012C;   // Set To 30 ms Interval Inhibit Time
const TXPDO1SI5:            u16 = 0x764;    // Set To 50 ms Interval Event Time

impl Wago750 {
    /* This is a pure custom implementation due to the nature of the flex I/O as the data packs itself dynamically */
    /* Pg. 97 Gives A Basic Idea For Commissioning */
    pub fn setup_wago(&mut self, bus: &Can) {
        // Set Up Node Guarding / PDOs
        match self.co_node.get_state() {
            canopen::CANOpenState::PreOperational => {
                if bus.write_free() {
                    match self.step {
                        0 => {      // Initialize The Guard Time
                            self.setup_guardtime(bus);
                        } 1 => {    // Initialize The Life Factor Time
                            self.setup_lifefactor(bus);
                        } 2 => {    // Initialize The PDO Rx (Client Recieve) Digital Outputs Being 8 bits
                            self.setup_rpdo1_digoutputs(bus);
                        } 3 => {    // Initialize The PDO Rx (Client Recieve) First Analogue Signal 16 Bits
                            self.setup_rpdo1_analogue1output(bus);
                        } 4 => {    // Initialize The PDO Rx (Client Recieve) Second Analogue Signal 16 Bits
                            self.setup_rpdo1_analogue2output(bus);
                        } 5 => {    // Initialize The PDO Rx (Client Recieve) Set The Number Of Mapped Interactions
                            self.setup_rpdo1_number(bus);
                        } 6 => {    // Initialize The PDO TX (Server Recieve) Digital Inputs Being 8 bits
                            self.setup_tpdo1_diginput(bus);
                        } 7 => {    // Initialize The PDO TX (Server Recieve) First Analogue Signal 16 Bits
                            self.setup_tpdo1_analogue1input(bus);
                        } 8 => {    // Initialize The PDO TX (Server Recieve) Second Analogue Signal 16 Bits
                            self.setup_tpdo1_analogue2input(bus);
                        } 9 => {    // Initialize The PDO TX (Server Recieve) Set The Number Of Mapped Interactions
                            self.setup_tpdo1_number(bus);
                        } 10 => {    // Initialize The PDO TX (Server Recieve) Disable The COB-ID
                            self.setup_pdo_cobid(bus, TXPDO1, 1, 0, true);
                        } 11 => {    // Initialize The PDO TX (Server Recieve) Map As Change Of State Type
                            self.setup_tpdo1_cos(bus);
                        } 12 => {    // Initialize The PDO TX (Server Recieve) Set Up Inhibit Time
                            self.setup_inhibittime(bus);
                        } 13 => {    // Initialize The PDO TX (Server Recieve) Set Up Event Time
                            self.setup_eventtime(bus);
                        } 14 => {    // Initialize The PDO TX (Server Recieve) Enable COB-ID To Default, Default TPDO + Node Number
                            self.setup_pdo_cobid(bus, TXPDO1, 1, canopen::TPDO1 + self.co_node.get_node(), false);
                        }_ => {    // Add A Read Function For Intepretting The COB-ID
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

    /* P.100 Describes The Transmission Of A PDO */
    /* P.117 Describes The Default Mapping For PDO */
    /* P.129 Describes The Default Mapping For PDO */
    /* P.168 Describes Transmit/ Recieve Of PDO */

        /*Node Guarding is switched off by default because 0 is entered in the respective indices (0x100C = Guard-Time, 0x100D = Life Time Factor)*/
    /* 0x100C – Guard Time 
                This object specifies the Guard Time in milliseconds. 
                An NMT master requests the state of the NMT slave in a cyclical manner. 
                The time between two requests is the Guard Time. 
    */
    fn setup_guardtime(&mut self, bus: &Can) {
        let mut msg = CanMsg::init();
        self.co_node.sdo_init_download(sdo::N::Bytes2, sdo::E::Expedited, GUARDTIME, GTLTSI, [((GUARDTIMEMS >> 0) as u8 & BYTEMASK), ((GUARDTIMEMS >> 8) as u8 & BYTEMASK), 0, 0], &mut msg);
        self.write_bus_step(bus, msg);
    }

    /* 0x100D – Life Time Factor
                The Life Time Factor is part of the Node Guarding Protocol. The NMT slave
                checks whether it was queried within the Node Life Time (guard time multiplied
                by the Life Time Factor). If not, the slave must assume that the NMT master is no
                longer in normal operation. It then initiates a Life Guarding Event.
                If the Node Life Time is zero, there is no monitoring.
    */
    fn setup_lifefactor(&mut self, bus: &Can) {
        let mut msg = CanMsg::init();
        self.co_node.sdo_init_download(sdo::N::Bytes3, sdo::E::Expedited, LIFEFACTOR, GTLTSI, [LIFEFACTORMUL, 0, 0, 0], &mut msg);
        self.write_bus_step(bus, msg);
    }

    /* Initialize The PDO Rx (Client Recieve) Digital Outputs Being 8 bits */
    fn setup_rpdo1_digoutputs(&mut self, bus: &Can) {
        let mut msg = CanMsg::init();
        self.co_node.sdo_init_download(sdo::N::Bytes0, sdo::E::Expedited, RPDO1, 1, self.pdo_mapping_structure(RPDO1DIGOUTPUTS, RPDO1DIG1SI, RPDO1DIGBITS), &mut msg);
        self.write_bus_step(bus, msg);
    }

    fn setup_rpdo1_analogue1output(&mut self, bus: &Can) {
        let mut msg = CanMsg::init();
        self.co_node.sdo_init_download(sdo::N::Bytes0, sdo::E::Expedited, RPDO1, 2, self.pdo_mapping_structure(RPDO1ANALOGOUT, RPDO1ANALOGOUT1SI, RPDO1ANALOGBITS), &mut msg);
        self.write_bus_step(bus, msg);
    }

    fn setup_rpdo1_analogue2output(&mut self, bus: &Can) {
        let mut msg = CanMsg::init();
        self.co_node.sdo_init_download(sdo::N::Bytes0, sdo::E::Expedited, RPDO1, 3, self.pdo_mapping_structure(RPDO1ANALOGOUT, RPDO1ANALOGOUT2SI, RPDO1ANALOGBITS), &mut msg);
        self.write_bus_step(bus, msg);
    }

    fn setup_rpdo1_number(&mut self, bus: &Can) {
        let mut msg = CanMsg::init();
        self.co_node.sdo_init_download(sdo::N::Bytes3, sdo::E::Expedited, RPDO1, 0, [RPDO1SINUMAPPED, 0, 0, 0], &mut msg);
        self.write_bus_step(bus, msg);
    }

    fn setup_tpdo1_diginput(&mut self, bus: &Can) {
        let mut msg = CanMsg::init();
        self.co_node.sdo_init_download(sdo::N::Bytes0, sdo::E::Expedited, TPDO1, 1, self.pdo_mapping_structure(TPDO1DIGINPUTS, TPDO1DIG1SI, TPDO1DIGBITS), &mut msg);
        self.write_bus_step(bus, msg);
    }

    fn setup_tpdo1_analogue1status(&mut self, bus: &Can) { /* Implement */
        let mut msg = CanMsg::init();
        self.co_node.sdo_init_download(sdo::N::Bytes0, sdo::E::Expedited, TPDO1, 2, self.pdo_mapping_structure(TPDO1ANALOGIN, TPDO1ANALOGIN1SI, TPDO1ANALOGBITS), &mut msg);
        self.write_bus_step(bus, msg);
    }

    fn setup_tpdo1_analogue1input(&mut self, bus: &Can) {
        let mut msg = CanMsg::init();
        self.co_node.sdo_init_download(sdo::N::Bytes0, sdo::E::Expedited, TPDO1, 2, self.pdo_mapping_structure(TPDO1ANALOGIN, TPDO1ANALOGIN1SI, TPDO1ANALOGBITS), &mut msg);
        self.write_bus_step(bus, msg);
    }

    fn setup_tpdo1_analogue2status(&mut self, bus: &Can) { /* Implement */
        let mut msg = CanMsg::init();
        self.co_node.sdo_init_download(sdo::N::Bytes0, sdo::E::Expedited, TPDO1, 2, self.pdo_mapping_structure(TPDO1ANALOGIN, TPDO1ANALOGIN1SI, TPDO1ANALOGBITS), &mut msg);
        self.write_bus_step(bus, msg);
    }

    fn setup_tpdo1_analogue2input(&mut self, bus: &Can) {
        let mut msg = CanMsg::init();
        self.co_node.sdo_init_download(sdo::N::Bytes0, sdo::E::Expedited, TPDO1, 3, self.pdo_mapping_structure(TPDO1ANALOGIN, TPDO1ANALOGIN2SI, TPDO1ANALOGBITS), &mut msg);
        self.write_bus_step(bus, msg);
    }

    fn setup_tpdo1_number(&mut self, bus: &Can) {
        let mut msg = CanMsg::init();
        self.co_node.sdo_init_download(sdo::N::Bytes3, sdo::E::Expedited, TPDO1, 0, [TPDO1SINUMAPPED, 0, 0, 0], &mut msg);
        self.write_bus_step(bus, msg);
    }

    pub fn setup_pdo_cobid(&mut self, bus: &Can, index: u16, subindex: u8, cob_id: u32, disable: bool) {
        let mut msg = CanMsg::init();
        // Set The Heartbeat Interval
        if disable {
            self.co_node.sdo_init_download(sdo::N::Bytes0, sdo::E::Expedited, index, subindex, [0, 0, 0, TXPDO1DISABLECOBID], &mut msg);
        } else {
            self.co_node.sdo_init_download(sdo::N::Bytes0, sdo::E::Expedited, index, subindex, [((cob_id >> 0) & 0xFF) as u8, ((cob_id >> 8) & 0xFF) as u8, ((cob_id >> 16) & 0xFF) as u8, 0], &mut msg);
        }

        self.write_bus_step(bus, msg);
    }

    fn setup_tpdo1_cos(&mut self, bus: &Can) {
        let mut msg = CanMsg::init();
        self.co_node.sdo_init_download(sdo::N::Bytes3, sdo::E::Expedited, TXPDO1, 2, [TXPDO1SI2, 0, 0, 0], &mut msg);
        self.write_bus_step(bus, msg);
    }

    fn setup_inhibittime(&mut self, bus: &Can) {
        let mut msg = CanMsg::init();
        self.co_node.sdo_init_download(sdo::N::Bytes2, sdo::E::Expedited, TXPDO1, 3, [((TXPDO1SI3 >> 0) as u8 & BYTEMASK), ((TXPDO1SI3 >> 8) as u8 & BYTEMASK), 0, 0], &mut msg);
        self.write_bus_step(bus, msg);
    }

    fn setup_eventtime(&mut self, bus: &Can) {
        let mut msg = CanMsg::init();
        self.co_node.sdo_init_download(sdo::N::Bytes2, sdo::E::Expedited, TXPDO1, 5, [((TXPDO1SI5 >> 0) as u8 & BYTEMASK), ((TXPDO1SI5 >> 8) as u8 & BYTEMASK), 0, 0], &mut msg);
        self.write_bus_step(bus, msg);
    }

    fn write_bus_step(&mut self, bus: &Can, msg: CanMsg) {
        // Write To The Bus
        bus.write(msg);
        // Increment Step
        self.step += 1;
    }
}
