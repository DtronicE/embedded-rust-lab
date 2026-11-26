//! 03_led_patterns - ESP32: LEDs on GPIO16..19, button on GPIO4.

#![no_std]
#![no_main]

use core::convert::Infallible;
use embedded_hal::{
    delay::DelayNs,
    digital::{InputPin, OutputPin},
};
use esp_backtrace as _;
use esp_hal::{
    delay::Delay,
    gpio::{Input, InputConfig, Level, Output, OutputConfig, Pull},
    main,
};
use m02_shared::{Debouncer, Event};
use m03_shared::{apply_mask, frame, Pattern};

esp_bootloader_esp_idf::esp_app_desc!();

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
        if let Some(Event::Pressed) = debouncer.update(button.is_low().unwrap_or(false)) {
            pattern = pattern.next();
            tick = 0;
            log::info!("next pattern");
        }
        if ms_in_step == 0 {
            apply_mask(leds, frame(pattern, tick, n));
            tick = tick.wrapping_add(1);
        }
        ms_in_step = (ms_in_step + 1) % 150;
        delay.delay_ms(1);
    }
}

#[main]
fn main() -> ! {
    esp_println::logger::init_logger_from_env();
    let p = esp_hal::init(esp_hal::Config::default());
    let cfg = OutputConfig::default();
    let mut l0 = Output::new(p.GPIO16, Level::Low, cfg);
    let mut l1 = Output::new(p.GPIO17, Level::Low, OutputConfig::default());
    let mut l2 = Output::new(p.GPIO18, Level::Low, OutputConfig::default());
    let mut l3 = Output::new(p.GPIO19, Level::Low, OutputConfig::default());
    let mut button = Input::new(p.GPIO4, InputConfig::default().with_pull(Pull::Up));
    let mut delay = Delay::new();

    let mut leds: [&mut dyn OutputPin<Error = Infallible>; 4] = [&mut l0, &mut l1, &mut l2, &mut l3];
    run(&mut leds, &mut button, &mut delay)
}
