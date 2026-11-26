//! 02_button_input - ESP32 DevKit: button between GPIO4 and GND, LED on GPIO2.

#![no_std]
#![no_main]

use embedded_hal::{
    delay::DelayNs,
    digital::{InputPin, OutputPin, StatefulOutputPin},
};
use esp_backtrace as _;
use esp_hal::{
    delay::Delay,
    gpio::{Input, InputConfig, Level, Output, OutputConfig, Pull},
    main,
};
use m02_shared::{Debouncer, Event};

esp_bootloader_esp_idf::esp_app_desc!();

fn button_toggles_led<B, L, D>(button: &mut B, led: &mut L, delay: &mut D) -> !
where
    B: InputPin,
    L: OutputPin + StatefulOutputPin,
    D: DelayNs,
{
    let mut debouncer = Debouncer::new();
    loop {
        let pressed = button.is_low().unwrap_or(false);
        if let Some(Event::Pressed) = debouncer.update(pressed) {
            let _ = led.toggle();
            log::info!("button pressed");
        }
        delay.delay_ms(1);
    }
}

#[main]
fn main() -> ! {
    esp_println::logger::init_logger_from_env();
    let peripherals = esp_hal::init(esp_hal::Config::default());

    // GPIO4 as input with internal pull-up.
    let mut button = Input::new(peripherals.GPIO4, InputConfig::default().with_pull(Pull::Up));
    let mut led = Output::new(peripherals.GPIO2, Level::Low, OutputConfig::default());
    let mut delay = Delay::new();

    button_toggles_led(&mut button, &mut led, &mut delay)
}
