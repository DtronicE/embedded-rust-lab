# 03 - LED patterns

## Learning objectives
1. Drive several GPIO outputs; understand per-pin **types** in Rust HALs.
2. Use **trait objects** (`&mut dyn OutputPin`) to treat different pin types uniformly - without a heap.
3. Separate *what to show* (pure function `frame`) from *how to show it* (`apply_mask`).
4. Write non-blocking timing with a 1 ms "tick" loop instead of long `delay_ms` calls.
5. Write host unit tests for pattern logic; reuse crates between modules (`m02-shared`).

## Concept
Each pattern is a function `(pattern, tick) -> u8 mask`. Bit *n* = LED *n*. No hardware involved, so we
can test it on a PC. `apply_mask` writes the mask to the pins.

Why `dyn`? In Rust `Pin<PB0>` and `Pin<PB1>` are **different types**. An array needs one type. Options:
| Technique | Cost | Used when |
|-----------|------|-----------|
| `dyn OutputPin` (trait object) | one indirect call per access | simple, few pins (this module) |
| HAL `.erase()` / `degrade()` | tiny | HAL offers it; pins become one type |
| Generics / macros | none | when performance critical |

## Wiring (4 LEDs, one resistor each)
```
 GPIO0 ──[330]──►|── GND      STM32: PB0 PB1 PB2 PB3
 GPIO1 ──[330]──►|── GND      RP2040: GP16 GP17 GP18 GP19
 GPIO2 ──[330]──►|── GND      ESP32: GPIO16 17 18 19
 GPIO3 ──[330]──►|── GND
 Button (to GND): STM32 PA0 | RP2040 GP15 | ESP32 GPIO4 (internal pull-up)
```
Total LED current must stay under the chip limit (~5 mA per LED is fine).

## Folder structure
```
03_led_patterns/
├── shared/ src/lib.rs      Pattern, frame(), apply_mask() + tests
├── stm32/ rp2040/ esp32/   each depends on ../shared and ../../02_button_input/shared
```

## Build / test
```bash
(cd shared && cargo test)
cd stm32 && cargo run --release
```

## Code walkthrough
**shared**
| Code | Explanation |
|------|-------------|
| `enum Pattern` + `next()` | A simple state enum; `match` must be exhaustive so adding a variant forces you to handle it. |
| `n.clamp(1, 8)` | Defensive: avoids shifting by >= 8 which would panic in debug. |
| `full = (1u8 << n) - 1` | n low bits set, e.g. n=4 -> `0b1111`. Special-cased for n=8 to avoid overflow. |
| `Chase: 1 << (tick % n)` | One lit LED moving; `%` wraps. |
| `Bounce: period = 2*(n-1)` | Triangle wave: position goes up then down. `if p < n {p} else {period-p}`. |
| `Binary: tick as u8 & full` | Truncation keeps low bits = counter. |
| `apply_mask<E>(leds: &mut [&mut dyn OutputPin<Error=E>], mask)` | Slice of trait objects; generic error type `E` keeps it chip-independent. |

**main**
| Code | Explanation |
|------|-------------|
| `[&mut l0, &mut l1, ...]` typed as `[&mut dyn OutputPin<Error=Infallible>; 4]` | Unsized coercion: each concrete pin reference becomes a trait object. |
| `ms_in_step` | Counts 1 ms loops; every 150 ms we compute the next frame. Button polling continues each ms. |
| `tick.wrapping_add(1)` | Explicit wrap instead of overflow panic. |

## Exercises
1. Add a `Random` pattern using a small LFSR (`x ^= x << 1 ...`) in `frame`. Test determinism.
2. Make step time depend on the pattern (Blink slower).
3. Add a pattern `Stack` that fills LEDs one by one then clears. Write the test first.
4. Support 8 LEDs by adding wiring; test `frame` with n=8.
5. Replace `dyn` with a const-generic `[Pin; N]` approach using the HAL's pin erasure (`.erase()` on STM32 pins). Compare binary size.
6. Make a speed button: short press = next pattern, long press = faster (needs long-press from module 02 exercise 3).

## Common mistakes
| Mistake | Effect | Fix |
|---------|--------|-----|
| One resistor for several LEDs | uneven brightness | one per LED |
| `1u8 << 8` | panic "shift left with overflow" in debug | clamp / special-case |
| `delay_ms(150)` in the loop | button unresponsive | 1 ms tick loop |
| Forgetting `mut` on pins | `cannot borrow as mutable` | `let mut l0` |
| Mixing pin lists of different `Error` types | trait object mismatch | `Error = Infallible` (GPIO) |
| ESP32 GPIO6-11 | connected to flash, crash | avoid |
| ESP32 GPIO12 strapping pin high at boot | boot failure | avoid LEDs pulling it high |

## Debugging tips
* `defmt::info!("mask={:08b}", mask)` prints the pattern.
* Use a logic analyser or just watch for the wrong LED order = wiring or bit order bug.
