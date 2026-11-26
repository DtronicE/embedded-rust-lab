# 03 - embedded-hal and the Rust embedded layers

```
   your application (main.rs)
        │ uses traits (embedded-hal 1.0 / embedded-io / embedded-can)
   driver crates (ssd1306, bme280, mipidsi...)       <- written once, work on every chip
        │ generic over  I2c / SpiDevice / OutputPin / DelayNs
   HAL crate (stm32f4xx-hal, rp2040-hal, esp-hal)    <- implements the traits for one family
        │
   PAC (svd2rust: stm32f4, rp2040-pac)               <- type-safe register access, auto-generated
        │
   hardware registers
```
* **PAC** (Peripheral Access Crate): one struct per peripheral, one method per register field. Zero cost, but low level.
* **HAL**: ergonomic ownership-based API (`Pin<...>` types, `Serial`, `Spi`). Implements `embedded-hal` traits.
* **embedded-hal 1.0**: stable trait definitions only: `digital::{InputPin, OutputPin, StatefulOutputPin}`, `delay::DelayNs`, `i2c::I2c`, `spi::{SpiBus, SpiDevice}`, `pwm::SetDutyCycle`. (Serial moved to `embedded-io`, CAN to `embedded-can`, async versions to `embedded-hal-async`.)
* **embedded-hal-bus**: helpers to *share* a bus between several drivers (`ExclusiveDevice`, `RefCellDevice`, `CriticalSectionDevice`).

## Why traits? The same logic on three chips

```rust
use embedded_hal::{delay::DelayNs, digital::OutputPin};

fn blink<L: OutputPin, D: DelayNs>(led: &mut L, delay: &mut D) {
    let _ = led.set_high();
    delay.delay_ms(500);
    let _ = led.set_low();
    delay.delay_ms(500);
}
```
This function compiles unchanged for STM32, RP2040 and ESP32. Generics are monomorphised at compile time -> **no runtime cost** (unlike C function pointers). Only `main()` (clock + pin setup) is chip-specific.

## Ownership = safety
`Peripherals::take()` returns `Some` exactly once. Pins are *moved* into drivers; you cannot accidentally use GPIO5 as both I2C SDA and an LED: the compiler rejects it. Pin modes are **type states**: `Pin<Input>` has no `set_high`.

## Error handling
Every trait has an associated `Error` type. GPIO on most chips is `Infallible`, so `let _ = led.set_high();` is fine. I2C/SPI can fail (NACK, bus error) - always propagate or log those.

## Blocking vs async
Everything in this repo up to module 24 is blocking. Module 25 shows `embedded-hal-async` with Embassy.
