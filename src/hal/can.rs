/* CAN (Controller Area Network) */
/* Manual Page 1476 */

use super::{common, pointer};

pub struct Can {
    base:       u32,            // Base - Used For Filter Creation
    mcr:        *mut u32,       // Master Control Register
    msr:        *mut u32,       // Master Status Register
    tsr:        *mut u32,       // Transmit Status Register
    rf0r:       *mut u32,       // Receive FIFO 0 Register
    rf1r:       *mut u32,       // Receive FIFO 1 Register
    ier:        *mut u32,       // Interrupt Enable Register
    esr:        *mut u32,       // Error Status Register
    btr:        *mut u32,       // Bit Timing Register
    ti0r:       *mut u32,       // TX Mailbox Identifer Register
    tdt0r:      *mut u32,       // TX Mailbox Data Length Control And Timestamp Register
    tdl0r:      *mut u32,       // TX Mailbox Data Low Register
    tdh0r:      *mut u32,       // TX Mailbox Data High Register
    ti1r:       *mut u32,       // TX Mailbox Identifer Register
    tdt1r:      *mut u32,       // TX Mailbox Data Length Control And Timestamp Register
    tdl1r:      *mut u32,       // TX Mailbox Data Low Register
    tdh1r:      *mut u32,       // TX Mailbox Data High Register
    ti2r:       *mut u32,       // TX Mailbox Identifer Register
    tdt2r:      *mut u32,       // TX Mailbox Data Length Control And Timestamp Register
    tdl2r:      *mut u32,       // TX Mailbox Data Low Register
    tdh2r:      *mut u32,       // TX Mailbox Data High Register
    ri0r:       *mut u32,       // RX Mailbox Identifer Register
    rdt0r:      *mut u32,       // RX Mailbox Data Length Control And Timestamp Register
    rdl0r:      *mut u32,       // RX Mailbox Data Low Register
    rdh0r:      *mut u32,       // RX Mailbox Data High Register
    ri1r:       *mut u32,       // RX Mailbox Identifer Register
    rdt1r:      *mut u32,       // RX Mailbox Data Length Control And Timestamp Register
    rdl1r:      *mut u32,       // RX Mailbox Data Low Register
    rdh1r:      *mut u32,       // RX Mailbox Data High Register
    fmr:        *mut u32,       // Filter Master Register
    fm1r:       *mut u32,       // Filter Mode Register
    fs1r:       *mut u32,       // Filter Scale Register
    ffa1r:      *mut u32,       // Filter FIFO Assignment Register
    fa1r:       *mut u32,       // Filter Activation Register
}

/* Register Offset */
const MCR:      u32 = 0x0000;
const MSR:      u32 = 0x0004;
const TSR:      u32 = 0x0008;
const RF0R:     u32 = 0x000C;
const RF1R:     u32 = 0x0010;
const IER:      u32 = 0x0014;
const ESR:      u32 = 0x0018;
const BTR:      u32 = 0x001C;
const TI0R:     u32 = 0x0180;
const TDT0R:    u32 = 0x0184;
const TDL0R:    u32 = 0x0188;
const TDH0R:    u32 = 0x018C;
const TI1R:     u32 = 0x0190;
const TDT1R:    u32 = 0x0194;
const TDL1R:    u32 = 0x0198;
const TDH1R:    u32 = 0x019C;
const TI2R:     u32 = 0x01A0;
const TDT2R:    u32 = 0x01A4;
const TDL2R:    u32 = 0x01A8;
const TDH2R:    u32 = 0x01AC;
const RI0R:     u32 = 0x01B0;
const RDT0R:    u32 = 0x01B4;
const RDL0R:    u32 = 0x01B8;
const RDH0R:    u32 = 0x01BC;
const RI1R:     u32 = 0x01C0;
const RDT1R:    u32 = 0x01C4;
const RDL1R:    u32 = 0x01C8;
const RDH1R:    u32 = 0x01CC;
const FMR:      u32 = 0x0200;
const FM1R:     u32 = 0x0204;
const FS1R:     u32 = 0x020C;
const FFA1R:    u32 = 0x0214;
const FA1R:     u32 = 0x021C;
const FRB:      u32 = 0x0240;

