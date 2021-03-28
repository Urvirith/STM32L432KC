/* Timers */
/* Manual Page 965 */

use super::common;
use super::pointer;

pub struct Timer {
    cr1:        *mut u32,       // Control Register 1
    cr2:        *mut u32,       // Control Register 2
    smcr:       *mut u32,       // Slave Mode Control Register
    dier:       *mut u32,       // DMA/ Interrupt Enable Register
    sr:         *mut u32,       // Status Register
    egr:        *mut u32,       // Event Generation Register
    ccmr1:      *mut u32,       // Capture/Compare Mode Register 1
    ccmr2:      *mut u32,       // Capture/Compare Mode Register 2
    ccer:       *mut u32,       // Capture/Compare Enable Register
    cnt:        *mut u32,       // Counter Register
    psc:        *mut u32,       // Prescaler Register
    arr:        *mut u32,       // Auto Reload Register
    ccr1:       *mut u32,       // Capture/Compare Register 1
    ccr2:       *mut u32,       // Capture/Compare Register 2
    ccr3:       *mut u32,       // Capture/Compare Register 3
    ccr4:       *mut u32,       // Capture/Compare Register 4
    dcr:        *mut u32,       // DMA Control Register
    dmar:       *mut u32,       // DMA Address Register
    or:         *mut u32,       // Option Register
}

/* Register Offset */
const CR1:      u32 = 0x00;
const CR2:      u32 = 0x04;
const SMCR:     u32 = 0x08;
const DIER:     u32 = 0x0C;
const SR:       u32 = 0x10;
const EGR:      u32 = 0x14;
const CCMR1:    u32 = 0x18;
const CCMR2:    u32 = 0x1C;
const CCER:     u32 = 0x20;
const CNT:      u32 = 0x24;
const PSC:      u32 = 0x28;
const ARR:      u32 = 0x2C;
const CCR1:     u32 = 0x34;
const CCR2:     u32 = 0x38;
const CCR3:     u32 = 0x3C;
const CCR4:     u32 = 0x40;
const DCR:      u32 = 0x48;
const DMAR:     u32 = 0x4C;
const OR:       u32 = 0x50;

/* Enumerations */
// 0 = Continous, 1 = ONS (clears en bit)
pub enum TimerType {Cont, Ons}

// 0 = Upcounter, 1 = Downcounter
pub enum Direction {Upcount, Downcount}

/* Register Masks */
/* CR1 */
const CMS_MASK:         u32 = common::MASK_2_BIT;       /* Mode is mask required, here we set the mask to two bit 11 */
const CKD_MASK:         u32 = common::MASK_2_BIT;       /* Mode is mask required, here we set the mask to two bit 11 */

/* Register Bits */
/* CR1 */
const EN_BIT:           u32 = common::BIT_0;            /* 0 = Disabled, 1 = Enabled */
const UDIS_BIT:         u32 = common::BIT_1;            /* Update event, 0 = EN, 1 = DISABLED */
const URS_BIT:          u32 = common::BIT_2;            /* 0 = All events enables, 1 = Only OF or UF Events */
const OPM_BIT:          u32 = common::BIT_3;            /* 0 = Continous, 1 = ONS (clears en bit) */
const DIR_BIT:          u32 = common::BIT_4;            /* 0 = Upcounter, 1 = Downcounter (ONLY ACTIVE IF CMS = 00) */
const ARPE_BIT:         u32 = common::BIT_7;            /* 0 ARR = Not Buffered, 1 = Buffered */
const UIFREMAP_BIT:     u32 = common::BIT_11;           /* Output, 0 = Pulse, 1 = Toggle */

/* SR */
const UPDATE_BIT:       u32 = common::BIT_0;

/* Register Offsets */
/* CR1 */
const CMS_OFFSET:       u32 = 5;                        /* 00 = Edge Aligned     01 = Center Aligned Down     10 = Center Aligned Up     11 - Center Aligned Up */
const CKD_OFFSET:       u32 = 8;                        /* 00 = Tdts = Tclk_int  01 = Tdts = 2*Tclk_int       10 = Tdts = 4*Tclk_int     11 - Reserved */

