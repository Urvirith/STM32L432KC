
use crate::board;
use crate::stm32hal;

/* Set Up Area For All GPIO Move To Own Area */
pub fn gpio_setup() {
    let gpioa       = stm32hal::gpio::Gpio::init(board::l432kc::GPIOA_BASE);  
    let gpiob       = stm32hal::gpio::Gpio::init(board::l432kc::GPIOB_BASE);

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
}

/* Heartbeat Area For LED To Inform User Of Cyclic Operation Of The Processor */
pub struct Heartbeat {
    toggle: bool,
    gpio:   stm32hal::gpio::Gpio
}

impl Heartbeat {
    pub fn init() -> Heartbeat {
        return Heartbeat {
            toggle: false,
            gpio:   stm32hal::gpio::Gpio::init(board::l432kc::GPIOB_BASE)
        }
    }

    pub fn heartbeat(&mut self) {
        if self.toggle == true {
            self.gpio.set_pin(board::l432kc::USER_LED_BIT);
            self.toggle = false;
        } else {
            self.gpio.clr_pin(board::l432kc::USER_LED_BIT);
            self.toggle = true;
        }
    }
}