/* Config Struct */
pub struct CanInit {
    txfp:       bool,       // Transmit FIFO Priority
    rflm:       bool,       // Receive FIFO Locked mode
    nart:       bool,       // No Automatic Retransmission
    awum:       bool,       // Automatic Wakeup Mode
    abom:       bool,       // Automatic Bus-off Management
    ttcm:       bool,       // Time Triggered Communication Mode
    brp:        u32,        // Baud Rate Prescaler
    ts1:        u32,        // Time Segment 1
    ts2:        u32,        // Time Segment 2
    sjw:        u32         // Resynchronization Jump Width
}

/* Message Struct */
pub struct CanMsg {
    id:         u32,        // Standard Identifier Or Extended Identifier
    ide:        bool,       // This Bit Defines The Identifier Type Of Message In The Mailbox 0: Standard Identifier 1: Extended Identifier.
    rtr:        bool,       // Remote Transmission Request 0: Data Frame 1: Remote Frame
    dlc:        u32,        // Data Length Code
    fmi:        u32,        // Filter Match Index
    data:       [u8; 8]     // Data Bytes 0 - 7
}

/* Enumerations */
/* Baud Rates */
pub enum BaudRate {Baud125kB, Baud250kB, Baud500kB, Baud1MB}

pub enum FifoReg {Fifo0, Fifo1}

pub fn baud(br: BaudRate) -> u32 {
    return match br {
        BaudRate::Baud125kB => 16,
        BaudRate::Baud250kB => 8,
        BaudRate::Baud500kB => 4,
        BaudRate::Baud1MB   => 2
    };
}

/* Register Masks */
const TIMEOUT:          u32 = 0x0000FFFF;

/* BTR */
const BRP_MASK:         u32 = common::MASK_9_BIT;
const TS1_MASK:         u32 = common::MASK_4_BIT;
const TS2_MASK:         u32 = common::MASK_3_BIT;
const SJW_MASK:         u32 = common::MASK_2_BIT;

/* RFxR */
const FMP_MASK:         u32 = common::MASK_2_BIT;

/* TIxR or RIxR */
const STID_MASK:        u32 = common::MASK_11_BIT;
const EXID_MASK:        u32 = common::MASK_29_BIT;

/* TDTxR or RDTxR */
const DLC_MASK:         u32 = common::MASK_4_BIT;
const FMI_MASK:         u32 = common::MASK_8_BIT;

/* DLR & DHR */
const DATA_MASK:        u32 = common::MASK_8_BIT;

/* Register Bits */
/* MCR */
const INRQ_BIT:         u32 = common::BIT_0;
const SLRQ_BIT:         u32 = common::BIT_1;
const TXFP_BIT:         u32 = common::BIT_2;
const RFLM_BIT:         u32 = common::BIT_3;
const NART_BIT:         u32 = common::BIT_4;
const AWUM_BIT:         u32 = common::BIT_5;
const ABOM_BIT:         u32 = common::BIT_6;
const TTCM_BIT:         u32 = common::BIT_7;

/* MSR */
const INAK_BIT:         u32 = common::BIT_0;
const SLAK_BIT:         u32 = common::BIT_1;

/* BTR */
const LBKM_BIT:         u32 = common::BIT_30;
const SILM_BIT:         u32 = common::BIT_31;

/* TSR */
const TME0_BIT:         u32 = common::BIT_26;
const TME1_BIT:         u32 = common::BIT_27;
const TME2_BIT:         u32 = common::BIT_28;

/* RFxR */
const RFOM_BIT:         u32 = common::BIT_5;

/* TIxR or RIxR */
const TXRQ_BIT:         u32 = common::BIT_0;
const RTR_BIT:          u32 = common::BIT_1;
const IDE_BIT:          u32 = common::BIT_2;

/* FM1R */
const FINIT:            u32 = common::BIT_0;


/* Register Offsets */
/* BTR */
const BRP_OFFSET:       u32 = 0;
const TS1_OFFSET:       u32 = 16;
const TS2_OFFSET:       u32 = 20;
const SJW_OFFSET:       u32 = 24;

