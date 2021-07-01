
//https://github.com/rust-lang/compiler-builtins/tree/master/src
//https://github.com/rust-lang/compiler-builtins/blob/master/src/arm.rs

#[cfg(not(target_os = "ios"))]
#[cfg_attr(not(feature = "mangled-names"), no_mangle)]
pub unsafe extern "aapcs" fn __aeabi_memcpy(dest: *mut u8, src: *const u8, n: usize) {
    memcpy(dest, src, n);
}

#[cfg(not(target_os = "ios"))]
#[cfg_attr(not(feature = "mangled-names"), no_mangle)]
pub unsafe extern "aapcs" fn __aeabi_memcpy4(dest: *mut u8, src: *const u8, mut n: usize) {
    // We are guaranteed 4-alignment, so accessing at u32 is okay.
    let mut dest = dest as *mut u32;
    let mut src = src as *mut u32;

    while n >= 4 {
        *dest = *src;
        dest = dest.offset(1);
        src = src.offset(1);
        n -= 4;
    }

    __aeabi_memcpy(dest as *mut u8, src as *const u8, n);
}

#[cfg(not(target_os = "ios"))]
#[cfg_attr(not(feature = "mangled-names"), no_mangle)]
pub unsafe extern "aapcs" fn __aeabi_memcpy8(dest: *mut u8, src: *const u8, n: usize) {
    __aeabi_memcpy4(dest, src, n);
}

#[cfg(not(target_os = "ios"))]
#[cfg_attr(not(feature = "mangled-names"), no_mangle)]
pub unsafe extern "aapcs" fn __aeabi_memmove(dest: *mut u8, src: *const u8, n: usize) {
    memmove(dest, src, n);
}

#[cfg(not(any(target_os = "ios", target_env = "msvc")))]
#[cfg_attr(not(feature = "mangled-names"), no_mangle)]
pub unsafe extern "aapcs" fn __aeabi_memmove4(dest: *mut u8, src: *const u8, n: usize) {
    __aeabi_memmove(dest, src, n);
}

#[cfg(not(any(target_os = "ios", target_env = "msvc")))]
#[cfg_attr(not(feature = "mangled-names"), no_mangle)]
pub unsafe extern "aapcs" fn __aeabi_memmove8(dest: *mut u8, src: *const u8, n: usize) {
    __aeabi_memmove(dest, src, n);
}

#[cfg(not(target_os = "ios"))]
#[cfg_attr(not(feature = "mangled-names"), no_mangle)]
pub unsafe extern "aapcs" fn __aeabi_memset(dest: *mut u8, n: usize, c: i32) {
    // Note the different argument order
    memset(dest, c, n);
}

#[cfg(not(target_os = "ios"))]
#[cfg_attr(not(feature = "mangled-names"), no_mangle)]
pub unsafe extern "aapcs" fn __aeabi_memset4(dest: *mut u8, mut n: usize, c: i32) {
    let mut dest = dest as *mut u32;

    let byte = (c as u32) & 0xff;
    let c = (byte << 24) | (byte << 16) | (byte << 8) | byte;

    while n >= 4 {
        *dest = c;
        dest = dest.offset(1);
        n -= 4;
    }

    __aeabi_memset(dest as *mut u8, n, byte as i32);
}

#[cfg(not(target_os = "ios"))]
#[cfg_attr(not(feature = "mangled-names"), no_mangle)]
pub unsafe extern "aapcs" fn __aeabi_memset8(dest: *mut u8, n: usize, c: i32) {
    __aeabi_memset4(dest, n, c);
}

#[cfg(not(target_os = "ios"))]
#[cfg_attr(not(feature = "mangled-names"), no_mangle)]
pub unsafe extern "aapcs" fn __aeabi_memclr(dest: *mut u8, n: usize) {
    __aeabi_memset(dest, n, 0);
}

#[cfg(not(any(target_os = "ios", target_env = "msvc")))]
#[cfg_attr(not(feature = "mangled-names"), no_mangle)]
pub unsafe extern "aapcs" fn __aeabi_memclr4(dest: *mut u8, n: usize) {
    __aeabi_memset4(dest, n, 0);
}

#[cfg(not(any(target_os = "ios", target_env = "msvc")))]
#[cfg_attr(not(feature = "mangled-names"), no_mangle)]
pub unsafe extern "aapcs" fn __aeabi_memclr8(dest: *mut u8, n: usize) {
    __aeabi_memset4(dest, n, 0);
}

//https://github.com/rust-lang/compiler-builtins/blob/master/src/arm.rs
#[cfg_attr(all(feature = "mem", not(feature = "mangled-names")), no_mangle)]
#[cfg(not(any(target_os = "windows", target_env = "gnu")))]
pub unsafe extern "C" fn memcpy(dest: *mut u8, src: *const u8, n: usize) -> *mut u8 {
    copy_forward(dest, src, n);
    dest
}

#[cfg_attr(all(feature = "mem", not(feature = "mangled-names")), no_mangle)]
#[cfg(not(any(target_os = "windows", target_env = "gnu")))]
pub unsafe extern "C" fn memmove(dest: *mut u8, src: *const u8, n: usize) -> *mut u8 {
    let delta = (dest as usize).wrapping_sub(src as usize);
    if delta >= n {
        // We can copy forwards because either dest is far enough ahead of src,
        // or src is ahead of dest (and delta overflowed).
        copy_forward(dest, src, n);
    } else {
        copy_backward(dest, src, n);
    }
    dest
}

#[cfg_attr(all(feature = "mem", not(feature = "mangled-names")), no_mangle)]
#[cfg(not(any(target_os = "windows", target_env = "gnu")))]
pub unsafe extern "C" fn memset(s: *mut u8, c: i32, n: usize) -> *mut u8 {
    set_bytes(s, c as u8, n);
    s
}

// https://github.com/rust-lang/compiler-builtins/blob/master/src/mem/impls.rs
#[inline(always)]
pub unsafe fn copy_forward(dest: *mut u8, src: *const u8, n: usize) {
    let mut i = 0;
    while i < n {
        *dest.add(i) = *src.add(i);
        i += 1;
    }
}

#[inline(always)]
pub unsafe fn copy_backward(dest: *mut u8, src: *const u8, n: usize) {
    // copy from end
    let mut i = n;
    while i != 0 {
        i -= 1;
        *dest.add(i) = *src.add(i);
    }
}

#[inline(always)]
pub unsafe fn set_bytes(s: *mut u8, c: u8, n: usize) {
    let mut i = 0;
    while i < n {
        *s.add(i) = c;
        i += 1;
    }
}