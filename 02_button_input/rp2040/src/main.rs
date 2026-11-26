//! 02_button_input - Pico: button between GP15 and GND, LED on GP25.

#![no_std]
#![no_main]

use defmt_rtt as _;
use panic_probe as _;

use embedded_hal::delay::DelayNs;
use embedded_hal::digital::{InputPin, OutputPin, StatefulOutputPin};
use hal::{clocks::init_clocks_and_plls, entry, pac, Sio, Timer, Watchdog};
use m02_shared::{Debouncer, Event};
use rp2040_hal as hal;

#[link_section = ".boot2"]
#[used]
pub static BOOT2: [u8; 256] = rp2040_boot2::BOOT_LOADER_W25Q080;
const XTAL_FREQ_HZ: u32 = 12_000_000;

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
            defmt::info!("button pressed");
        }
        delay.delay_ms(1);
    }
}

#[entry]
fn main() -> ! {
    let mut pac = pac::Peripherals::take().unwrap();
    let mut watchdog = Watchdog::new(pac.WATCHDOG);
    let clocks = init_clocks_and_plls(
        XTAL_FREQ_HZ, pac.XOSC, pac.CLOCKS, pac.PLL_SYS, pac.PLL_USB, &mut pac.RESETS, &mut watchdog,
    )
    .ok()
    .unwrap();
    let sio = Sio::new(pac.SIO);
    let pins = hal::gpio::Pins::new(pac.IO_BANK0, pac.PADS_BANK0, sio.gpio_bank0, &mut pac.RESETS);

    let mut button = pins.gpio15.into_pull_up_input();
    let mut led = pins.gpio25.into_push_pull_output();
    let mut timer = Timer::new(pac.TIMER, &mut pac.RESETS, &clocks);

    button_toggles_led(&mut button, &mut led, &mut timer)
}
