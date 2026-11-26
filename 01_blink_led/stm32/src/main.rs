//! 01_blink_led - STM32F411 "Black Pill"
//!
//! The *portable* part is `blink_forever` (generic over embedded-hal traits).
//! The *chip-specific* part is `main` (clock tree + pin setup).

#![no_std] // no standard library: only `core`
#![no_main] // no `fn main()` called by an OS: cortex-m-rt provides the reset entry

use cortex_m_rt::entry; // macro that marks our entry point and wires it to the Reset vector
use defmt_rtt as _; // links the RTT transport so defmt logs can reach the host
use panic_probe as _; // defines #[panic_handler]: logs the panic and halts

use embedded_hal::delay::DelayNs; // trait: delay_ms / delay_us / delay_ns
use embedded_hal::digital::OutputPin; // trait: set_high / set_low
use stm32f4xx_hal::{pac, prelude::*}; // PAC = raw registers; prelude = extension traits (.constrain(), .split(), .MHz() ...)

/// Blink any pin `L` forever using any delay provider `D`.
/// `-> !` ("never type") tells the compiler this function never returns.
fn blink_forever<L: OutputPin, D: DelayNs>(led: &mut L, delay: &mut D, half_period_ms: u32) -> ! {
    loop {
        let _ = led.set_high(); // Result is Err only if the pin can fail; GPIO is Infallible
        delay.delay_ms(half_period_ms);
        let _ = led.set_low();
        delay.delay_ms(half_period_ms);
    }
}

#[entry]
fn main() -> ! {
    defmt::info!("01_blink_led (STM32F411) starting");

    // `take()` returns Some only the FIRST time -> guarantees a single owner of every peripheral.
    let dp = pac::Peripherals::take().unwrap(); // device peripherals (GPIO, RCC, TIM, ...)
    let cp = cortex_m::Peripherals::take().unwrap(); // core peripherals (SysTick, NVIC, SCB, ...)

    // RCC = Reset & Clock Control. `constrain()` turns the raw register block into a typed builder.
    let rcc = dp.RCC.constrain();
    // Black Pill has a 25 MHz crystal. Use it (HSE), PLL up to 84 MHz, then `freeze()` programs the
    // hardware and returns `Clocks` - a proof-token that clocks are configured. Peripherals that
    // need clock info (delay, UART baud...) take `&clocks`.
    let clocks = rcc.cfgr.use_hse(25.MHz()).sysclk(84.MHz()).freeze();

    // `split()` consumes GPIOC and hands out one typed object per pin.
    let gpioc = dp.GPIOC.split();
    // PC13 is wired to the on-board LED (active LOW). `into_push_pull_output` changes the type state.
    let mut led = gpioc.pc13.into_push_pull_output();

    // SysTick as a blocking delay. It implements embedded_hal::delay::DelayNs.
    let mut delay = cp.SYST.delay(&clocks);

    blink_forever(&mut led, &mut delay, 500)
}
