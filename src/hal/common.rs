
use core::ptr;

/* Standard Bit Shifts for 32-bit words */
pub const BIT_0:        u32 = 1 << 0;
pub const BIT_1:        u32 = 1 << 1;
pub const BIT_2:        u32 = 1 << 2;
pub const BIT_3:        u32 = 1 << 3;
pub const BIT_4:        u32 = 1 << 4;
pub const BIT_5:        u32 = 1 << 5;
pub const BIT_6:        u32 = 1 << 6;
pub const BIT_7:        u32 = 1 << 7;
pub const BIT_8:        u32 = 1 << 8;
pub const BIT_9:        u32 = 1 << 9;
pub const BIT_10:       u32 = 1 << 10;
pub const BIT_11:       u32 = 1 << 11;
pub const BIT_12:       u32 = 1 << 12;
pub const BIT_13:       u32 = 1 << 13;
pub const BIT_14:       u32 = 1 << 14;
pub const BIT_15:       u32 = 1 << 15;
pub const BIT_16:       u32 = 1 << 16;
pub const BIT_17:       u32 = 1 << 17;
pub const BIT_18:       u32 = 1 << 18;
pub const BIT_19:       u32 = 1 << 19;
pub const BIT_20:       u32 = 1 << 20;
pub const BIT_21:       u32 = 1 << 21;
pub const BIT_22:       u32 = 1 << 22;
pub const BIT_23:       u32 = 1 << 23;
pub const BIT_24:       u32 = 1 << 24;
pub const BIT_25:       u32 = 1 << 25;
pub const BIT_26:       u32 = 1 << 26;
pub const BIT_27:       u32 = 1 << 27;
pub const BIT_28:       u32 = 1 << 28;
pub const BIT_29:       u32 = 1 << 29;
pub const BIT_30:       u32 = 1 << 30;
pub const BIT_31:       u32 = 1 << 31;

/* Standard Bit Masks for 32-bit words */
pub const MASK_1_BIT:   u32 = 0x00000001;
pub const MASK_2_BIT:   u32 = 0x00000003;
pub const MASK_3_BIT:   u32 = 0x00000007;
pub const MASK_4_BIT:   u32 = 0x0000000F;
pub const MASK_5_BIT:   u32 = 0x0000001F;
pub const MASK_6_BIT:   u32 = 0x0000003F;
pub const MASK_7_BIT:   u32 = 0x0000007F;
pub const MASK_8_BIT:   u32 = 0x000000FF;
pub const MASK_9_BIT:   u32 = 0x000001FF;
pub const MASK_10_BIT:  u32 = 0x000003FF;
pub const MASK_11_BIT:  u32 = 0x000007FF;
pub const MASK_12_BIT:  u32 = 0x00000FFF;
pub const MASK_13_BIT:  u32 = 0x00001FFF;
pub const MASK_14_BIT:  u32 = 0x00003FFF;
pub const MASK_15_BIT:  u32 = 0x00007FFF;
pub const MASK_16_BIT:  u32 = 0x0000FFFF;
pub const MASK_17_BIT:  u32 = 0x0001FFFF;
pub const MASK_18_BIT:  u32 = 0x0003FFFF;
pub const MASK_19_BIT:  u32 = 0x0007FFFF;
pub const MASK_20_BIT:  u32 = 0x000FFFFF;
pub const MASK_21_BIT:  u32 = 0x001FFFFF;
pub const MASK_22_BIT:  u32 = 0x003FFFFF;
pub const MASK_23_BIT:  u32 = 0x007FFFFF;
pub const MASK_24_BIT:  u32 = 0x00FFFFFF;
pub const MASK_25_BIT:  u32 = 0x01FFFFFF;
pub const MASK_26_BIT:  u32 = 0x03FFFFFF;
pub const MASK_27_BIT:  u32 = 0x07FFFFFF;
pub const MASK_28_BIT:  u32 = 0x0FFFFFFF;
pub const MASK_29_BIT:  u32 = 0x1FFFFFFF;
pub const MASK_30_BIT:  u32 = 0x3FFFFFFF;
pub const MASK_31_BIT:  u32 = 0x7FFFFFFF;
pub const MASK_32_BIT:  u32 = 0xFFFFFFFF;

/* Bool Handling */
pub fn get_ptr_vol_bit_u32(addr: *mut u32, val: u32) -> bool {
    if (get_ptr_vol_raw_u32(addr) & val) > 0 {
        return true;
    } else {
        return false;
    }
}

pub fn set_ptr_vol_bit_u32(addr: *mut u32, val: u32) {
    let mut reg = get_ptr_vol_raw_u32(addr);

    reg |= val;

    set_ptr_vol_raw_u32(addr, reg);
}

pub fn clr_ptr_vol_bit_u32(addr: *mut u32, val: u32) {
    let mut reg = get_ptr_vol_raw_u32(addr);

    reg &= !val;

    set_ptr_vol_raw_u32(addr, reg);
}

/* Bool Handling */
pub fn get_ptr_vol_u32(addr: *mut u32, offset: u32, mask: u32) -> u32 {
    return (get_ptr_vol_raw_u32(addr) >> offset) & mask;
}

pub fn set_ptr_vol_u32(addr: *mut u32, offset: u32, mask: u32, val: u32) {
    let mut reg = get_ptr_vol_raw_u32(addr);

    reg &= !(mask << offset);
    reg |= val << offset;

    set_ptr_vol_raw_u32(addr, reg);
}

/* Unsafe Access To Pointers */
/* 32 Bit Pointer */
pub fn get_ptr_vol_raw_u32(addr: *mut u32) -> u32 {
    return unsafe { ptr::read_volatile(addr) };
}

pub fn set_ptr_vol_raw_u32(addr: *mut u32, val: u32) {
    unsafe { ptr::write_volatile(addr, val) };
}

/* 16 Bit Pointer */
pub fn get_ptr_vol_raw_u16(addr: *mut u16) -> u16 {
    return unsafe { ptr::read_volatile(addr) };
}

pub fn set_ptr_vol_raw_u16(addr: *mut u16, val: u16) {
    unsafe { ptr::write_volatile(addr, val) };
}

/* 8 Bit Pointer */
pub fn get_ptr_vol_raw_u8(addr: *mut u8) -> u8 {
    return unsafe { ptr::read_volatile(addr) };
}

pub fn set_ptr_vol_raw_u8(addr: *mut u8, val: u8) {
    unsafe { ptr::write_volatile(addr, val) };
}

/* Create raw pointers, can inline */
/*
#[inline(always)]
fn raw_ptr_32bit(address: u32) -> *mut u32 {
    return(address) as *mut u32;
}

#[inline(always)]
fn raw_ptr_16bit(address: u32) -> *mut u16 {
    return(address) as *mut u16;
}

#[inline(always)]
fn raw_ptr_8bit(address: u32) -> *mut u8 {
    return(address) as *mut u8;
}
*/