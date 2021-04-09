// EASE OF UNDERSTANDING EXAMPLE OF HOW TO READ AND WRITE FROM A CHIP SET THIS IS AN EXAMPLE OF AN STM32L432KC Nucleo Board,
#![no_std]
/* Use Core As STD Library Will Not Work Here */
/* Call Pointer */
use core::ptr;
use core::panic::PanicInfo;

/* GPIO Registers */
const MODER:        u32 = 0x00;
const OTYPER:       u32 = 0x04;
const BSRR:         u32 = 0x18;

/* Combine GPIO Base with Register Offset To Get Hardware Register Location */
const GPIOB_BASE:   u32 = 0x48000400;
const GPIOB_MODER:  u32 = GPIOB_BASE + MODER; 
const GPIOB_OTYPER: u32 = GPIOB_BASE + OTYPER; 
const GPIOB_BSRR:   u32 = GPIOB_BASE + BSRR; 

/* RCC GPIO Clock Enable Bit */
const GPIOA_CLOCK:  u32 = 0;
const GPIOB_CLOCK:  u32 = 1;

// GPIO BASE AND LED PIN NUMBER
const USER_LED1:    u32 = 3;

/* Combine RCC Base with Register Offset To Get Hardware Register Location */
const RCC_BASE:     u32 = 0x40021000;
const RCC_AHB2ENR:  u32 = RCC_BASE + 0x4C;

#[no_mangle]
pub extern fn system_init() {
    // RCC SHOULD ALWAYS BE IN THE SYSTEM INIT TRYING TO OPERATE THE GPIO PINS EVEN ACTIVATING WILL CAUSE ISSUES AS THERE IS NO CLOCK TO RUN
    unsafe{ptr::write_volatile((RCC_AHB2ENR) as *mut u32, (1<<GPIOB_CLOCK) | (1<<GPIOA_CLOCK))};  // EN CLK FOR GPIO B and A
}

#[no_mangle]
pub extern fn start() {
    /* Initialize the LED on L432KC board */ 
    /* Form a Pointer Via Register Address And Write Volitile To That Address */
    /* From Page 267 0x01 = General Output As The Output For The USER LED is at 3 it is x 2 */
    unsafe{ptr::write_volatile((GPIOB_MODER) as *mut u32, 1<<(USER_LED1 * 2))};
    /* From Page 268 0x0 = Push Pull From Board Docs, LED is Push Pull so we ensure the bit is not set by inverting
    In Practice this would set all others to open drain but since we are running only 1 output here we can get away with it */
    unsafe{ptr::write_volatile((GPIOB_OTYPER) as *mut u32, !(1<<(USER_LED1)))};

    let mut i = 0;

    loop {
        while i <= 10000000 {
            if i == 5000000 {
                /* From Page 270 0x1 turns on the output and offset + 16 will reset the output when set*/
                unsafe{ptr::write_volatile((GPIOB_BSRR) as *mut u32, 1<<USER_LED1)};
            } else if i == 0 {
                /* From Page 270 0x1 turns on the output and offset + 16 will reset the output when set*/
                unsafe{ptr::write_volatile((GPIOB_BSRR) as *mut u32, 1<<(USER_LED1 + 16))}
            }
            i += 1;
        }
        i = 0
	}
}

/* Required if an unwind of the program happens */
#[no_mangle]
pub extern "C" fn __aeabi_unwind_cpp_pr0() {
    loop {}
}

/* Required if a panic of the program happens */
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
