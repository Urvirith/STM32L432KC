#![no_std]
use core::panic::PanicInfo;
mod board;
mod hal;

#[no_mangle]
pub extern fn system_init() {
    /* RCC Enabling of the bus */
    let rcc = hal::rcc::Rcc::init(board::l432kc::RCC_BASE);
    rcc.write_ahb2_enr(board::l432kc::GPIOA_CLOCK);
    rcc.write_ahb2_enr(board::l432kc::GPIOB_CLOCK);
}

#[no_mangle]
pub extern fn start() {
    // Initialize the LED on L432KC board  
    let gpiob = hal::gpio::Gpio::init(board::l432kc::GPIOB_BASE);

    gpiob.otype(board::l432kc::USER_LED, hal::gpio::Mode::Out, hal::gpio::OType::PushPull, hal::gpio::AltFunc::Af0);

    let mut i = 0;

    loop
    {
        while i <= 1000000 {
            if i == 500000 {
                gpiob.set_pin(board::l432kc::USER_LED_BIT);
            } else if i == 0 {
                gpiob.clr_pin(board::l432kc::USER_LED_BIT);
            }
            i += 1;
        }
        i = 0;
	}
}

#[no_mangle]
pub extern "C" fn __aeabi_unwind_cpp_pr0() {
    loop {}
}


#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}