/* RFxR */
const FMP_OFFSET:       u32 = 0;

/* TIxR or RIxR */
const STID_OFFSET:      u32 = 21;
const EXID_OFFSET:      u32 = 3;

/* TDTxR or RDTxR */
const DLC_OFFSET:       u32 = 0;
const FMI_OFFSET:       u32 = 8;

/* DLR & DHR */
const DATA_0_OFFSET:    u32 = 0;
const DATA_1_OFFSET:    u32 = 8;
const DATA_2_OFFSET:    u32 = 16;
const DATA_3_OFFSET:    u32 = 24;
const DATA_4_OFFSET:    u32 = 0;
const DATA_5_OFFSET:    u32 = 8;
const DATA_6_OFFSET:    u32 = 16;
const DATA_7_OFFSET:    u32 = 24;

const FXR1:             u32 = 0x00;
const FXR2:             u32 = 0x04;
const FR_BASE:          u32 = 0x08;

impl Can {
    /* Initialize The Structure */
    pub fn init(base: u32) -> Can {
        return Can {
            base:       base,
            msr:        (base + MSR)        as *mut u32,
            mcr:        (base + MCR)        as *mut u32,
            tsr:        (base + TSR)        as *mut u32,
            rf0r:       (base + RF0R)       as *mut u32,
            rf1r:       (base + RF1R)       as *mut u32,
            ier:        (base + IER)        as *mut u32,
            esr:        (base + ESR)        as *mut u32,
            btr:        (base + BTR)        as *mut u32,
            ti0r:       (base + TI0R)       as *mut u32,
            tdt0r:      (base + TDT0R)      as *mut u32,
            tdl0r:      (base + TDL0R)      as *mut u32,
            tdh0r:      (base + TDH0R)      as *mut u32,
            ti1r:       (base + TI1R)       as *mut u32,
            tdt1r:      (base + TDT1R)      as *mut u32,
            tdl1r:      (base + TDL1R)      as *mut u32,
            tdh1r:      (base + TDH1R)      as *mut u32,
            ti2r:       (base + TI2R)       as *mut u32,
            tdt2r:      (base + TDT2R)      as *mut u32,
            tdl2r:      (base + TDL2R)      as *mut u32,
            tdh2r:      (base + TDH2R)      as *mut u32,
            ri0r:       (base + RI0R)       as *mut u32,
            rdt0r:      (base + RDT0R)      as *mut u32,
            rdl0r:      (base + RDL0R)      as *mut u32,
            rdh0r:      (base + RDH0R)      as *mut u32,
            ri1r:       (base + RI1R)       as *mut u32,
            rdt1r:      (base + RDT1R)      as *mut u32,
            rdl1r:      (base + RDL1R)      as *mut u32,
            rdh1r:      (base + RDH1R)      as *mut u32,
            fmr:        (base + FMR)        as *mut u32,
            fm1r:       (base + FM1R)       as *mut u32,
            fs1r:       (base + FS1R)       as *mut u32,
            ffa1r:      (base + FFA1R)      as *mut u32,
            fa1r:       (base + FA1R)       as *mut u32
        };
    }

