#![no_std]

use core::panic::PanicInfo;
mod board;
mod stm32hal;
mod driver;
mod routine;
mod setup;
mod common;
// mod arm;

/* Set Clock In One Area */
const CLK:          stm32hal::common::MsiRange = stm32hal::common::MsiRange::Clk16MHz;

#[no_mangle]
pub extern fn sys_init() {
    /* RCC Enabling of the bus */
    let rcc = stm32hal::rcc::Rcc::init(board::l432kc::RCC_BASE);

    rcc.write_msi_range(CLK);
    rcc.write_ahb2_enr(board::l432kc::GPIOA_RCC_AHB2_ENABLE);
    rcc.write_ahb2_enr(board::l432kc::GPIOB_RCC_AHB2_ENABLE);
    rcc.write_apb1_enr1(board::l432kc::TIMER2_RCC_APB1R1_ENABLE);
    rcc.write_apb1_enr1(board::l432kc::USART2_RCC_APB1R1_ENABLE);
    rcc.write_apb1_enr1(board::l432kc::CAN_RCC_APB1R1_ENABLE)
}

#[no_mangle]
pub extern fn start() {
    let freq = stm32hal::common::range(CLK);
    // Initialize the LED on L432KC board
    let usart       = stm32hal::usart::Usart::init(board::l432kc::USART2_BASE);
    let can         = stm32hal::can::Can::init(board::l432kc::CAN_BASE);
    let seq_timer   = stm32hal::timer::Timer::init(board::l432kc::TIMER2_BASE);
    let mut hb      = setup::Heartbeat::init();

    setup::gpio_setup();
    
    seq_timer.open(stm32hal::timer::TimerType::Cont, stm32hal::timer::Direction::Upcount);
    seq_timer.set_scl(500, freq, 1000);
    seq_timer.start();

    usart.open(stm32hal::usart::WordLen::Bits8, stm32hal::usart::StopLen::StopBit1, stm32hal::usart::BaudRate::Baud921600, freq, stm32hal::usart::OverSample::Oversample16);
    let ci = stm32hal::can::CanInit::init();
    can.open(&ci);
    can.filter_init(0, false, false, true, 0);

    let mut wago = driver::can::wago750_337::Wago750::init(1);

    //wago.start_node(&can);

    let mut ind = 0;
    let mut i = 1;

    loop {
        while can.read_pend() {
            let msgr = can.read();
            let node = driver::can::canopen::CANOpen::get_ext_node(msgr.get_id());
            if (msgr.get_id() >> 8) as u8 == 01 {
                let analogue1 = ((msgr.get_data()[1] as u16) << 0) | ((msgr.get_data()[2] as u16) << 8);
                let analogue2 = ((msgr.get_data()[3] as u16) << 0) | ((msgr.get_data()[4] as u16) << 8);
                let anscale1 = common::scale(analogue1 as u32, 0, 32761, 0, 100);
                let anscale2 = common::scale(analogue2 as u32, 0, 32761, 0, 100);
                usart.write(&[0x44, i, node as u8, msgr.get_data()[0], (analogue1 >> 8) as u8, (analogue1 >> 0) as u8, (analogue2 >> 8) as u8, (analogue2 >> 0) as u8, anscale1 as u8, anscale2 as u8, 0x0D]);
                //usart.write(&[0x44, i, node as u8, (msgr.get_id() >> 24) as u8, (msgr.get_id() >> 16) as u8, (msgr.get_id() >> 8) as u8, (msgr.get_id() >> 0) as u8, msgr.get_data()[0], msgr.get_data()[1], msgr.get_data()[2], msgr.get_data()[3], msgr.get_data()[4], msgr.get_data()[5], msgr.get_data()[6], msgr.get_data()[7], 0x0D]);
            }
            wago.read_message(msgr);

            if i > 250 {
                i = 1;
            }
            i += 1;
        }
        
        i = 1;

        if seq_timer.get_flag() {
            if ind > 7 {
                ind = 0;
            }

            /* SET STATE SHOULD BE SET LOWER INTERNAL TO THE WAGO */
            wago.set_state(&can);
            wago.write_node_guarding(&can);
            wago.setup_wago(&can);
            if wago.setup_complete() {
                let analogue1 = ind as u16 * 4096;
                let analogue2 = ind as u16 * 2048;
                wago.write_mapped_outputs([1 << ind, (analogue1 >> 0) as u8, (analogue1 >> 8) as u8, (analogue2 >> 0) as u8, (analogue2 >> 8) as u8, 0, 0, 0], &can);
            }
            //wago.test_outputs(&can, &ind);
            //wago.test_request_inputs(&can);

            hb.heartbeat();
            
            ind += 1;
            seq_timer.clr_flag();
        }
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
