# embedded-rust-lab

A hands-on course in **`no_std` Embedded Rust** using the `embedded-hal` 1.0 ecosystem on three platforms:

| Family | Board | Core | Target triple | Flash/debug tool |
|--------|-------|------|---------------|------------------|
| STM32 | F411 "Black Pill" | Cortex-M4F | `thumbv7em-none-eabihf` | probe-rs / cargo-embed (ST-Link) |
| RP2040 | Raspberry Pi Pico | Cortex-M0+ | `thumbv6m-none-eabi` | probe-rs / cargo-embed (picoprobe) |
| ESP32 | classic DevKit (Xtensa) | LX6 | `xtensa-esp32-none-elf` | espflash (+ JTAG for probe-rs) |

Editor: **Zed** - every chip crate has `.zed/settings.json` and `.zed/tasks.json`; open the *chip crate folder*.

## ⚠️ Honest status
* Foundation docs + modules **01-04 are fully written** (README, 3 chips, wiring, exercises, mistakes).
* Modules 05-40 and projects 01-07 exist as **roadmap stubs** with planned concepts; they are generated in later batches so each gets real quality instead of filler.
* The code was written against pinned crate versions (see each `Cargo.toml`) but **has not been compiled on hardware by the author of this scaffold**. HAL APIs - especially `esp-hal` and `stm32f4xx-hal` PWM - move between releases; each file says where to look if something fails. Treat compile errors as part of the learning: `docs.rs` + the HAL's `examples/` folder is the source of truth.

## Read this first (concept docs)
1. [00 Toolchain setup, probes, Zed](docs/00_toolchain_setup.md)
2. [01 Cargo for embedded](docs/01_cargo_for_embedded.md)
3. [02 memory.x, linker scripts, startup code, vector table](docs/02_memory_x_linker_startup.md)
4. [03 embedded-hal and the layered stack](docs/03_embedded_hal.md)
5. [04 Debugging with probe-rs and cargo-embed](docs/04_debugging_probe_rs.md)

## Design rules used in every module
* **Portable core, thin glue:** logic lives in generic functions bounded by `embedded-hal` traits or in a host-testable `shared/` crate; `main.rs` only initialises clocks/pins.
* Each chip crate is an independent Cargo project (own target, runner, linker flags).
* `defmt` + RTT for logs on STM32/RP2040; `log` + `esp-println` on ESP32.
* Pure integer maths where possible (RP2040 has no FPU).
* Each module README: objectives -> concept -> wiring -> walkthrough -> exercises -> common mistakes -> debugging.

