#![no_std]
#![no_main]

use core::{cell::RefCell, panic::PanicInfo};

use ch32v00x_hal::{
    gpio::GpioExt,
    pac::{Peripherals, DMA1, RCC, TIM2},
    rcc::RccExt,
};
use critical_section::Mutex;
use qingke_rt::{entry, interrupt};

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
    p.TIM2.ch4cvr.write(|w| w.ch4cvr().variant(0));
    p.TIM2.chctlr2_output().modify(|_, w| {
        w.cc4s()
            .variant(0b00 /* output mode */)
            .oc4m()
            .variant(0b110 /* PWM 1 */)
            .oc4pe()
            .set_bit()
            .oc4fe()
            .clear_bit()
    });
    p.TIM2
        .ccer
        .modify(|_, w| w.cc4e().set_bit().cc4p().clear_bit());

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
    let _tx = gpiod.pd5.into_alternate();

    critical_section::with(|cs| {
        let mut ctx = DMA_CTX.borrow(cs).borrow_mut();
        *ctx = Some(DmaContext::new(p.DMA1, p.TIM2));
        ctx.as_mut().unwrap().start();
    });

    loop {
        // FIXME Seems to freeze the DMA controller
        // wfi();
    }
}

struct DmaContext {
    dma1: DMA1,
    tim2: TIM2,
}

impl DmaContext {
    fn new(dma1: DMA1, tim2: TIM2) -> Self {
        Self { dma1, tim2 }
    }

    fn poll(&mut self) {
        let intf = self.dma1.intfr.read();

        if intf.tcif2().bit_is_set() {
            self.start();
            self.dma1.intfcr.write(|w| w.ctcif2().set_bit());
        }
        self.dma1.intfcr.write(|w| w.cgif2().set_bit());
    }

    fn start(&mut self) {
        // TODO safety

        self.dma1.cfgr2.modify(|_, w| w.en().clear_bit());

        const PAYLOAD: &[u8] = b"Hello World!";

        static mut TXBUF: [u8; 162] = [0; 162];

        for (byte, buf) in PAYLOAD.iter().zip(unsafe { TXBUF[1..].chunks_mut(10) }) {
            buf[0] = 0xa0; // Start symbol
            for (i_bit, slot) in (0..8).rev().zip(&mut buf[1..]) {
                let bit = (byte & (1 << i_bit)) != 0;
                *slot = if bit { 0x60 } else { 0x20 };
            }
            buf[9] = 0xe0; // Stop symbol
        }
        let tx_len = 10 * PAYLOAD.len() + 2;
        unsafe {
            TXBUF[0] = 0;
            TXBUF[tx_len - 1] = 0;
        }

        self.dma1.cfgr2.write(|w| {
            w.msize()
                .variant(0b00 /* 8 bits */)
                .psize()
                .variant(0b10 /* 32 bits */)
                .minc()
                .set_bit()
                .dir()
                .bit(true /* From memory to peripheral */)
                .tcie()
                .set_bit()
        });
        self.dma1
            .paddr2
            .write(|w| w.pa().variant(self.tim2.ch4cvr.as_ptr() as u32));
        self.dma1
            .maddr2
            .write(|w| w.ma().variant(unsafe { TXBUF.as_ptr() } as u32));
        self.dma1.cntr2.write(|w| w.ndt().variant(tx_len as u16));
        self.dma1.cfgr2.modify(|_, w| w.en().set_bit());
    }
}

static DMA_CTX: Mutex<RefCell<Option<DmaContext>>> = Mutex::new(RefCell::new(None));

#[interrupt]
fn DMA1_CHANNEL2() {
    //FIXME is the interrupt running? why doesn't the interrupt restart the transfer?
    critical_section::with(|cs| {
        let mut ctx = DMA_CTX.borrow(cs).borrow_mut();
        if let Some(ctx) = &mut *ctx {
            ctx.poll();
        }
    });
}
