/* Reset and Clock Control (RCC) */
/* Manual Page 195 */

use super::{common, pointer};

pub struct Rcc {
    cr:             *mut u32,       // Control Register
    icscr:          *mut u32,       // Internal Clock Sources Calibration Register
    cfgr:           *mut u32,       // Clock Configuration Register 
    pll_cfgr:       *mut u32,       // PLL Configuration Register
    pll_sai1_cfgr:  *mut u32,       // PLL SAI1 Configuration Register
    cier:           *mut u32,       // Clock Interrupt Enable Register
    cifr:           *mut u32,       // Clock Interrupt Flag Status Register
    cicr:           *mut u32,       // Clock Interrupt Clear Register
    ahb1_rstr:      *mut u32,       // AHB1 Peripheral Reset Register
    ahb2_rstr:      *mut u32,       // AHB2 Peripheral Reset Register
    ahb3_rstr:      *mut u32,       // AHB3 Peripheral Reset Register
    apb1_rstr1:     *mut u32,       // APB1 Peripheral Reset Register 1
    apb1_rstr2:     *mut u32,       // APB1 Peripheral Reset Register 2
    apb2_rstr:      *mut u32,       // APB2 Peripheral Reset Register
    ahb1_enr:       *mut u32,       // AHB1 Peripheral Enable Register
    ahb2_enr:       *mut u32,       // AHB2 Peripheral Enable Register
    ahb3_enr:       *mut u32,       // AHB3 Peripheral Enable Register
    apb1_enr1:      *mut u32,       // APB1 Peripheral Enable Register 1
    apb1_enr2:      *mut u32,       // APB1 Peripheral Enable Register 2
    apb2_enr:       *mut u32,       // APB2 Peripheral Enable Register
    ahb1_sm_enr:    *mut u32,       // AHB1 Peripheral Sleep And Stop Modes Enable Register
    ahb2_sm_enr:    *mut u32,       // AHB2 Peripheral Sleep And Stop Modes Enable Register
    ahb3_sm_enr:    *mut u32,       // AHB3 Peripheral Sleep And Stop Modes Enable Register
    apb1_sm_enr1:   *mut u32,       // APB1 Peripheral Sleep And Stop Modes Enable Register 1
    apb1_sm_enr2:   *mut u32,       // APB1 Peripheral Sleep And Stop Modes Enable Register 2
    apb2_sm_enr:    *mut u32,       // APB2 Peripheral Sleep And Stop Modes Enable Register
    ccipr1:         *mut u32,       // Peripherals Independent Clock Configuration Register
    bdcr:           *mut u32,       // Backup Domain Control Register
    csr:            *mut u32,       // Control Status Register
    crrcr:          *mut u32,       // Clock Recovery RC Register
    ccipr2:         *mut u32        // Peripherals Independent Clock Configuration Register
}

/* Register Offset */
const CR:               u32 = 0x00;
const ICSCR:            u32 = 0x04;
const CFGR:             u32 = 0x08;
const PLL_CFGR:         u32 = 0x0C;
const PLL_SAI1_CFGR:    u32 = 0x10;
const CIER:             u32 = 0x18;
const CIFR:             u32 = 0x1C;
const CICR:             u32 = 0x20;
const AHB1_RSTR:        u32 = 0x28;
const AHB2_RSTR:        u32 = 0x2C;
const AHB3_RSTR:        u32 = 0x30;
const APB1_RSTR1:       u32 = 0x38;
const APB1_RSTR2:       u32 = 0x3C;
const APB2_RSTR:        u32 = 0x40;
const AHB1_ENR:         u32 = 0x48;
const AHB2_ENR:         u32 = 0x4C;
const AHB3_ENR:         u32 = 0x50;
const APB1_ENR1:        u32 = 0x58;
const APB1_ENR2:        u32 = 0x5C;
const APB2_ENR:         u32 = 0x60;
const AHB1_SM_ENR:      u32 = 0x68;
const AHB2_SM_ENR:      u32 = 0x6C;
const AHB3_SM_ENR:      u32 = 0x70;
const APB1_SM_ENR1:     u32 = 0x78;
const APB1_SM_ENR2:     u32 = 0x7C;
const APB2_SM_ENR:      u32 = 0x80;
const CCIPR1:           u32 = 0x88;
const BDCR:             u32 = 0x90;
const CSR:              u32 = 0x94;
const CRRCR:            u32 = 0x98;
const CCIPR2:           u32 = 0x9C;