## Repository tree
```
embedded-rust-lab/
├── 01_blink_led/
│   ├── esp32
│   │   ├── .cargo
│   │   │   └── config.toml
│   │   ├── .zed
│   │   │   ├── settings.json
│   │   │   └── tasks.json
│   │   ├── src
│   │   │   └── main.rs
│   │   ├── Cargo.toml
│   │   └── rust-toolchain.toml
│   ├── rp2040
│   │   ├── .cargo
│   │   │   └── config.toml
│   │   ├── .zed
│   │   │   ├── settings.json
│   │   │   └── tasks.json
│   │   ├── src
│   │   │   └── main.rs
│   │   ├── Cargo.toml
│   │   ├── Embed.toml
│   │   ├── build.rs
│   │   ├── memory.x
│   │   └── rust-toolchain.toml
│   ├── stm32
│   │   ├── .cargo
│   │   │   └── config.toml
│   │   ├── .zed
│   │   │   ├── settings.json
│   │   │   └── tasks.json
│   │   ├── src
│   │   │   └── main.rs
│   │   ├── Cargo.toml
│   │   ├── Embed.toml
│   │   ├── build.rs
│   │   ├── memory.x
│   │   └── rust-toolchain.toml
│   └── README.md
├── 02_button_input/
│   └── README.md, shared/, stm32/, rp2040/, esp32/   (same layout as 01 + shared/)
├── 03_led_patterns/
│   └── README.md, shared/, stm32/, rp2040/, esp32/   (same layout as 01 + shared/)
├── 04_pwm_led_dimming/
│   └── README.md, shared/, stm32/, rp2040/, esp32/   (same layout as 01 + shared/)
├── 05_uart_serial_tx/
│   └── README.md   (roadmap stub -> grows into stm32/ rp2040/ esp32/ like 01)
├── 06_uart_serial_rx/
│   └── README.md   (roadmap stub -> grows into stm32/ rp2040/ esp32/ like 01)
├── 07_uart_serial_terminal/
│   └── README.md   (roadmap stub -> grows into stm32/ rp2040/ esp32/ like 01)
├── 08_i2c_scan/
│   └── README.md   (roadmap stub -> grows into stm32/ rp2040/ esp32/ like 01)
├── 09_i2c_temperature_sensor/
│   └── README.md   (roadmap stub -> grows into stm32/ rp2040/ esp32/ like 01)
├── 10_i2c_eeprom/
│   └── README.md   (roadmap stub -> grows into stm32/ rp2040/ esp32/ like 01)
├── 11_spi_basics/
│   └── README.md   (roadmap stub -> grows into stm32/ rp2040/ esp32/ like 01)
├── 12_spi_display/
│   └── README.md   (roadmap stub -> grows into stm32/ rp2040/ esp32/ like 01)
├── 13_oled_display_ssd1306/
│   └── README.md   (roadmap stub -> grows into stm32/ rp2040/ esp32/ like 01)
├── 14_lcd_16x2/
│   └── README.md   (roadmap stub -> grows into stm32/ rp2040/ esp32/ like 01)
├── 15_rtc_clock/
│   └── README.md   (roadmap stub -> grows into stm32/ rp2040/ esp32/ like 01)
├── 16_adc_reading/
│   └── README.md   (roadmap stub -> grows into stm32/ rp2040/ esp32/ like 01)
├── 17_potentiometer/
│   └── README.md   (roadmap stub -> grows into stm32/ rp2040/ esp32/ like 01)
├── 18_interrupts/
│   └── README.md   (roadmap stub -> grows into stm32/ rp2040/ esp32/ like 01)
├── 19_timers/
│   └── README.md   (roadmap stub -> grows into stm32/ rp2040/ esp32/ like 01)
├── 20_watchdog/
│   └── README.md   (roadmap stub -> grows into stm32/ rp2040/ esp32/ like 01)
├── 21_rust_logging/
│   └── README.md   (roadmap stub -> grows into stm32/ rp2040/ esp32/ like 01)
├── 22_low_power_modes/
│   └── README.md   (roadmap stub -> grows into stm32/ rp2040/ esp32/ like 01)
├── 23_dma_basics/
│   └── README.md   (roadmap stub -> grows into stm32/ rp2040/ esp32/ like 01)
├── 24_rtic_introduction/
│   └── README.md   (roadmap stub -> grows into stm32/ rp2040/ esp32/ like 01)
├── 25_multitasking/
│   └── README.md   (roadmap stub -> grows into stm32/ rp2040/ esp32/ like 01)
├── 26_motor_control_pwm/
│   └── README.md   (roadmap stub -> grows into stm32/ rp2040/ esp32/ like 01)
├── 27_servo_motor/
│   └── README.md   (roadmap stub -> grows into stm32/ rp2040/ esp32/ like 01)
├── 28_stepper_motor/
│   └── README.md   (roadmap stub -> grows into stm32/ rp2040/ esp32/ like 01)
├── 29_encoder_interface/
│   └── README.md   (roadmap stub -> grows into stm32/ rp2040/ esp32/ like 01)
├── 30_buzzer_audio/
│   └── README.md   (roadmap stub -> grows into stm32/ rp2040/ esp32/ like 01)
├── 31_sd_card_storage/
│   └── README.md   (roadmap stub -> grows into stm32/ rp2040/ esp32/ like 01)
├── 32_filesystem_fat/
│   └── README.md   (roadmap stub -> grows into stm32/ rp2040/ esp32/ like 01)
├── 33_usb_serial/
│   └── README.md   (roadmap stub -> grows into stm32/ rp2040/ esp32/ like 01)
├── 34_usb_hid_keyboard/
│   └── README.md   (roadmap stub -> grows into stm32/ rp2040/ esp32/ like 01)
├── 35_can_bus/
│   └── README.md   (roadmap stub -> grows into stm32/ rp2040/ esp32/ like 01)
├── 36_rs485/
│   └── README.md   (roadmap stub -> grows into stm32/ rp2040/ esp32/ like 01)
├── 37_modbus/
│   └── README.md   (roadmap stub -> grows into stm32/ rp2040/ esp32/ like 01)
├── 38_bootloader_basics/
│   └── README.md   (roadmap stub -> grows into stm32/ rp2040/ esp32/ like 01)
├── 39_custom_bootloader/
│   └── README.md   (roadmap stub -> grows into stm32/ rp2040/ esp32/ like 01)
├── 40_firmware_update_system/
│   └── README.md   (roadmap stub -> grows into stm32/ rp2040/ esp32/ like 01)
├── docs/
│   ├── 00_toolchain_setup.md
│   ├── 01_cargo_for_embedded.md
│   ├── 02_memory_x_linker_startup.md
│   ├── 03_embedded_hal.md
│   └── 04_debugging_probe_rs.md
├── project_01_digital_thermometer/
│   └── README.md   (roadmap stub -> grows into stm32/ rp2040/ esp32/ like 01)
├── project_02_data_logger/
│   └── README.md   (roadmap stub -> grows into stm32/ rp2040/ esp32/ like 01)
├── project_03_weather_station/
│   └── README.md   (roadmap stub -> grows into stm32/ rp2040/ esp32/ like 01)
├── project_04_rust_oscilloscope/
│   └── README.md   (roadmap stub -> grows into stm32/ rp2040/ esp32/ like 01)
├── project_05_robot_controller/
│   └── README.md   (roadmap stub -> grows into stm32/ rp2040/ esp32/ like 01)
├── project_06_iot_sensor_node/
│   └── README.md   (roadmap stub -> grows into stm32/ rp2040/ esp32/ like 01)
└── project_07_home_automation_controller/
    └── README.md   (roadmap stub -> grows into stm32/ rp2040/ esp32/ like 01)
```