/* CNT */
const CLEAR_CNT:        u32 = 0;  

impl Timer {
    pub fn init(base: u32) -> Timer {
        return Timer {
            cr1:    (base + CR1)    as *mut u32,
            cr2:    (base + CR2)    as *mut u32,
            smcr:   (base + SMCR)   as *mut u32,
            dier:   (base + DIER)   as *mut u32,
            sr:     (base + SR)     as *mut u32,
            egr:    (base + EGR)    as *mut u32,
            ccmr1:  (base + CCMR1)  as *mut u32,
            ccmr2:  (base + CCMR2)  as *mut u32,
            ccer:   (base + CCER)   as *mut u32,
            cnt:    (base + CNT)    as *mut u32,
            psc:    (base + PSC)    as *mut u32,
            arr:    (base + ARR)    as *mut u32,
            ccr1:   (base + CCR1)   as *mut u32,
            ccr2:   (base + CCR2)   as *mut u32,
            ccr3:   (base + CCR3)   as *mut u32,
            ccr4:   (base + CCR4)   as *mut u32,
            dcr:    (base + DCR)    as *mut u32,
            dmar:   (base + DMAR)   as *mut u32,
            or:     (base + OR)     as *mut u32
        };
    }

    pub fn open(&self, timer_type: TimerType, dir: Direction) {
        pointer::clr_ptr_vol_bit_u32(self.cr1, UDIS_BIT);
        pointer::clr_ptr_vol_bit_u32(self.cr1, URS_BIT);
    
        match timer_type {
            TimerType::Ons          =>      pointer::set_ptr_vol_bit_u32(self.cr1, OPM_BIT),
            TimerType::Cont         =>      pointer::clr_ptr_vol_bit_u32(self.cr1, OPM_BIT)
        }
    
        match dir {
            Direction::Downcount    =>      pointer::set_ptr_vol_bit_u32(self.cr1, DIR_BIT),
            Direction::Upcount      =>      pointer::clr_ptr_vol_bit_u32(self.cr1, DIR_BIT)
        }

        pointer::set_ptr_vol_u32(self.cr1, CMS_OFFSET, CMS_MASK, 0);
        pointer::clr_ptr_vol_bit_u32(self.cr1, ARPE_BIT);
        pointer::set_ptr_vol_u32(self.cr1, CKD_OFFSET, CKD_MASK, 0);
        pointer::clr_ptr_vol_bit_u32(self.cr1, UIFREMAP_BIT);
    }
    
    pub fn get_flag(&self) -> bool {
        return pointer::get_ptr_vol_bit_u32(self.sr, UPDATE_BIT);
    }
    
    pub fn clr_flag(&self) {
        pointer::clr_ptr_vol_bit_u32(self.sr, UPDATE_BIT);
    }
    
    pub fn read_count(&self) -> u32 {
        return pointer::get_ptr_vol_raw_u32(self.cnt);
    }
    
    pub fn clr_count(&self) {
        pointer::set_ptr_vol_raw_u32(self.cnt, CLEAR_CNT);
    }
    
    pub fn start(&self) {
        pointer::set_ptr_vol_bit_u32(self.cr1, EN_BIT);
    }
    
    pub fn stop(&self,) {
        pointer::clr_ptr_vol_bit_u32(self.cr1, EN_BIT);
    }
    
    pub fn set_time(&self, time: u32, clock_speed: u32, prescale: u32) {  
        let val;
        
        if prescale == 0 {
            val = time * clock_speed;
        } else {
            val = (time * clock_speed) / prescale;
        }
    
        pointer::set_ptr_vol_raw_u32(self.psc, prescale);
        pointer::set_ptr_vol_raw_u32(self.arr, val);
    }
}
