#![no_std]
#![no_main]
use cortex_m::Peripherals;
use cortex_m_rt::entry;
use panic_halt as _;
use stm32h7xx_hal::*;

#[entry]
fn main() -> ! {
    // It's fine unwrapping because it only returns Option<T>
    // iff you try to call take() again
    let perp = Peripherals::take().unwrap();
    loop {
        // do nothing
    }
}
