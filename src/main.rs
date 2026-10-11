#![no_std]
#![no_main]

use cortex_m_rt::entry;
use firmblop::drivers::stm32h735::max17320::BMS;
use panic_halt as _;
use stm32h7xx_hal::{pac, prelude::*};

#[entry]
fn main() -> ! {
    // init I2C on the stm32h735
    let dp = pac::Peripherals::take().unwrap();

    let pwr = dp.PWR.constrain();
    let pwrcfg = pwr.freeze();
    let rcc = dp.RCC.constrain();

    let ccdr = rcc.sys_ck(200.MHz()).freeze(pwrcfg, &dp.SYSCFG);

    let gpiob = dp.GPIOB.split(ccdr.peripheral.GPIOB);
    let gpioc = dp.GPIOC.split(ccdr.peripheral.GPIOC);
    // pins are temporary FIND IT ONCE CONTROL BOARD IS MADE!
    let scl = gpiob.pb8.into_alternate_open_drain::<4>();
    let sda = gpiob.pb9.into_alternate_open_drain::<4>();

    let i2c = dp
        .I2C1
        .i2c((scl, sda), 100.kHz(), ccdr.peripheral.I2C1, &ccdr.clocks);
    let tx = gpioc.pc10.into_alternate();
    let rx = gpioc.pc11.into_alternate();
    // init UART on the stm32h735
    let serial = dp
        .USART3
        .serial(
            (tx, rx),
            115_200.bps(),
            ccdr.peripheral.USART3,
            &ccdr.clocks,
        )
        .unwrap();

    let (mut tx, mut rx) = serial.split();

    let mut bms = BMS::new(i2c);

    loop {
        // SIX SEVEN!
    }
}
