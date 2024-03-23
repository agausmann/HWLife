#![no_std]
#![no_main]

use core::panic::PanicInfo;

use ch32v00x_hal::{delay::CycleDelay, gpio::GpioExt, pac::Peripherals, rcc::RccExt};
use embedded_hal::delay::DelayNs;
use qingke_rt::entry;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[entry]
fn main() -> ! {
    let p = unsafe { Peripherals::steal() };

    let mut rcc = p.RCC.constrain();
    let clocks = rcc.config.freeze();

    let gpiod = p.GPIOD.split(&mut rcc);

    let mut led = gpiod.pd6.into_push_pull_output();
    let mut delay = CycleDelay::new(&clocks);

    loop {
        led.toggle();
        delay.delay_ms(500);
    }
}
