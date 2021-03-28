/* Unsafe Area For Pointer Functions */
use core::ptr;

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
