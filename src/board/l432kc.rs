use crate::hal::common;

/* Register Base */
/* Reset and Clock Control (RCC) */
pub const RCC_BASE:                 u32 = 0x40021000;

/* General Purpose I/O */
pub const GPIOA_BASE:               u32 = 0x48000000;  
pub const GPIOB_BASE:               u32 = 0x48000400; 
pub const GPIOC_BASE:               u32 = 0x48000800;

/* Timers */
pub const TIMER1_BASE:              u32 = 0x40012C00;
pub const TIMER2_BASE:              u32 = 0x40000000;
pub const TIMER3_BASE:              u32 = 0x40000400;
pub const TIMER6_BASE:              u32 = 0x40001000;
pub const TIMER7_BASE:              u32 = 0x40001400;
pub const TIMER15_BASE:             u32 = 0x40014000;
pub const TIMER16_BASE:             u32 = 0x40014400;

/* USART (Universal Synchronous and Asynchronous Receiver Transmitter) */
pub const USART1_BASE:              u32 = 0x40013800;
pub const USART2_BASE:              u32 = 0x40004400;      
pub const USART3_BASE:              u32 = 0x40004800;

/* Inter-Integrated Circuit (I2C) */
pub const I2C1_BASE:                u32 = 0x40005400; 
//pub const I2C2_BASE:                u32 = 0x40005800; Does not exist in the 432KC
pub const I2C3_BASE:                u32 = 0x40005C00;

/* Serial Peripheral Interface */
pub const SPI1_BASE:                u32 = 0x40013000; 
//pub const SPI2_BASE:                u32 = 0x40003800; Does not exist in the 432KC
pub const SPI3_BASE:                u32 = 0x40003C00;

/* Serial Peripheral Interface */
pub const CAN_BASE:                 u32 = 0x40006400; 
      
/* Reset and Clock Control (RCC) */
pub const GPIOA_RCC_AHB2_ENABLE:    u32 = common::BIT_0;
pub const GPIOB_RCC_AHB2_ENABLE:    u32 = common::BIT_1;

/* General Purpose I/O */
pub const USER_LED:                 u32 = 3;
pub const USER_LED_BIT:             u32 = common::BIT_3;

/* Timer */
pub const TIMER2_RCC_APB1R1_ENABLE: u32 = common::BIT_0;

/* USART (Universal Synchronous and Asynchronous Receiver Transmitter) */
pub const USART2_RCC_APB1R1_ENABLE: u32 = common::BIT_17;
pub const PORTA_PIN2:               u32 = 2;    //A7    TX
pub const PORTA_PIN3:               u32 = 3;    //A2    RX
pub const USART2_TX:                u32 = PORTA_PIN2;
pub const USART2_RX:                u32 = PORTA_PIN3;

/* I2C 1*/
pub const I2C1_RCC_APB1R1_ENABLE:   u32 = common::BIT_21;
pub const PORTB_PIN6:               u32 = 6;    //D5    SCL
pub const PORTB_PIN7:               u32 = 7;    //D4    SDA
pub const I2C1_SCL:                 u32 = PORTB_PIN6;
pub const I2C1_SDA:                 u32 = PORTB_PIN7;

/* CAN */
pub const CAN_RCC_APB1R1_ENABLE:    u32 = common::BIT_25;
pub const PORTA_PIN11:              u32 = 11;   //D10   RX
pub const PORTA_PIN12:              u32 = 12;   //D2    TX
pub const CAN_RX:                   u32 = PORTA_PIN11;
pub const CAN_TX:                   u32 = PORTA_PIN12;

/* SPI 1*/
/* RCC */
pub const SPI1_RCC_APB2R_ENABLE:    u32 = common::BIT_12;

/* SPI 2*/
/* RCC */
// pub const SPI2_RCC_APB1R1_ENABLE:   u32 = common::BIT_14; // NOT AVAILABLE 432KC

/* SPI 3*/
/* RCC */
pub const SPI3_RCC_APB1R1_ENABLE:   u32 = common::BIT_15;




