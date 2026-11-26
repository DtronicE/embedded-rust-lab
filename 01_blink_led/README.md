# 01 - Blink LED (deep dive)

You already blinked an LED. This module re-does it **the way the rest of the course will work**:
portable logic written against `embedded-hal` traits, with all chip-specific code isolated in `main`.
Use it to understand every moving part of a `no_std` project.

## Learning objectives
After this module you can:
1. Explain `#![no_std]`, `#![no_main]`, `#[entry]` and why `main` returns `!`.
2. Describe the Rust embedded stack: PAC -> HAL -> embedded-hal -> your code.
3. Explain `Peripherals::take()`, clock configuration and pin type states.
4. Build, flash and log with `cargo run`, `cargo embed` and `probe-rs`.
5. Read `memory.x`, `build.rs` and `.cargo/config.toml` and say what each does.

Read first: [`docs/01`](../docs/01_cargo_for_embedded.md), [`docs/02`](../docs/02_memory_x_linker_startup.md), [`docs/03`](../docs/03_embedded_hal.md).

## Concept
A GPIO pin is a register bit connected to a pad. "Output push-pull" means the pad actively drives both
high and low. An LED needs a **current-limiting resistor**: `R = (Vcc - Vf) / I`, e.g. `(3.3 - 2.0) / 0.005 = 260 Ω` -> use 330 Ω.

## Wiring
Use the **on-board LED** (no wiring) or an external one:

```
 MCU pin ──[ 330 Ω ]──►|──┐         (LED anode on the resistor side)
                         │
 GND ────────────────────┘

 STM32 Black Pill : PC13 on-board LED (ACTIVE-LOW: pin low = LED on) or PA1 + external
 Pico (RP2040)    : GP25 on-board LED (Pico W: use GP15 + external LED)
 ESP32 DevKit     : GPIO2 on-board LED on most boards, or GPIO2 + external
```
Debug probe (STM32):
```
 ST-Link  SWDIO ── PA13 (SWDIO)      Pico probe  GP2 (SWCLK) ── Target SWCLK
 ST-Link  SWCLK ── PA14 (SWCLK)                  GP3 (SWDIO) ── Target SWDIO
 ST-Link  GND ──── GND                           GND ─────────── GND
 ST-Link  3V3 ──── 3V3
```

## Folder structure
```
01_blink_led/
├── stm32/   Cargo.toml  .cargo/config.toml  rust-toolchain.toml  build.rs  memory.x  Embed.toml  src/main.rs
├── rp2040/  (same, memory.x includes BOOT2)
└── esp32/   (no memory.x/build.rs: esp-hal ships linker scripts; uses espflash)
```

## Build and run
```bash
cd stm32   && cargo run --release      # probe-rs flashes + shows defmt logs; Ctrl-C to stop
cd rp2040  && cargo run --release
cd esp32   && cargo run --release      # espflash flashes + opens serial monitor (needs `. ~/export-esp.sh`)
cargo embed --release                  # alternative for STM32/RP2040 using Embed.toml
```

## Code walkthrough (STM32 version; RP2040/ESP32 differ only where noted)