## Modules
| # | Module | Level | Status |
|---|--------|-------|--------|
| 01 | [Blink LED (deep dive)](01_blink_led/) | Beginner | ✅ complete |
| 05 | [UART serial TX](05_uart_serial_tx/) | Intermediate | 🚧 roadmap |
| 06 | [UART serial RX](06_uart_serial_rx/) | Intermediate | 🚧 roadmap |
| 07 | [UART serial terminal](07_uart_serial_terminal/) | Intermediate | 🚧 roadmap |
| 08 | [I2C bus scan](08_i2c_scan/) | Intermediate | 🚧 roadmap |
| 09 | [I2C temperature sensor](09_i2c_temperature_sensor/) | Intermediate | 🚧 roadmap |
| 10 | [I2C EEPROM](10_i2c_eeprom/) | Intermediate | 🚧 roadmap |
| 11 | [SPI basics](11_spi_basics/) | Intermediate | 🚧 roadmap |
| 12 | [SPI TFT display](12_spi_display/) | Intermediate | 🚧 roadmap |
| 13 | [SSD1306 OLED](13_oled_display_ssd1306/) | Intermediate | 🚧 roadmap |
| 14 | [HD44780 16x2 LCD](14_lcd_16x2/) | Intermediate | 🚧 roadmap |
| 15 | [RTC clock](15_rtc_clock/) | Intermediate | 🚧 roadmap |
| 16 | [ADC reading](16_adc_reading/) | Intermediate | 🚧 roadmap |
| 17 | [Potentiometer input](17_potentiometer/) | Intermediate | 🚧 roadmap |
| 18 | [Interrupts](18_interrupts/) | Advanced | 🚧 roadmap |
| 19 | [Hardware timers](19_timers/) | Advanced | 🚧 roadmap |
| 20 | [Watchdog](20_watchdog/) | Advanced | 🚧 roadmap |
| 21 | [Logging](21_rust_logging/) | Intermediate | 🚧 roadmap |
| 22 | [Low-power modes](22_low_power_modes/) | Advanced | 🚧 roadmap |
| 23 | [DMA basics](23_dma_basics/) | Advanced | 🚧 roadmap |
| 24 | [RTIC introduction](24_rtic_introduction/) | Advanced | 🚧 roadmap |
| 25 | [Multitasking with Embassy](25_multitasking/) | Advanced | 🚧 roadmap |
| 26 | [DC motor control](26_motor_control_pwm/) | Intermediate | 🚧 roadmap |
| 27 | [Servo motor](27_servo_motor/) | Intermediate | 🚧 roadmap |
| 28 | [Stepper motor](28_stepper_motor/) | Advanced | 🚧 roadmap |
| 29 | [Quadrature encoder](29_encoder_interface/) | Advanced | 🚧 roadmap |
| 30 | [Buzzer & tones](30_buzzer_audio/) | Intermediate | 🚧 roadmap |
| 31 | [SD card (SPI)](31_sd_card_storage/) | Advanced | 🚧 roadmap |
| 32 | [FAT filesystem](32_filesystem_fat/) | Advanced | 🚧 roadmap |
| 33 | [USB CDC serial](33_usb_serial/) | Advanced | 🚧 roadmap |
| 34 | [USB HID keyboard](34_usb_hid_keyboard/) | Advanced | 🚧 roadmap |
| 35 | [CAN bus](35_can_bus/) | Advanced | 🚧 roadmap |
| 36 | [RS-485](36_rs485/) | Advanced | 🚧 roadmap |
| 37 | [Modbus RTU](37_modbus/) | Advanced | 🚧 roadmap |
| 38 | [Bootloader basics](38_bootloader_basics/) | Expert | 🚧 roadmap |
| 39 | [Custom bootloader](39_custom_bootloader/) | Expert | 🚧 roadmap |
| 40 | [Firmware update system](40_firmware_update_system/) | Expert | 🚧 roadmap |

## Advanced projects
| Project | Status |
|---|---|
| [Digital thermometer](project_01_digital_thermometer/) | 🚧 roadmap |
| [Data logger](project_02_data_logger/) | 🚧 roadmap |
| [Weather station](project_03_weather_station/) | 🚧 roadmap |
| [Rust oscilloscope](project_04_rust_oscilloscope/) | 🚧 roadmap |
| [Robot controller](project_05_robot_controller/) | 🚧 roadmap |
| [IoT sensor node](project_06_iot_sensor_node/) | 🚧 roadmap |
| [Home automation controller](project_07_home_automation_controller/) | 🚧 roadmap |

## Suggested pace
Beginner (01-04): week 1 · Serial/I2C/SPI (05-14): weeks 2-4 · Analog, interrupts, timers (15-20): weeks 5-6 · Systems (21-25): weeks 7-8 · Actuators (26-30): week 9 · Storage/USB/buses (31-37): weeks 10-12 · Bootloaders (38-40): weeks 13-14 · Projects after.

## Quick start
```bash
cd 01_blink_led/rp2040 && cargo run --release
(cd 02_button_input/shared && cargo test)     # host tests, no hardware
```