/* Register Bits */
const MSI_ON_BIT:           u32 = common::BIT_0;
const MSI_RANGE_BIT:        u32 = common::BIT_3;

/* Register Masks */
const MSI_RANGE_MASK:       u32 = common::MASK_4_BIT;       /* MSI is mask required, here we set the mask to four bit 1111 */

/* Register Offsets */
const MSI_RANGE_OFFSET:     u32 = 4;                        /* MSI Offset is 4 Bits */

impl Rcc {
    /* Initialize The Structure */
    pub fn init(base: u32) -> Rcc {
        return Rcc {
            cr:             (base + CR)             as *mut u32,
            icscr:          (base + ICSCR)          as *mut u32,
            cfgr:           (base + CFGR)           as *mut u32,
            pll_cfgr:       (base + PLL_CFGR)       as *mut u32,
            pll_sai1_cfgr:  (base + PLL_SAI1_CFGR)  as *mut u32,
            cier:           (base + CIER)           as *mut u32,
            cifr:           (base + CIFR)           as *mut u32,
            cicr:           (base + CICR)           as *mut u32,
            ahb1_rstr:      (base + AHB1_RSTR)      as *mut u32,
            ahb2_rstr:      (base + AHB2_RSTR)      as *mut u32,
            ahb3_rstr:      (base + AHB3_RSTR)      as *mut u32,
            apb1_rstr1:     (base + APB1_RSTR1)     as *mut u32,
            apb1_rstr2:     (base + APB1_RSTR2)     as *mut u32,
            apb2_rstr:      (base + APB2_RSTR)      as *mut u32,
            ahb1_enr:       (base + AHB1_ENR)       as *mut u32,
            ahb2_enr:       (base + AHB2_ENR)       as *mut u32,
            ahb3_enr:       (base + AHB3_ENR)       as *mut u32,
            apb1_enr1:      (base + APB1_ENR1)      as *mut u32,
            apb1_enr2:      (base + APB1_ENR2)      as *mut u32,
            apb2_enr:       (base + APB2_ENR)       as *mut u32,
            ahb1_sm_enr:    (base + AHB1_SM_ENR)    as *mut u32,
            ahb2_sm_enr:    (base + AHB2_SM_ENR)    as *mut u32,
            ahb3_sm_enr:    (base + AHB3_SM_ENR)    as *mut u32,
            apb1_sm_enr1:   (base + APB1_SM_ENR1)   as *mut u32,
            apb1_sm_enr2:   (base + APB1_SM_ENR2)   as *mut u32,
            apb2_sm_enr:    (base + APB2_SM_ENR)    as *mut u32,
            ccipr1:         (base + CCIPR1)         as *mut u32,
            bdcr:           (base + BDCR)           as *mut u32,
            csr:            (base + CSR)            as *mut u32,
            crrcr:          (base + CRRCR)          as *mut u32,
            ccipr2:         (base + CCIPR2)         as *mut u32,
        };
    }

    /* Set the clock speed of the chipset */
    pub fn write_msi_range(&self, rng: common::MsiRange) {
        pointer::clr_ptr_vol_bit_u32(self.cr, MSI_ON_BIT);
        pointer::set_ptr_vol_u32(self.cr, MSI_RANGE_OFFSET, MSI_RANGE_MASK, rng as u32);
        pointer::set_ptr_vol_bit_u32(self.cr, MSI_RANGE_BIT);
        pointer::set_ptr_vol_bit_u32(self.cr, MSI_ON_BIT);
    }

    pub fn write_ahb1_enr(&self, val: u32) {
        pointer::set_ptr_vol_bit_u32(self.ahb1_enr, val);
    }
    
    pub fn write_ahb2_enr(&self, val: u32) {
        pointer::set_ptr_vol_bit_u32(self.ahb2_enr, val);
    }
    
    pub fn write_ahb3_enr(&self, val: u32) {
        pointer::set_ptr_vol_bit_u32(self.ahb3_enr, val);
    }
    
    pub fn write_apb1_enr1(&self, val: u32) {
        pointer::set_ptr_vol_bit_u32(self.apb1_enr1, val);
    }
    
    pub fn write_apb1_enr2(&self, val: u32) {
        pointer::set_ptr_vol_bit_u32(self.apb1_enr2, val);
    }
    
    pub fn write_apb2_enr(&self, val: u32) {
        pointer::set_ptr_vol_bit_u32(self.apb2_enr, val);
    }
}
