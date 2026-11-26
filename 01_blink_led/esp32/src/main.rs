//! 01_blink_led - ESP32 (classic, Xtensa) DevKit with esp-hal 1.0
//! NOTE: esp-hal evolves quickly. If this does not compile, run `esp-generate --chip esp32 blink`
//! and compare - the structure (init -> pins -> delay -> loop) stays the same.

#![no_std]
#![no_main]

use embedded_hal::{delay::DelayNs, digital::OutputPin};
use esp_backtrace as _; // panic handler: prints a backtrace on the serial console
use esp_hal::{
    delay::Delay,
    gpio::{Level, Output, OutputConfig},
    main,
};

// Embeds the "app descriptor" (name, version) that the ESP-IDF 2nd-stage bootloader expects.
esp_bootloader_esp_idf::esp_app_desc!();

fn blink_forever<L: OutputPin, D: DelayNs>(led: &mut L, delay: &mut D, half_period_ms: u32) -> ! {
    loop {
        let _ = led.set_high();
        delay.delay_ms(half_period_ms);
        let _ = led.set_low();
        delay.delay_ms(half_period_ms);
    }
}

#[main] // esp-hal's entry macro (plays the role of cortex-m-rt's #[entry])
fn main() -> ! {
    esp_println::logger::init_logger_from_env(); // enable `log::info!` over UART0
    log::info!("01_blink_led (ESP32) starting");

    // Configure clocks (default: max CPU clock) and take ownership of all peripherals.
    let peripherals = esp_hal::init(esp_hal::Config::default());

    // GPIO2 drives the blue LED on many DevKits. Output::new(pin, initial_level, config).
    let mut led = Output::new(peripherals.GPIO2, Level::Low, OutputConfig::default());

    let mut delay = Delay::new(); // busy-wait delay, implements DelayNs
    blink_forever(&mut led, &mut delay, 500)
}
