# 38 - Bootloader basics

> **Status:** 🚧 *Roadmap entry - scaffold only. The full lesson (README, code for STM32 / RP2040 / ESP32, wiring, exercises, common mistakes) is generated in a later batch.*
> **Level:** Expert

## What this module is about
Memory layout, flash organisation, vector tables, handoff.

## Planned key concepts
- Flash sectors & erase granularity
- Vector table & VTOR
- Why a bootloader exists

## Planned deliverables (same as completed modules)
- [ ] `README.md` with concept explanation, line-by-line code walkthrough
- [ ] Wiring diagram (ASCII)
- [ ] Learning objectives
- [ ] `stm32/`, `rp2040/`, `esp32/` crates (`no_std`, embedded-hal 1.0)
- [ ] Exercises
- [ ] Common mistakes

### Bootloader-specific sections that will be included
- [ ] Memory layout (diagram of flash/RAM regions)
- [ ] Flash organisation (sectors, erase size, alignment)
- [ ] Linker scripts (bootloader `memory.x` vs application `memory.x`)
- [ ] Vector tables and `VTOR`
- [ ] Firmware handoff (stack pointer + reset vector jump)
- [ ] Safety mechanisms (CRC, magic words, watchdog, rollback)
- [ ] Example Rust bootloader code

Prerequisites: all earlier modules; read `../docs/` first.
