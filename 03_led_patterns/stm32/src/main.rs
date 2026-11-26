//! 03_led_patterns - STM32F411: 4 LEDs on PB0..PB3, button on PA0 cycles the pattern.

#![no_std]
#![no_main]

use cortex_m_rt::entry;
use defmt_rtt as _;
use panic_probe as _;

use core::convert::Infallible;
use embedded_hal::delay::DelayNs;
use embedded_hal::digital::{InputPin, OutputPin};
use m02_shared::{Debouncer, Event}; // reuse the debouncer from module 02 (see Cargo.toml)
use m03_shared::{apply_mask, frame, Pattern};
use stm32f4xx_hal::{pac, prelude::*};

fn run<B: InputPin, D: DelayNs>(
    leds: &mut [&mut dyn OutputPin<Error = Infallible>],
    button: &mut B,
    delay: &mut D,
) -> ! {
    let n = leds.len() as u8;
    let mut pattern = Pattern::Chase;
    let mut debouncer = Debouncer::new();
    let (mut tick, mut ms_in_step) = (0u32, 0u32);
    loop {
        // 1 ms loop: debounce the button AND keep time without blocking for 150 ms.
        if let Some(Event::Pressed) = debouncer.update(button.is_low().unwrap_or(false)) {
            pattern = pattern.next();
            tick = 0;
            defmt::info!("next pattern");
        }
        if ms_in_step == 0 {
            apply_mask(leds, frame(pattern, tick, n));
            tick = tick.wrapping_add(1);
        }
        ms_in_step = (ms_in_step + 1) % 150; // pattern step = 150 ms
        delay.delay_ms(1);
    }
}

#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();
    let cp = cortex_m::Peripherals::take().unwrap();
    let clocks = dp.RCC.constrain().cfgr.use_hse(25.MHz()).sysclk(84.MHz()).freeze();
    let gpioa = dp.GPIOA.split();
    let gpiob = dp.GPIOB.split();

    let mut l0 = gpiob.pb0.into_push_pull_output();
    let mut l1 = gpiob.pb1.into_push_pull_output();
    let mut l2 = gpiob.pb2.into_push_pull_output();
    let mut l3 = gpiob.pb3.into_push_pull_output();
    let mut button = gpioa.pa0.into_pull_up_input();
    let mut delay = cp.SYST.delay(&clocks);

    // Four different pin TYPES coerced to one trait-object type.
    let mut leds: [&mut dyn OutputPin<Error = Infallible>; 4] = [&mut l0, &mut l1, &mut l2, &mut l3];
    run(&mut leds, &mut button, &mut delay)
}
