#![no_std]
#![no_main]

use avrxmega_hal::{
    pac::Peripherals,
    port::{
        mode::{Input, PullUp},
        Pin,
    },
    Pins,
};
use core::panic::PanicInfo;

const EEPROM_START: usize = 0x1f00;
const EEPROM_SIZE: usize = 128;

#[avrxmega_hal::entry]
fn main() -> ! {
    let dp = Peripherals::take().unwrap();
    let pins = Pins::new(dp.PORTA, dp.PORTB);

    let eeprom: &[u8; EEPROM_SIZE] = unsafe { &*(EEPROM_START as *const [u8; EEPROM_SIZE]) };

    // Order matters for the index calculation -
    // Starting top-left and going around clockwise:
    let neighbors: [Pin<Input<PullUp>>; 8] = [
        pins.pa7.into_pull_up_input().downgrade(), // DIA1
        pins.pa6.into_pull_up_input().downgrade(), // ADJ1
        pins.pa4.into_pull_up_input().downgrade(), // DIA2
        pins.pa5.into_pull_up_input().downgrade(), // ADJ2
        pins.pb0.into_pull_up_input().downgrade(), // DIA3
        pins.pb1.into_pull_up_input().downgrade(), // ADJ3
        pins.pb2.into_pull_up_input().downgrade(), // DIA4
        pins.pb3.into_pull_up_input().downgrade(), // ADJ4
    ];
    let clk = pins.pa1.into_pull_up_input();
    let mut state = pins.pa2.into_opendrain_high();
    let mut alive = false;
    loop {
        //TODO interrupt-based sleep/wake
        while clk.is_high() {}

        let mut alive_neighbors: u8 = 0;
        for neighbor in &neighbors {
            alive_neighbors <<= 1;
            alive_neighbors |= neighbor.is_low() as u8;
        }

        let index = usize::from_le_bytes([alive_neighbors, alive as u8]);

        while clk.is_low() {}

        let byte = index / 8;
        let bit = index % 8;
        alive = (eeprom[byte] & (1 << bit)) != 0;

        if alive {
            state.set_low();
        } else {
            state.set_high();
        }
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
