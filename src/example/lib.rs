// EASE OF UNDERSTANDING EXAMPLE OF HOW TO READ AND WRITE FROM A CHIP SET THIS IS AN EXAMPLE OF AN STM32L432KC Nucleo Board,
#![no_std]
use core::ptr;

// General Registers
const MODER:        u32 = 0x00;
const OTYPER:       u32 = 0x04;
const BSRR:         u32 = 0x18;

const GPIOB_BASE:   u32 = 0x48000400;
const GPIOB_MODER:  u32 = GPIOB_BASE + MODER; 
const GPIOB_OTYPER: u32 = GPIOB_BASE + OTYPER; 
const GPIOB_BSRR:   u32 = GPIOB_BASE + BSRR; 

const GPIOA_CLOCK:  u32 = 0;
const GPIOB_CLOCK:  u32 = 1;

// GPIO BASE AND LED PIN NUMBER
const USER_LED1:    u32 = 3;

const RCC_BASE:     u32 = 0x40021000;
const RCC_AHB2ENR:  u32 = RCC_BASE + 0x4C;

#[no_mangle]
pub extern fn system_init() {
    // RCC SHOULD ALWAYS BE IN THE SYSTEM INIT TRYING TO OPERATE THE GPIO PINS EVEN ACTIVATING WILL CAUSE ISSUES
    unsafe{ptr::write_volatile((RCC_AHB2ENR) as *mut u32, (1<<GPIOB_CLOCK) | (1<<GPIOA_CLOCK))};  // EN CLK FOR GPIO B and A
}

#[no_mangle]
pub extern fn start() {
    // Initialize the LED on L432KC board  
    unsafe{ptr::write_volatile((GPIOB_MODER) as *mut u32, 1<<(USER_LED1 * 2))};
    unsafe{ptr::write_volatile((GPIOB_OTYPER) as *mut u32, !(1<<(USER_LED1)))};

    let mut i = 0;

    loop
    {

        while i <= 10000000 {
            if i == 5000000 {
                unsafe{ptr::write_volatile((GPIOB_BSRR) as *mut u32, 1<<USER_LED1)};
            } else if i == 0 {
                unsafe{ptr::write_volatile((GPIOB_BSRR) as *mut u32, 1<<(USER_LED1 + 16))}
            }
            i += 1;
        }
        i = 0
	}
}

#[no_mangle]
pub extern "C" fn __aeabi_unwind_cpp_pr0() {
    loop {}
}

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
