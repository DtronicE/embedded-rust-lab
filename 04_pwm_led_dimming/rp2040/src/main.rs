//! 04_pwm_led_dimming - Pico: PWM slice 4 channel B drives GP25 (on-board LED) - no wiring needed.

#![no_std]
#![no_main]

use defmt_rtt as _;
use panic_probe as _;

use embedded_hal::delay::DelayNs;
use embedded_hal::pwm::SetDutyCycle;
use hal::{clocks::init_clocks_and_plls, entry, pac, Sio, Timer, Watchdog};
use m04_shared::{gamma2, triangle};
use rp2040_hal as hal;

#[link_section = ".boot2"]
#[used]
pub static BOOT2: [u8; 256] = rp2040_boot2::BOOT_LOADER_W25Q080;

fn breathe<P: SetDutyCycle, D: DelayNs>(pwm: &mut P, delay: &mut D) -> ! {
    let max = pwm.max_duty_cycle() as u32;
    const STEPS: u32 = 256;
    let mut step = 0u32;
    loop {
        let duty = gamma2(triangle(step, STEPS, max), max);
        let _ = pwm.set_duty_cycle(duty as u16);
        step = step.wrapping_add(1);
        delay.delay_ms(4);
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

    // RP2040 has 8 PWM slices x 2 channels (A/B). Each GPIO maps to a fixed slice/channel:
    // slice = (gpio / 2) % 8 ; channel = gpio % 2 (even = A, odd = B). GPIO25 -> slice 4, B.
    let mut slices = hal::pwm::Slices::new(pac.PWM, &mut pac.RESETS);
    let pwm = &mut slices.pwm4;
    pwm.set_ph_correct(); // phase-correct counting: smoother, half the frequency
    pwm.enable();
    let channel = &mut pwm.channel_b;
    channel.output_to(pins.gpio25); // route the PWM output to the pad

    let mut timer = Timer::new(pac.TIMER, &mut pac.RESETS, &clocks);
    breathe(channel, &mut timer)
}