    // The software initialization can be done while the hardware is in Initialization mode. 
    // To enter this mode the software sets the INRQ bit in the CAN_MCR register and waits 
    // until the hardware has confirmed the request by setting the INAK bit in the CAN_MSR register.
    // To leave Initialization mode, the software clears the INQR bit. 
    // bxCAN has left Initialization mode once the INAK bit has been cleared by hardware.
    // While in Initialization Mode, all message transfers to and from the CAN bus are stopped 
    // and the status of the CAN bus output CANTX is recessive (high).
    // Entering Initialization Mode does not change any of the configuration registers.
    // To initialize the CAN Controller, software has to set up the Bit Timing (CAN_BTR) 
    // and CAN options (CAN_MCR) registers. To initialize the registers associated with the CAN filter banks 
    // (mode, scale, FIFO assignment, activation and filter values), software has to set the FINIT bit (CAN_FMR). 
    // Filter initialization also can be done outside the initialization mode.
    pub fn open(&self, ci: &CanInit) -> bool {
        let mut wait = 0;

        /* Remove from sleep mode and place into initialization mode */
        pointer::clr_ptr_vol_bit_u32(self.mcr, SLRQ_BIT);
        pointer::set_ptr_vol_bit_u32(self.mcr, INRQ_BIT);

        while !pointer::get_ptr_vol_bit_u32(self.msr, INAK_BIT) { // Wait for initialization mode
            if wait > TIMEOUT {
                return false;
            }
            wait+=1;
        }

        /* 0: Priority driven by the identifier of the message, 1: Priority driven by the request order (chronologically) */
        match ci.txfp { 
            true    => pointer::set_ptr_vol_bit_u32(self.mcr, TXFP_BIT),
            false   => pointer::clr_ptr_vol_bit_u32(self.mcr, TXFP_BIT)
        }

        /* 0: Receive FIFO not locked on overrun. Once a receive FIFO is full the next incoming message will overwrite the previous one 1: Receive FIFO locked against overrun. Once a receive FIFO is full the next incoming message will be discarded */
        match ci.rflm { 
            true    => pointer::set_ptr_vol_bit_u32(self.mcr, RFLM_BIT),
            false   => pointer::clr_ptr_vol_bit_u32(self.mcr, RFLM_BIT)
        }

        /* 0: The CAN hardware will automatically retransmit the message until it has been successfully transmitted according to the CAN standard 1: A message will be transmitted only once, independently of the transmission result (successful, error or arbitration lost) */
        match ci.nart { 
            true    => pointer::set_ptr_vol_bit_u32(self.mcr, NART_BIT),
            false   => pointer::clr_ptr_vol_bit_u32(self.mcr, NART_BIT)
        }

        /* 0: The Sleep mode is left on software request by clearing the SLEEP bit of the CAN_MCR register 1: The Sleep mode is left automatically by hardware on CAN message detection.The SLEEP bit of the CAN_MCR register and the SLAK bit of the CAN_MSR register are cleared by hardware */
        match ci.awum { 
            true    => pointer::set_ptr_vol_bit_u32(self.mcr, AWUM_BIT),
            false   => pointer::clr_ptr_vol_bit_u32(self.mcr, AWUM_BIT)
        }

        /* 0: The Bus-Off state is left on software request, once 128 occurrences of 11 recessive bits have been monitored and the software has first set and cleared the INRQ bit of the CAN_MCR register 1: The Bus-Off state is left automatically by hardware once 128 occurrences of 11 recessive bits have been monitored */
        match ci.abom { 
            true    => pointer::set_ptr_vol_bit_u32(self.mcr, ABOM_BIT),
            false   => pointer::clr_ptr_vol_bit_u32(self.mcr, ABOM_BIT)
        }

        /* 0: Time Triggered Communication mode disabled 1: Time Triggered Communication mode enabled */
        match ci.ttcm { 
            true    => pointer::set_ptr_vol_bit_u32(self.mcr, TTCM_BIT),
            false   => pointer::clr_ptr_vol_bit_u32(self.mcr, TTCM_BIT)
        }

        self.clock_setup(ci);

        pointer::clr_ptr_vol_bit_u32(self.mcr, INRQ_BIT);

        wait = 0;

        while pointer::get_ptr_vol_bit_u32(self.msr, INAK_BIT) { // Wait for initialization mode
            if wait > TIMEOUT {
                return false;
            }
            wait+=1;
        }

        return true;
    }

    /* Reception Handling */
    // Check if either FIFO has data in it
    pub fn read_pend(&self) -> bool {
        if pointer::get_ptr_vol_u32(self.rf0r, FMP_OFFSET, FMP_MASK) > 0 {
            return true;
        } else if pointer::get_ptr_vol_u32(self.rf1r, FMP_OFFSET, FMP_MASK) > 0 {
            return true;
        } else {
            return false;
        }
    }

