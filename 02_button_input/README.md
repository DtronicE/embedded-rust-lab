# 02 - Button input (with debouncing)

## Learning objectives
1. Read a digital input; understand floating inputs, pull-up and pull-down resistors.
2. Understand why mechanical switches *bounce* and how to debounce in software.
3. Use `embedded_hal::digital::InputPin` and `StatefulOutputPin` as generic bounds.
4. Split code into a **hardware-independent, host-testable crate** (`shared/`) and thin chip glue.
5. Run `cargo test` for embedded logic on your PC.

## Concept
* **Floating input:** an unconnected pin picks up noise and reads random values.
* **Pull-up:** a resistor to 3.3 V makes the idle level HIGH; the button connects the pin to GND -> **active-low**. This needs no external parts because MCUs have internal pull-ups.
* **Bounce:** metal contacts chatter ~1-20 ms. One press may produce 5-50 edges. We accept a change only after 20 consecutive 1 ms samples agree (a *counter debouncer*).

```
 3.3V ──[ pull-up (internal ~40k) ]──┬──── MCU input
                                     │
                              [ BUTTON ]
                                     │
 GND ────────────────────────────────┘

 Raw signal on press:      ‾‾‾‾‾╲╱╲╱╲_______________   (bounce)
 Debounced (after 20 ms):  ‾‾‾‾‾‾‾‾‾‾‾‾‾‾╲___________
```

## Wiring
```
 STM32 Black Pill : onboard KEY on PA0 (to GND), LED on PC13
 Pico             : button  GP15 ──[BTN]── GND ;   LED GP25 (onboard)
 ESP32 DevKit     : button  GPIO4 ──[BTN]── GND ;  LED GPIO2 (onboard)
```

## Folder structure
```
02_button_input/
├── shared/   Cargo.toml  src/lib.rs      <- Debouncer + 3 unit tests (host)
├── stm32/    ...                          <- depends on ../shared by path
├── rp2040/
└── esp32/
```

## Build / test
```bash
cd shared && cargo test            # runs on your PC, no hardware
cd ../stm32 && cargo run --release
```

## Code walkthrough
**`shared/src/lib.rs`**
| Code | Explanation |
|------|-------------|
| `#![cfg_attr(not(test), no_std)]` | `no_std` normally; during `cargo test` the std test harness is allowed. |
| `enum Event {Pressed, Released}` | Edge events: the program reacts to *changes*, not levels. |
| `struct Debouncer { stable, counter }` | `stable` = accepted state, `counter` = how many consecutive samples disagree. |
| `update(raw)` | If raw equals stable: reset counter. Else increment; at `THRESHOLD` accept the new state and emit an event. |
| `const fn new()` | Can be used in `static` initialisers (module 18). |

**`main.rs` (all chips)**
| Code | Explanation |
|------|-------------|
| `fn button_toggles_led<B, L, D>(...) where B: InputPin, L: OutputPin + StatefulOutputPin` | `toggle()` lives in `StatefulOutputPin` because it must know the output's current state. |
| `button.is_low().unwrap_or(false)` | `is_low` takes `&mut self` and returns `Result<bool, E>`; treat an error as "not pressed". |
| `if let Some(Event::Pressed) = debouncer.update(pressed)` | Pattern-match only the press edge. |
| `delay.delay_ms(1)` | Fixed sample period -> debounce time = `THRESHOLD x 1 ms`. |
| `into_pull_up_input()` / `Pull::Up` | Enables the internal pull-up resistor and switches the pin to input. |

## Exercises
1. Change `THRESHOLD` to 5 and 100. Which feels responsive? Which misses fast presses?
2. Toggle on **release** instead of press.
3. Add **long press** detection (> 1 s -> log "long") using a tick counter in the Debouncer.
4. Add a test `long_press_detected` in `shared` first (TDD), then implement.
5. Count presses and blink the LED that many times.
6. Replace polling with a GPIO interrupt (come back after module 18) - what new problems appear?
7. Remove the debouncer and log every edge. How many edges per press do you see?

## Common mistakes
| Mistake | Effect | Fix |
|---------|--------|-----|
| No pull-up/pull-down | random readings | `into_pull_up_input()` |
| Treating pressed as `is_high` with pull-up | inverted logic | active-low: `is_low` |
| Debounce with `delay_ms(50)` blocking | misses other work | sample-based debouncer |
| Reading too slowly (e.g. every 100 ms) | counter logic breaks | sample every 1 ms |
| Using 5 V buttons/signals on 3.3 V pins | damage | keep 3.3 V |
| ESP32 GPIO34-39 as pull-up input | no internal pulls on those pins | use another pin |
| Black Pill PA0 pressed during reset | enters some boot modes only for BOOT0 pin - PA0 is safe, but BOOT0 is not! | don't hold BOOT0 |

## Debugging tips
* Log raw reads with `defmt::trace!` and set `DEFMT_LOG=trace`.
* Put a scope/logic analyser on the pin to see real bounce time.
