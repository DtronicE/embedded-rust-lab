# 06 - UART serial RX

> **Status:** 🚧 *Roadmap entry - scaffold only. The full lesson (README, code for STM32 / RP2040 / ESP32, wiring, exercises, common mistakes) is generated in a later batch.*
> **Level:** Intermediate

## What this module is about
Receive bytes: blocking, polled, and interrupt-fed ring buffer.

## Planned key concepts
- Overrun / framing / parity errors
- `nb` vs blocking reads, `heapless::spsc::Queue`
- Never block forever

## Planned deliverables (same as completed modules)
- [ ] `README.md` with concept explanation, line-by-line code walkthrough
- [ ] Wiring diagram (ASCII)
- [ ] Learning objectives
- [ ] `stm32/`, `rp2040/`, `esp32/` crates (`no_std`, embedded-hal 1.0)
- [ ] Exercises
- [ ] Common mistakes

Prerequisites: all earlier modules; read `../docs/` first.
