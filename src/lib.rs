#![no_std]

use core::panic::PanicInfo;
mod board;
mod stm32hal;
// mod driver;
// mod routine;
// mod setup;
mod common;
// mod arm;

/* Set Clock In One Area */
const CLK:          stm32hal::common::MsiRange = stm32hal::common::MsiRange::Clk16MHz;

#[no_mangle]
pub extern "C" fn sys_init() {
    /* RCC Enabling of the bus */
    let rcc = stm32hal::rcc::Rcc::init(board::l432kc::RCC_BASE);

    rcc.write_msi_range(CLK);
    rcc.write_ahb2_enr(board::l432kc::GPIOA_RCC_AHB2_ENABLE);
    rcc.write_ahb2_enr(board::l432kc::GPIOB_RCC_AHB2_ENABLE);
    rcc.write_apb1_enr1(board::l432kc::TIMER2_RCC_APB1R1_ENABLE);
    rcc.write_apb1_enr1(board::l432kc::TIMER7_RCC_APB1R1_ENABLE);
}

#[no_mangle]
pub extern "C" fn start() {
    let freq = stm32hal::common::range(CLK);
    // Initialize the LED on L432KC board
    let gpioa       = stm32hal::gpio::Gpio::init(board::l432kc::GPIOA_BASE);
    let gpiob       = stm32hal::gpio::Gpio::init(board::l432kc::GPIOB_BASE);
    let seq_timer   = stm32hal::timer::Timer::init(board::l432kc::TIMER2_BASE);
    let int_timer   = stm32hal::timer::Timer::init(board::l432kc::TIMER7_BASE);
    let mut nvic    = stm32hal::nvic::Nvic::init(board::l432kc::NVIC_BASE);

    gpiob.otype(board::l432kc::USER_LED, board::l432kc::LED_MODE, board::l432kc::LED_OTYPE, board::l432kc::LED_AF);
    gpioa.otype(board::l432kc::LED1, board::l432kc::LED_MODE, board::l432kc::LED_OTYPE, board::l432kc::LED_AF);
    gpioa.otype(board::l432kc::LED2, board::l432kc::LED_MODE, board::l432kc::LED_OTYPE, board::l432kc::LED_AF);
    gpioa.otype(board::l432kc::LED3, board::l432kc::LED_MODE, board::l432kc::LED_OTYPE, board::l432kc::LED_AF);

    seq_timer.open(stm32hal::timer::TimerType::Cont, stm32hal::timer::Direction::Upcount);
    seq_timer.set_scl(2000, freq, 1000);

    int_timer.open(stm32hal::timer::TimerType::Ons, stm32hal::timer::Direction::Upcount);
    int_timer.set_interrupt();
    int_timer.delay(1000, freq, 500);

    nvic.set_interrupt(board::l432kc::NvicIrq::TIM7_IRQ as u32);
    int_timer.set_scl(1000, freq, 500);
    seq_timer.start();
    int_timer.start();

    let mut i = 1;

    loop {
        if seq_timer.get_flag() {
            seq_timer.clr_flag();

            match i {
                0 => {
                    gpioa.set_pin(board::l432kc::LED1_BIT);
                    gpioa.clr_pin(board::l432kc::LED2_BIT);
                    gpioa.clr_pin(board::l432kc::LED3_BIT);
                } 1 => {
                    gpioa.clr_pin(board::l432kc::LED1_BIT);
                    gpioa.set_pin(board::l432kc::LED2_BIT);
                    gpioa.clr_pin(board::l432kc::LED3_BIT);
                } _ => {
                    gpioa.clr_pin(board::l432kc::LED1_BIT);
                    gpioa.clr_pin(board::l432kc::LED2_BIT);
                    gpioa.set_pin(board::l432kc::LED3_BIT);
                }
            }

            if i < 3 {
                i += 1;
            } else {
                i = 0;
            }
        }
	}
}


#[no_mangle]
pub extern "C" fn TIM7_IRQHandler() {
    let gpiob       = stm32hal::gpio::Gpio::init(board::l432kc::GPIOB_BASE);
    let int_timer   = stm32hal::timer::Timer::init(board::l432kc::TIMER7_BASE);
    int_timer.clr_flag();

    if gpiob.get_pin(board::l432kc::USER_LED_BIT) {
        gpiob.clr_pin(board::l432kc::USER_LED_BIT);
    } else {
        gpiob.set_pin(board::l432kc::USER_LED_BIT);
    }

    int_timer.start();
}







#[no_mangle]
pub extern "C" fn __aeabi_unwind_cpp_pr0() {
    loop {}
}


#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
