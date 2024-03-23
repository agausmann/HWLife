#![no_std]
#![no_main]

use avrxmega_hal::{clock::MHz1, delay::Delay, pac::Peripherals, prelude::*, Pins};
use core::panic::PanicInfo;

#[avrxmega_hal::entry]
fn main() -> ! {
    let dp = Peripherals::take().unwrap();
    let pins = Pins::new(dp.PORTA, dp.PORTB);
    let mut delay: Delay<MHz1> = Delay::new();

    let mut led = pins.pb2.into_output();
    loop {
        led.toggle();
        delay.delay_ms(1000_u16);
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
