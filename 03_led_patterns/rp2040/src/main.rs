//! 03_led_patterns - Pico: LEDs on GP16..GP19, button on GP15.

#![no_std]
#![no_main]

use defmt_rtt as _;
use panic_probe as _;

use core::convert::Infallible;
use embedded_hal::delay::DelayNs;
use embedded_hal::digital::{InputPin, OutputPin};
use hal::{clocks::init_clocks_and_plls, entry, pac, Sio, Timer, Watchdog};
use m02_shared::{Debouncer, Event};
use m03_shared::{apply_mask, frame, Pattern};
use rp2040_hal as hal;

#[link_section = ".boot2"]
#[used]
pub static BOOT2: [u8; 256] = rp2040_boot2::BOOT_LOADER_W25Q080;

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
            defmt::info!("next pattern");
        }
        if ms_in_step == 0 {
            apply_mask(leds, frame(pattern, tick, n));
            tick = tick.wrapping_add(1);
        }
        ms_in_step = (ms_in_step + 1) % 150;
        delay.delay_ms(1);
    }
}

#[entry]
fn main() -> ! {
    let mut pac = pac::Peripherals::take().unwrap();
    let mut wd = Watchdog::new(pac.WATCHDOG);
    let clocks = init_clocks_and_plls(
        12_000_000, pac.XOSC, pac.CLOCKS, pac.PLL_SYS, pac.PLL_USB, &mut pac.RESETS, &mut wd,
    )
    .ok()
    .unwrap();
    let sio = Sio::new(pac.SIO);
    let pins = hal::gpio::Pins::new(pac.IO_BANK0, pac.PADS_BANK0, sio.gpio_bank0, &mut pac.RESETS);

    let mut l0 = pins.gpio16.into_push_pull_output();
    let mut l1 = pins.gpio17.into_push_pull_output();
    let mut l2 = pins.gpio18.into_push_pull_output();
    let mut l3 = pins.gpio19.into_push_pull_output();
    let mut button = pins.gpio15.into_pull_up_input();
    let mut timer = Timer::new(pac.TIMER, &mut pac.RESETS, &clocks);

    let mut leds: [&mut dyn OutputPin<Error = Infallible>; 4] = [&mut l0, &mut l1, &mut l2, &mut l3];
    run(&mut leds, &mut button, &mut timer)
}
