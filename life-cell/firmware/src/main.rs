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

#[avrxmega_hal::entry]
fn main() -> ! {
    let dp = Peripherals::take().unwrap();
    let pins = Pins::new(dp.PORTA, dp.PORTB);

    let neighbors: [Pin<Input<PullUp>>; 8] = [
        pins.pa4.into_pull_up_input().downgrade(),
        pins.pa5.into_pull_up_input().downgrade(),
        pins.pa6.into_pull_up_input().downgrade(),
        pins.pa7.into_pull_up_input().downgrade(),
        pins.pb0.into_pull_up_input().downgrade(),
        pins.pb1.into_pull_up_input().downgrade(),
        pins.pb2.into_pull_up_input().downgrade(),
        pins.pb3.into_pull_up_input().downgrade(),
    ];
    let clk = pins.pa1.into_pull_up_input();
    let mut state = pins.pa2.into_opendrain_high();
    let mut alive = false;
    loop {
        //TODO interrupt-based sleep/wake
        while clk.is_high() {}

        let alive_neighbors: u8 = neighbors.iter().map(|pin| pin.is_low() as u8).sum();

        while clk.is_low() {}

        if alive {
            alive = alive_neighbors == 2 || alive_neighbors == 3;
        } else {
            alive = alive_neighbors == 3;
        }
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