    // For the reception of CAN messages, three mailboxes organized as a FIFO are provided. 
    // In order to save CPU load, simplify the software and guarantee data consistency, the FIFO is managed completely by hardware. 
    // The application accesses the messages stored in the FIFO through the FIFO output mailbox
    pub fn read(&self, msg: &mut CanMsg) ->  bool {
        /* Form The Pointers Dynamically */
        let regl;
        let regh;
        let ri;
        let rdt;
        let rdl;
        let rdh;
        let rf;
        
        /* Assign the pointer to simplify the logic, If Mailbox 0 Has More Than 1 Then Read 0 And Has A Message Waiting */
        if (pointer::get_ptr_vol_u32(self.rf0r, FMP_OFFSET, FMP_MASK) > pointer::get_ptr_vol_u32(self.rf1r, FMP_OFFSET, FMP_MASK)) && (pointer::get_ptr_vol_u32(self.rf0r, FMP_OFFSET, FMP_MASK) > 0) {
            ri  = self.ri0r;
            rdt = self.rdt0r;
            rdl = self.rdl0r;
            rdh = self.rdh0r;
            rf  = self.rf0r;
        /* Assign the pointer to simplify the logic, If Mailbox 1 Has More Than 0 Then Read 1 And Has A Message Waiting */
        } else if (pointer::get_ptr_vol_u32(self.rf1r, FMP_OFFSET, FMP_MASK) > pointer::get_ptr_vol_u32(self.rf0r, FMP_OFFSET, FMP_MASK)) && (pointer::get_ptr_vol_u32(self.rf1r, FMP_OFFSET, FMP_MASK) > 0) {
            ri  = self.ri1r;
            rdt = self.rdt1r;
            rdl = self.rdl1r;
            rdh = self.rdh1r;
            rf  = self.rf1r;
        } else { /* No Available Messages Were Found */
            return false;
        }

        msg.ide = pointer::get_ptr_vol_bit_u32(ri, IDE_BIT);

        if msg.ide {
            msg.id = pointer::get_ptr_vol_u32(ri, EXID_OFFSET, EXID_MASK);
        } else {
            msg.id = pointer::get_ptr_vol_u32(ri, STID_OFFSET, STID_MASK);
        }

        msg.rtr = pointer::get_ptr_vol_bit_u32(ri, RTR_BIT);
        msg.dlc = pointer::get_ptr_vol_u32(rdt, DLC_OFFSET, DLC_MASK);
        msg.fmi = pointer::get_ptr_vol_u32(rdt, FMI_OFFSET, FMI_MASK);
        regl = pointer::get_ptr_vol_raw_u32(rdl);
        regh = pointer::get_ptr_vol_raw_u32(rdh);
        msg.data[0] = ((regl >> DATA_0_OFFSET) & DATA_MASK) as u8;
        msg.data[1] = ((regl >> DATA_1_OFFSET) & DATA_MASK) as u8;
        msg.data[2] = ((regl >> DATA_2_OFFSET) & DATA_MASK) as u8;
        msg.data[3] = ((regl >> DATA_3_OFFSET) & DATA_MASK) as u8;
        msg.data[4] = ((regh >> DATA_4_OFFSET) & DATA_MASK) as u8;
        msg.data[5] = ((regh >> DATA_5_OFFSET) & DATA_MASK) as u8;
        msg.data[6] = ((regh >> DATA_6_OFFSET) & DATA_MASK) as u8;
        msg.data[7] = ((regh >> DATA_7_OFFSET) & DATA_MASK) as u8;

        pointer::set_ptr_vol_bit_u32(rf, RFOM_BIT);

        return true;
    }

    pub fn read_esr(&self) -> u32 {
        return pointer::get_ptr_vol_raw_u32(self.esr);
    }

    pub fn read_msr(&self) -> u32 {
        return pointer::get_ptr_vol_raw_u32(self.msr);
    }

    pub fn fifo_release(&self, fifo: FifoReg) {
        match fifo {
            FifoReg::Fifo0 => pointer::set_ptr_vol_bit_u32(self.rf0r, RFOM_BIT),
            FifoReg::Fifo1 => pointer::set_ptr_vol_bit_u32(self.rf1r, RFOM_BIT)
        }
    }

