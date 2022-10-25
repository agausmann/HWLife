#![no_std]
#![no_main]

use avr_device::attiny404::Peripherals;
use core::panic::PanicInfo;

#[avr_device::entry]
fn main() -> ! {
    let dp = Peripherals::take().unwrap();
    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    loop {}
}
