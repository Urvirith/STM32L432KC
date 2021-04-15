/* CAN (Controller Area Network) */
/* Manual Page 1476 */

pub struct Can {
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

impl Can {
    /* Initialize The Structure */
    pub fn init(base: u32) -> Can {
        return Can {
            msr:        (base + MSR)        as *mut u32,
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
}