//! 01_blink_led - Raspberry Pi Pico (RP2040)

#![no_std]
#![no_main]

use defmt_rtt as _;
use panic_probe as _;

use embedded_hal::delay::DelayNs;
use embedded_hal::digital::OutputPin;
use rp2040_hal as hal; // short alias
use hal::{clocks::init_clocks_and_plls, entry, pac, Sio, Timer, Watchdog};

/// Second-stage bootloader: 256 bytes copied by the RP2040 ROM from flash start. It configures the
/// external QSPI flash for XIP. `memory.x` puts section `.boot2` at 0x1000_0000.
#[link_section = ".boot2"]
#[used] // keep it even though nothing references it
pub static BOOT2: [u8; 256] = rp2040_boot2::BOOT_LOADER_W25Q080;

/// Crystal frequency on the Pico board.
const XTAL_FREQ_HZ: u32 = 12_000_000;

fn blink_forever<L: OutputPin, D: DelayNs>(led: &mut L, delay: &mut D, half_period_ms: u32) -> ! {
    loop {
        let _ = led.set_high();
        delay.delay_ms(half_period_ms);
        let _ = led.set_low();
        delay.delay_ms(half_period_ms);
    }
}

#[entry]
fn main() -> ! {
    defmt::info!("01_blink_led (RP2040) starting");

    let mut pac = pac::Peripherals::take().unwrap();
    // The watchdog block also provides a 1 us tick used by the clock/timer setup.
    let mut watchdog = Watchdog::new(pac.WATCHDOG);

    // Start the crystal oscillator, configure both PLLs (system 125 MHz, USB 48 MHz).
    let clocks = init_clocks_and_plls(
        XTAL_FREQ_HZ,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS, // peripherals are held in reset until released; HAL does it for us
        &mut watchdog,
    )
    .ok()
    .unwrap();

    // SIO = "single-cycle IO": the fast GPIO block shared by both cores.
    let sio = Sio::new(pac.SIO);
    let pins = hal::gpio::Pins::new(pac.IO_BANK0, pac.PADS_BANK0, sio.gpio_bank0, &mut pac.RESETS);

    // GPIO25 = on-board LED (original Pico). Pico W: LED is on the Wi-Fi chip; use an external LED on GP15.
    let mut led = pins.gpio25.into_push_pull_output();

    // The 64-bit microsecond hardware timer implements DelayNs.
    let mut timer = Timer::new(pac.TIMER, &mut pac.RESETS, &clocks);

    blink_forever(&mut led, &mut timer, 500)
}
