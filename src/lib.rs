#![no_std]

use core::panic::PanicInfo;
mod board;
mod hal;
mod driver;
mod routine;

/* Set Clock In One Area */
const CLK:          hal::common::MsiRange = hal::common::MsiRange::Clk16MHz;

#[no_mangle]
pub extern fn sys_init() {
    /* RCC Enabling of the bus */
    let rcc = hal::rcc::Rcc::init(board::l432kc::RCC_BASE);

    rcc.write_msi_range(CLK);
    rcc.write_ahb2_enr(board::l432kc::GPIOA_RCC_AHB2_ENABLE);
    rcc.write_ahb2_enr(board::l432kc::GPIOB_RCC_AHB2_ENABLE);
    rcc.write_apb1_enr1(board::l432kc::TIMER2_RCC_APB1R1_ENABLE);
    rcc.write_apb1_enr1(board::l432kc::USART2_RCC_APB1R1_ENABLE);
    rcc.write_apb1_enr1(board::l432kc::CAN_RCC_APB1R1_ENABLE)
}

#[no_mangle]
pub extern fn start() {
    let freq = hal::common::range(CLK);
    // Initialize the LED on L432KC board
    let gpioa = hal::gpio::Gpio::init(board::l432kc::GPIOA_BASE);  
    let gpiob = hal::gpio::Gpio::init(board::l432kc::GPIOB_BASE);
    let usart = hal::usart::Usart::init(board::l432kc::USART2_BASE);
    let can = hal::can::Can::init(board::l432kc::CAN_BASE);
    let seq_timer = hal::timer::Timer::init(board::l432kc::TIMER2_BASE);

    /* USART Setup */
    gpioa.otype(board::l432kc::USART2_TX, board::l432kc::USART_MODE, board::l432kc::USART_OTYPE, board::l432kc::USART_AF);
    gpioa.otype(board::l432kc::USART2_RX, board::l432kc::USART_MODE, board::l432kc::USART_OTYPE, board::l432kc::USART_AF);

    /* CAN Setup */
    gpioa.otype(board::l432kc::CAN_TX, board::l432kc::CAN_MODE, board::l432kc::CAN_OTYPE, board::l432kc::CAN_AF);
    gpioa.otype(board::l432kc::CAN_RX, board::l432kc::CAN_MODE, board::l432kc::CAN_OTYPE, board::l432kc::CAN_AF);
    gpioa.ospeed(board::l432kc::CAN_TX, board::l432kc::CAN_OSPEED);
    gpioa.ospeed(board::l432kc::CAN_RX, board::l432kc::CAN_OSPEED);
    gpioa.pupd(board::l432kc::CAN_TX, board::l432kc::CAN_PUPD);
    gpioa.pupd(board::l432kc::CAN_RX, board::l432kc::CAN_PUPD);

    /* LED */
    gpiob.otype(board::l432kc::USER_LED, board::l432kc::USER_LED_MODE, board::l432kc::USER_LED_OTYPE, board::l432kc::USER_LED_AF);

    seq_timer.open(hal::timer::TimerType::Cont, hal::timer::Direction::Upcount);
    seq_timer.set_scl(50, freq, 1500);
    seq_timer.start();

    usart.open(hal::usart::WordLen::Bits8, hal::usart::StopLen::StopBit1, hal::usart::BaudRate::Baud9600, freq, hal::usart::OverSample::Oversample16);
    let ci = hal::can::CanInit::init();
    can.open(&ci);
    can.filter_init(0, false, false, true, 0);

    //let dogmeat = [0x44, 0x6F, 0x67, 0x6D, 0x65, 0x61, 0x74, 0x0D];

    let canopen = driver::can::canopen::CANOpen::init(0);
    let mut msg = hal::can::CanMsg::init();
    let mut msgr = hal::can::CanMsg::init();

    canopen.nmt_write_start(0, &mut msg);

    let result = can.write(&msg);

    if result { // CHECK IF WRITE IS GOOD
        usart.write(&[0x44, 0x01, 0x01, 0x0D]);
    } else {
        usart.write(&[0x44, 0x01, 0x00, 0x0D]);
    }

    let result = can.write(&msg);

    if result == true { // CHECK IF WRITE IS GOOD
        usart.write(&[0x44, 0x02, 0x01, 0x0D]);
    } else {
        usart.write(&[0x44, 0x02, 0x00, 0x0D]);
    }

    let mut i = false;
    let mut ind = 0;

    loop {
        if seq_timer.get_flag() {
            if ind > 7 {
                ind = 0;
            }

            if can.read_pend() {
                can.read(&mut msgr);
                usart.write(&[0x44, 0x06, (msgr.get_id() >> 24) as u8, (msgr.get_id() >> 16) as u8, (msgr.get_id() >> 8) as u8, (msgr.get_id() >> 0) as u8, msgr.get_data()[0], msgr.get_data()[1], msgr.get_data()[2], msgr.get_data()[3], msgr.get_data()[4], msgr.get_data()[5], msgr.get_data()[6], msgr.get_data()[7], 0x0D]);
            }

            if can.read_pend() {
                can.read(&mut msgr);
                usart.write(&[0x44, 0x07, (msgr.get_id() >> 24) as u8, (msgr.get_id() >> 16) as u8, (msgr.get_id() >> 8) as u8, (msgr.get_id() >> 0) as u8, msgr.get_data()[0], msgr.get_data()[1], msgr.get_data()[2], msgr.get_data()[3], msgr.get_data()[4], msgr.get_data()[5], msgr.get_data()[6], msgr.get_data()[7], 0x0D]);
            }

            if can.read_pend() {
                can.read(&mut msgr);
                usart.write(&[0x44, 0x08, (msgr.get_id() >> 24) as u8, (msgr.get_id() >> 16) as u8, (msgr.get_id() >> 8) as u8, (msgr.get_id() >> 0) as u8, msgr.get_data()[0], msgr.get_data()[1], msgr.get_data()[2], msgr.get_data()[3], msgr.get_data()[4], msgr.get_data()[5], msgr.get_data()[6], msgr.get_data()[7], 0x0D]);
            }

            canopen.sdo_init_download(1, driver::can::canopen::sdo::N::Bytes3, driver::can::canopen::sdo::E::Expedited, 0x6200, 0x01, [1 << ind, 0x00, 0x00, 0x00], &mut msg);
            can.write(&msg);

            canopen.sdo_init_upload(1, 0x6000, 0x01, &mut msg);
            can.write(&msg);

            if i {
                gpiob.set_pin(board::l432kc::USER_LED_BIT);
                i = false;
            } else {
                gpiob.clr_pin(board::l432kc::USER_LED_BIT);
                i = true;
            }
            
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