    /* Transmission Handling */
    // In order to transmit a message, 
    //      The application must select one empty transmit mailbox, 
    //      Set up the identifier, 
    //      The data length code (DLC) and 
    //      The data 
    // before requesting the transmission by setting the corresponding TXRQ bit in the CAN_TIxR register. 
    // Once the mailbox has left empty state, the software no longer has write access to the mailbox registers. 
    // Immediately after the TXRQ bit has been set, the mailbox enters pending state and waits to become the highest priority mailbox, 
    //      see Transmit Priority. 
    // As soon as the mailbox has the highest priority it will be scheduled for transmission. 
    // The transmission of the message of the scheduled mailbox will start (enter transmit state) when the CAN bus becomes idle. 
    // Once the mailbox has been successfully transmitted, it will become empty again. 
    // The hardware indicates a successful transmission by setting the RQCP and TXOK bits in the CAN_TSR register. 
    // If the transmission fails, the cause is indicated by the ALST bit in the CAN_TSR register in case of an Arbitration Lost, and/or the TERR bit, 
    // in case of transmission error detection.
    pub fn write(&self, msg: &CanMsg) -> bool { 
        let regl = ((msg.data[3] as u32) << DATA_3_OFFSET) | ((msg.data[2] as u32) << DATA_2_OFFSET) | ((msg.data[1] as u32) << DATA_1_OFFSET) | ((msg.data[0] as u32) << DATA_0_OFFSET);
        let regh = ((msg.data[7] as u32) << DATA_7_OFFSET) | ((msg.data[6] as u32) << DATA_6_OFFSET) | ((msg.data[5] as u32) << DATA_5_OFFSET) | ((msg.data[4] as u32) << DATA_4_OFFSET);
        let ti;
        let tdt;
        let tdl;
        let tdh;

        /* Assign Pointer To Local Variable */
        if pointer::get_ptr_vol_bit_u32(self.tsr, TME0_BIT) {           // Check if the first mailbox is empty
            ti  = self.ti0r;
            tdt = self.tdt0r;
            tdl = self.tdl0r;
            tdh = self.tdh0r;
        } else if pointer::get_ptr_vol_bit_u32(self.tsr, TME1_BIT) {    // Check if the second mailbox is empty
            ti  = self.ti1r;
            tdt = self.tdt1r;
            tdl = self.tdl1r;
            tdh = self.tdh1r;
        } else if pointer::get_ptr_vol_bit_u32(self.tsr, TME2_BIT) {    // Check if the third mailbox is empty
            ti  = self.ti2r;
            tdt = self.tdt2r;
            tdl = self.tdl2r;
            tdh = self.tdh2r;
        } else {                                                        // No mailbox found return
            return false;
        }

        match msg.rtr {
            true    => pointer::set_ptr_vol_bit_u32(ti, RTR_BIT),
            false   => pointer::clr_ptr_vol_bit_u32(ti, RTR_BIT)
        }
        
        match msg.ide {
            true    => {
                pointer::set_ptr_vol_bit_u32(ti, IDE_BIT);
                pointer::set_ptr_vol_u32(ti, EXID_OFFSET, EXID_MASK, msg.id);
            } false => {
                pointer::clr_ptr_vol_bit_u32(ti, IDE_BIT);
                pointer::set_ptr_vol_u32(ti, STID_OFFSET, STID_MASK, msg.id);
            }
        }
        
        pointer::set_ptr_vol_u32(tdt, DLC_OFFSET, DLC_MASK, msg.dlc);
        pointer::set_ptr_vol_raw_u32(tdl, regl);
        pointer::set_ptr_vol_raw_u32(tdh, regh);
        pointer::set_ptr_vol_bit_u32(ti, TXRQ_BIT);
        
        return true;
    }

    /* TO-DO */
    pub fn write_sts(&self) {

    }