| Line | What / why |
|------|------------|
| `#![no_std]` | Don't link `std`; it needs an OS. Only `core` is available. |
| `#![no_main]` | The Rust runtime's `main` expects an OS. We supply the entry via `#[entry]`. |
| `use cortex_m_rt::entry` | Provides the reset handler (zero `.bss`, copy `.data`, call `main`) and the vector table. |
| `use defmt_rtt as _` | `as _` imports only for its **side effects** (registers the global defmt logger). |
| `use panic_probe as _` | Same trick: defines the mandatory `#[panic_handler]`. A `no_std` binary without one will not link. |
| `fn blink_forever<L: OutputPin, D: DelayNs>` | Generic function: `L` = anything that is a digital output, `D` = anything that can delay. Compiled separately for each concrete type (monomorphisation) -> zero overhead. |
| `-> !` | Never returns. Embedded `main` must not return - there is nothing to return to. |
| `let _ = led.set_high();` | Returns `Result<(), Error>`. Ignoring it is explicit with `let _`. |
| `pac::Peripherals::take().unwrap()` | Singleton pattern: first call gives all peripherals, later calls give `None`. Prevents two parts of the program owning the same hardware. |
| `cortex_m::Peripherals::take()` | Same for ARM core peripherals (SysTick, NVIC). |
| `dp.RCC.constrain()` | Wraps the raw RCC registers in a high-level builder. |
| `.use_hse(25.MHz()).sysclk(84.MHz()).freeze()` | Selects crystal, computes PLL multipliers, writes registers, returns `Clocks`. `25.MHz()` comes from `fugit` - typed units prevent MHz/kHz mix-ups. |
| `dp.GPIOC.split()` | Enables the GPIOC clock and splits the port into independent pin objects (ownership per pin). |
| `gpioc.pc13.into_push_pull_output()` | Consumes the *Input floating* pin and returns an *Output* pin: **type state**. You cannot call `set_high` on an input pin - compile error. |
| `cp.SYST.delay(&clocks)` | SysTick-based blocking delay; needs `clocks` to know the tick rate. |
| `blink_forever(&mut led, &mut delay, 500)` | Pass mutable borrows; the function owns the logic, `main` owns the hardware. |

### RP2040 differences
* `#[link_section = ".boot2"] static BOOT2` places the 256-byte QSPI setup blob at flash start (see docs/02).
* `init_clocks_and_plls` starts the 12 MHz crystal and PLLs; `Watchdog` is passed because its timer provides the 1 µs tick.
* `Sio` + `Pins::new` build the typed pin set; `Timer` replaces SysTick as delay.

### ESP32 differences
* `esp_hal::init(Config::default())` sets clocks and returns the peripherals (no PAC steps).
* `esp_bootloader_esp_idf::esp_app_desc!()` embeds the app descriptor.
* Logging uses `log` + `esp-println` over UART (no RTT). Flash with `espflash`.

## Exercises
1. **Duty cycle:** 100 ms on / 900 ms off. Change `blink_forever` to take `on_ms, off_ms`.
2. **SOS:** blink Morse `... --- ...` using a `const MORSE: &[u16]` pattern array.
3. **Log it:** add `defmt::info!("LED on")` (ESP32: `log::info!`) in the loop and watch the host console. Note the timestamps.
4. **Count:** keep a `u32` counter and log every 10th blink; then change the type to `u8` and see what happens at 256 in debug vs release (overflow checks!).
5. **Generic proof:** write a `struct FakeLed(bool)` implementing `OutputPin` + `ErrorType` and unit-test `blink_forever` logic on your PC (hint: move the loop body to a function that runs once).
6. **Inspect the binary:** `cargo size --release -- -A` before and after adding `defmt::info!`. Where did the bytes go?
7. **Change clocks:** drop `sysclk` to 16 MHz (HSI only, remove `use_hse`). Blink rate must stay the same - why?

## Common mistakes
| Mistake | Symptom | Fix |
|---------|---------|-----|
| Missing `#![no_main]` | link error `undefined reference to main` or startup mismatch | add it |
| No panic handler crate | `error: language item required: panic_impl` | `use panic_probe as _;` |
| Forgot `use defmt_rtt as _` | link error `_defmt_write` | add import |
| Wrong chip feature on HAL | HardFault / wrong pins | `features = ["stm32f411"]` |
| Wrong `--chip` in runner | probe-rs "chip not found" / flash size mismatch | `probe-rs chip list` |
| Expecting LED on when pin high on Black Pill | inverted | LED is active-low |
| LED without resistor | burnt LED / pin | 330 Ω |
| RP2040 without `.boot2` | probe attaches, code never runs | check BOOT2 static + memory.x |
| ESP32: running without `. ~/export-esp.sh` | `rustc: toolchain 'esp' not found` | source the script |
| `delay_ms` while later wanting responsiveness | UI frozen | later modules use timers/async |

## Debugging tips
* `probe-rs list` should show your probe. Use `--connect-under-reset` if the chip won't attach.
* If logs show nothing, check `DEFMT_LOG` and `use defmt_rtt as _;`.
* Step through with `cargo embed --gdb`.
