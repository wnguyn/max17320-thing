#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_stm32::{
    gpio::Speed,
    i2c::{Config as I2cConfig, I2c},
    rcc::{AHBPrescaler, APBPrescaler, Pll, PllDiv, PllMul, PllPreDiv, PllSource, Sysclk, VoltageScale},
    time::Hertz,
    usart::{Config as UartConfig, Uart},
};
use firmblop::drivers::stm32h735::max17320::BMS;
use panic_halt as _;

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let mut config = embassy_stm32::Config::default();
    config.rcc.pll1 = Some(Pll {
        source: PllSource::HSI,
        prediv: PllPreDiv::DIV4,
        mul: PllMul::MUL25,
        divp: Some(PllDiv::DIV2),
        divq: None,
        divr: Some(PllDiv::DIV4),
    });
    config.rcc.sys = Sysclk::PLL1_P;
    config.rcc.ahb_pre = AHBPrescaler::DIV2;
    config.rcc.apb1_pre = APBPrescaler::DIV2;
    config.rcc.apb2_pre = APBPrescaler::DIV2;
    config.rcc.apb3_pre = APBPrescaler::DIV2;
    config.rcc.apb4_pre = APBPrescaler::DIV2;
    config.rcc.voltage_scale = VoltageScale::Scale1;
    let p = embassy_stm32::init(config);

    let mut i2c_config = I2cConfig::default();
    i2c_config.frequency = Hertz(100_000);
    i2c_config.gpio_speed = Speed::Low;
    let i2c = I2c::new_blocking(p.I2C1, p.PB8, p.PB9, i2c_config);

    let mut uart_config = UartConfig::default();
    uart_config.baudrate = 115_200;
    let serial = Uart::new_blocking(p.USART3, p.PC11, p.PC10, uart_config).unwrap();
    let (_tx, _rx) = serial.split();
    let _bms = BMS::new(i2c);

    loop {}
}