    pub fn filter_init(&self, filter: u32, list: bool, fifo: bool, active: bool, mask: u32) {
        if filter > 13 {
            return;
        }

        let bit = 1 << filter;
        let fxr1 = (self.base + FRB + (filter * FR_BASE) + FXR1) as *mut u32;
        let fxr2 = (self.base + FRB + (filter * FR_BASE) + FXR2) as *mut u32;

        pointer::set_ptr_vol_bit_u32(self.fmr, FINIT);  // Initialization Mode For Filters
    
        match list {
            true    =>      pointer::set_ptr_vol_bit_u32(self.fs1r, bit),
            false   =>      pointer::clr_ptr_vol_bit_u32(self.fs1r, bit),
        };

        match fifo {
            true    =>      pointer::set_ptr_vol_bit_u32(self.ffa1r, bit),
            false   =>      pointer::clr_ptr_vol_bit_u32(self.ffa1r, bit),
        };

        match active {
            true    =>      pointer::set_ptr_vol_bit_u32(self.fa1r, bit),
            false   =>      pointer::clr_ptr_vol_bit_u32(self.fa1r, bit),
        };

        pointer::set_ptr_vol_raw_u32(fxr1, mask);
        pointer::set_ptr_vol_raw_u32(fxr2, mask);

        pointer::clr_ptr_vol_bit_u32(self.fmr, FINIT);  // Active Mode For Filters
    }

    /* Baud Rate Calc */
    // Baud Rate = 1 / NominalBitTime
    // NominalBitTime = 1 * tq + tbs1 + tbs2
    // tbs1 = tq * (TS1[3:0] + 1),
    // tbs2 = tq * (TS2[2:0] + 1),
    // tq = (BRP[9:0] + 1) * tpclk
    // tpclk = time period of the APB Clock
    // CURRENTLY USING http://www.bittiming.can-wiki.info/
    /* Example Table */
    /* 16 Mhz 
    Bit     accuracy    Pre-        TQ      TS1     TS2     Acc     Reg
    Rate                scaler 			
    1000	0.0000	    1	        16	    13	    2	    87.5	0x001c0000
    500	    0.0000	    2	        16	    13	    2	    87.5	0x001c0001
    250	    0.0000	    4	        16	    13	    2	    87.5	0x001c0003
    125	    0.0000	    8	        16	    13	    2	    87.5	0x001c0007
    */

    fn clock_setup(&self, ci: &CanInit) {
        /* Due to the + 1 in all calcs we remove 1 from all */
        pointer::set_ptr_vol_u32(self.btr, BRP_OFFSET, BRP_MASK, ci.brp);
        pointer::set_ptr_vol_u32(self.btr, TS1_OFFSET, TS1_MASK, ci.ts1);
        pointer::set_ptr_vol_u32(self.btr, TS2_OFFSET, TS2_MASK, ci.ts2);
        pointer::set_ptr_vol_u32(self.btr, SJW_OFFSET, SJW_MASK, ci.sjw);
    }
}

impl CanMsg {
    pub fn init() -> CanMsg {
        return CanMsg {
            id:         0, 
            ide:        false,
            rtr:        false,
            dlc:        0,
            fmi:        0,
            data:       [0; 8]
        };
    }

    pub fn get_id(&self) -> u32 {
        return self.id;
    }

    pub fn set_id(&mut self, id: u32, ide: bool) {
        self.id = id;
        self.ide = ide;
    }

    pub fn get_rtr(&self) -> bool {
        return self.rtr;
    }

    pub fn clr_rtr(&mut self) {
        self.rtr = false;
    }

    pub fn set_rtr(&mut self) {
        self.rtr = true;
    }

    pub fn get_dlc(&self) -> u32 {
        return self.dlc;
    }

    pub fn set_dlc(&mut self, dlc: u32) {
        self.dlc = dlc;
    }

    pub fn get_data(&self) -> [u8; 8] {
        return self.data;
    }

    pub fn set_data(&mut self, data: [u8; 8]) {
        self.data = data;
    }
}

impl CanInit {
    pub fn init() -> CanInit {
        return CanInit {
            txfp:       false,      // Transmit FIFO Priority
            rflm:       false,      // Receive FIFO Locked mode
            nart:       false,      // No Automatic Retransmission
            awum:       false,      // Automatic Wakeup Mode
            abom:       true,       // Automatic Bus-off Management
            ttcm:       false,
            // ALL OF THESE ARE + 1 within BTR, so from http://www.bittiming.can-wiki.info/ take all and - 1
            brp:        1,          // Baud Rate Prescaler
            ts1:        12,         // Time Segment 1
            ts2:        1,          // Time Segment 2
            sjw:        0           // Resynchronization Jump Width
        }
    }
}
