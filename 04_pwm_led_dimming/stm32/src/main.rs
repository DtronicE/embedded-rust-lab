//! 04_pwm_led_dimming - STM32F411: TIM3 channel 1 on PA6 -> LED (breathing).
//! NOTE: stm32f4xx-hal's PWM API changes between releases. If a line fails to compile, check
//! https://docs.rs/stm32f4xx-hal (module `timer::pwm`) and the crate's `examples/pwm.rs`.

#![no_std]
#![no_main]

use cortex_m_rt::entry;
use defmt_rtt as _;
use panic_probe as _;

use embedded_hal::delay::DelayNs;
use embedded_hal::pwm::SetDutyCycle; // max_duty_cycle() / set_duty_cycle()
use m04_shared::{gamma2, triangle};
use stm32f4xx_hal::{pac, prelude::*, timer::Channel1};

/// Breathe forever. Works with ANY PWM channel on ANY chip.
fn breathe<P: SetDutyCycle, D: DelayNs>(pwm: &mut P, delay: &mut D) -> ! {
    let max = pwm.max_duty_cycle() as u32; // timer's ARR+1 etc. - the hardware decides
    const STEPS: u32 = 256; // steps per half-cycle -> 512 steps * 4 ms = ~2 s per breath
    let mut step = 0u32;
    loop {
        let linear = triangle(step, STEPS, max);
        let duty = gamma2(linear, max);
        let _ = pwm.set_duty_cycle(duty as u16);
        step = step.wrapping_add(1);
        delay.delay_ms(4);
    }
}

#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();
    let cp = cortex_m::Peripherals::take().unwrap();
    let clocks = dp.RCC.constrain().cfgr.use_hse(25.MHz()).sysclk(84.MHz()).freeze();
    let gpioa = dp.GPIOA.split();

    // PA6 = TIM3_CH1 (alternate function 2). Channel1::new() claims the pin for that function.
    let channels = Channel1::new(gpioa.pa6);
    // TIM3 as PWM at 1 kHz. split() gives one PwmChannel implementing eh1 SetDutyCycle.
    let mut pwm = dp.TIM3.pwm_hz(channels, 1.kHz(), &clocks).split();
    pwm.enable();

    let mut delay = cp.SYST.delay(&clocks);
    breathe(&mut pwm, &mut delay)
}
