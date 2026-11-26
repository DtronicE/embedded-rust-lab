//! 04_pwm_led_dimming - ESP32: LEDC peripheral (LED Controller) on GPIO2.
//! LEDC's own API (not embedded-hal's SetDutyCycle) is used here: the portable `shared` maths is reused.

#![no_std]
#![no_main]

use embedded_hal::delay::DelayNs;
use esp_backtrace as _;
use esp_hal::{
    delay::Delay,
    gpio::{Level, Output, OutputConfig},
    ledc::{
        channel::{self, ChannelIFace},
        timer::{self, TimerIFace},
        LSGlobalClkSource, Ledc, LowSpeed,
    },
    main,
    time::Rate,
};
use m04_shared::{gamma2, triangle};

esp_bootloader_esp_idf::esp_app_desc!();

#[main]
fn main() -> ! {
    esp_println::logger::init_logger_from_env();
    let p = esp_hal::init(esp_hal::Config::default());
    let led = Output::new(p.GPIO2, Level::Low, OutputConfig::default());

    // LEDC = 8 low-speed channels fed by 4 timers; any channel can go to any GPIO.
    let mut ledc = Ledc::new(p.LEDC);
    ledc.set_global_slow_clock(LSGlobalClkSource::APBClk);

    // Timer: 8-bit resolution (0..=255 duty), 5 kHz carrier.
    let mut lstimer0 = ledc.timer::<LowSpeed>(timer::Number::Timer0);
    lstimer0
        .configure(timer::config::Config {
            duty: timer::config::Duty::Duty8Bit,
            clock_source: timer::LSClockSource::APBClk,
            frequency: Rate::from_khz(5),
        })
        .unwrap();

    let mut channel0 = ledc.channel(channel::Number::Channel0, led);
    channel0
        .configure(channel::config::Config {
            timer: &lstimer0,
            duty_pct: 0,
            pin_config: channel::config::PinConfig::PushPull,
        })
        .unwrap();

    let delay = Delay::new();
    let max = 255u32;
    const STEPS: u32 = 255;
    let mut step = 0u32;
    loop {
        let duty = gamma2(triangle(step, STEPS, max), max);
        let pct = (duty * 100 / max) as u8; // LEDC API takes a percentage here
        let _ = channel0.set_duty(pct);
        step = step.wrapping_add(1);
        delay.delay_millis(4);
    }
}
