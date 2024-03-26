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

const PAYLOAD: &[u8] = &[32, 96, 96, 32, 96, 32, 96, 32];

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

    // Enable and reset TIM1, TIM2, AFIO, DMA
    {
        // There is no HAL for these yet. We have to steal RCC to set it up.
        let rcc = unsafe { &*RCC::ptr() };

        rcc.ahbpcenr.modify(|_, w| w.dma1en().set_bit());

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
    let period = 256;
    p.TIM2.ctlr1.modify(|_, w| w.cen().clear_bit());

    // Set up PWM DMA (TIM2_UP, DMA1_CH2)
    p.TIM2.dmaintenr.modify(|_, w| w.ude().set_bit());
    p.TIM2.chctlr2_output().modify(|_, w| {
        w.cc4s()
            .variant(0b00 /* output mode */)
            .oc4m()
            .variant(0b110 /* PWM 1 */)
            .oc4pe()
            .clear_bit()
            .oc4fe()
            .clear_bit()
    });
    p.TIM2
        .ccer
        .modify(|_, w| w.cc4e().set_bit().cc4p().clear_bit());

    p.DMA1.cfgr2.write(|w| {
        w.msize()
            .variant(0b00 /* 8 bits */)
            .psize()
            .variant(0b10 /* 32 bits */)
            .minc()
            .set_bit()
            .circ()
            .set_bit()
            .dir()
            .bit(true /* From memory to peripheral */)
    });
    p.DMA1
        .paddr2
        .write(|w| w.pa().variant(p.TIM2.ch4cvr.as_ptr() as u32));
    p.DMA1
        .maddr2
        .write(|w| w.ma().variant(PAYLOAD.as_ptr() as u32));
    p.DMA1
        .cntr2
        .write(|w| w.ndt().variant(PAYLOAD.len() as u16));
    p.DMA1.cfgr2.modify(|_, w| w.en().set_bit());

    // Finalize TIM2 init and start
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
        .modify(|_, w| w.tim2rm().variant(0b11 /* CH3/PD6, CH4/PD5 */));
    let _led = gpiod.pd6.into_alternate();
    let _dma = gpiod.pd5.into_alternate();

    let mut delay = CycleDelay::new(&clocks);

    for duty in (0..=period).chain((0..=period).rev()).cycle() {
        p.TIM2.ch3cvr.write(|w| w.ch3cvr().variant(duty));
        delay.delay_ms(5);
    }
    loop {}
}
