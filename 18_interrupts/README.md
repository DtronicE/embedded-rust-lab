# 18 - Interrupts

> **Status:** 🚧 *Roadmap entry - scaffold only. The full lesson (README, code for STM32 / RP2040 / ESP32, wiring, exercises, common mistakes) is generated in a later batch.*
> **Level:** Advanced

## What this module is about
NVIC, EXTI/GPIO interrupts, sharing state safely.

## Planned key concepts
- `critical_section::Mutex<RefCell<Option<T>>>`
- Atomics, `#[interrupt]`
- Keeping ISRs short

## Planned deliverables (same as completed modules)
- [ ] `README.md` with concept explanation, line-by-line code walkthrough
- [ ] Wiring diagram (ASCII)
- [ ] Learning objectives
- [ ] `stm32/`, `rp2040/`, `esp32/` crates (`no_std`, embedded-hal 1.0)
- [ ] Exercises
- [ ] Common mistakes

Prerequisites: all earlier modules; read `../docs/` first.
