//! 02_button_input - STM32F411 Black Pill: onboard KEY (PA0, to GND when pressed) toggles LED PC13.

#![no_std]
#![no_main]

use cortex_m_rt::entry;
use defmt_rtt as _;
use panic_probe as _;

use embedded_hal::delay::DelayNs;
use embedded_hal::digital::{InputPin, OutputPin, StatefulOutputPin};
use m02_shared::{Debouncer, Event}; // our hardware-independent logic
use stm32f4xx_hal::{pac, prelude::*};

/// Poll `button` every 1 ms, toggle `led` on each debounced press. Portable across all chips.
/// `StatefulOutputPin` adds `toggle()` (needs to know the current output state).
fn button_toggles_led<B, L, D>(button: &mut B, led: &mut L, delay: &mut D) -> !
where
    B: InputPin,
    L: OutputPin + StatefulOutputPin,
    D: DelayNs,
{
    let mut debouncer = Debouncer::new();
    loop {
        // Button wired to GND with pull-up => LOW means pressed (active low).
        let pressed = button.is_low().unwrap_or(false);
        if let Some(Event::Pressed) = debouncer.update(pressed) {
            let _ = led.toggle();
            defmt::info!("button pressed");
        }
        delay.delay_ms(1); // fixed 1 ms sampling period => THRESHOLD = 20 ms debounce time
    }
}

#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();
    let cp = cortex_m::Peripherals::take().unwrap();
    let rcc = dp.RCC.constrain();
    let clocks = rcc.cfgr.use_hse(25.MHz()).sysclk(84.MHz()).freeze();

    let gpioa = dp.GPIOA.split();
    let gpioc = dp.GPIOC.split();

    // Input with the MCU's internal pull-up resistor enabled (~40 kΩ): no external resistor needed.
    let mut button = gpioa.pa0.into_pull_up_input();
    let mut led = gpioc.pc13.into_push_pull_output();
    let mut delay = cp.SYST.delay(&clocks);

    button_toggles_led(&mut button, &mut led, &mut delay)
}
