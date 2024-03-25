#![no_std]
#![no_main]

use core::panic::PanicInfo;

use ch32v00x_hal::{
    delay::CycleDelay,
    gpio::GpioExt,
    pac::{Peripherals, RCC},
    rcc::RccExt,
};
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

    // Enable and reset TIM1, TIM2, AFIO
    {
        // There is no HAL for these yet. We have to steal RCC to set it up.
        let rcc = unsafe { &*RCC::ptr() };

        rcc.apb1pcenr.modify(|_, w| w.tim2en().set_bit());
        rcc.apb1prstr.modify(|r, w| {
            // https://github.com/ch32-rs/ch32-rs/issues/20
            // w.tim2rst().set_bit()
            unsafe { w.bits(r.bits() | 0x1) }
        });
        rcc.apb1prstr.modify(|r, w| {
            // https://github.com/ch32-rs/ch32-rs/issues/20
            // w.tim2rst().clear_bit()
            unsafe { w.bits(r.bits() & !0x1) }
        });
        rcc.apb2pcenr
            .modify(|_, w| w.afioen().set_bit().tim1en().set_bit());
        rcc.apb2prstr
            .modify(|_, w| w.afiorst().set_bit().tim1rst().set_bit());
        rcc.apb2prstr
            .modify(|_, w| w.afiorst().clear_bit().tim1rst().clear_bit());
    }

    // Set up TIM2
    // 24MHz / 10kHz
    let period = 2400;
    p.TIM2.ctlr1.modify(|_, w| w.cen().clear_bit());
    p.TIM2.atrlr.write(|w| w.atrlr().variant(period));
    p.TIM2.cnt.write(|w| w.cnt().variant(period));
    p.TIM2.ctlr1.write(|w| {
        w.arpe()
            .set_bit()
            .cms()
            .variant(0b00 /* Edge-aligned */)
            .dir()
            .bit(false /* incrementing */)
            .opm()
            .clear_bit()
            .cen()
            .set_bit()
    });

    // Set up channel 3 PWM
    p.TIM2.chctlr2_output().modify(|_, w| {
        w.cc3s()
            .variant(0b00 /* output mode */)
            .oc3m()
            .variant(0b110 /* PWM 1 */)
            .oc3pe()
            .set_bit()
            .oc3fe()
            .clear_bit()
    });
    p.TIM2
        .ccer
        .modify(|_, w| w.cc3e().set_bit().cc3p().clear_bit());
    p.TIM2.ch3cvr.write(|w| w.ch3cvr().variant(100));

    p.AFIO
        .pcfr
        .modify(|_, w| w.tim2rm().variant(0b11 /* T2CH3 -> PD6 */));
    let _led = gpiod.pd6.into_alternate();

    let mut delay = CycleDelay::new(&clocks);

    for duty in (0..100).chain((0..100).rev()).cycle() {
        p.TIM2.ch3cvr.write(|w| w.ch3cvr().variant(duty * 24));
        delay.delay_ms(5);
    }
    loop {}
}
