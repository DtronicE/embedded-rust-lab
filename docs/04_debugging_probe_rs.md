# 04 - Debugging with probe-rs and cargo-embed

## Quick commands

| Goal | Command |
|------|---------|
| list probes | `probe-rs list` |
| list supported chips | `probe-rs chip list \| grep -i stm32f411` |
| flash + run + defmt logs | `cargo run --release` (runner in `.cargo/config.toml`) |
| flash with config file | `cargo embed --release` (uses `Embed.toml`) |
| attach without flashing | `probe-rs attach --chip RP2040 target/thumbv6m-none-eabi/release/<name>` |
| GDB server | `cargo embed --release --gdb` then `arm-none-eabi-gdb -ex "target remote :1337" <elf>` |
| erase chip | `probe-rs erase --chip STM32F411CEUx` |
| reset | `probe-rs reset --chip RP2040` |

## defmt + RTT (how logs get out without UART)
* `defmt::info!("x = {}", x)` does **not** format on the MCU. It sends a tiny index + raw bytes.
* The strings are stored in the **.elf** (section from `-Tdefmt.x`), not in flash.
* `defmt-rtt` writes into a ring buffer in RAM named `_SEGGER_RTT`. probe-rs reads that RAM through the debug port while the CPU runs - no pins, no baud rate.
* `DEFMT_LOG=trace|debug|info|warn|error` (set in `.cargo/config.toml [env]`) filters at compile time.

## Panics
`panic-probe` with `print-defmt` prints the panic message through defmt and then triggers a breakpoint/HardFault. With `probe-rs run` you get a **backtrace** with file:line - this is why `debug = 2` is in the profile.

## Embed.toml reference
```toml
[default.general]
chip = "STM32F411CEUx"
[default.rtt]
enabled = true          # open the RTT console
up_mode = "NoBlockSkip" # drop log lines instead of stalling the CPU if the buffer is full
[default.gdb]
enabled = false         # true => start a GDB server on :1337
```

## Typical problems
| Symptom | Cause / fix |
|---------|-------------|
| `no probe found` | cable/driver/udev rules; `probe-rs list` |
| `Error: Timeout / could not attach` | chip in deep sleep or SWD pins reconfigured by your own code; use *connect under reset*: `probe-rs run --connect-under-reset` |
| Logs stop after a while | RTT buffer full in `Block` mode while no host is attached; use `NoBlockSkip` |
| HardFault immediately | wrong `memory.x`, wrong chip feature, or stack overflow (try `flip-link`) |
| RP2040 runs but nothing happens | missing `.boot2` section (check `memory.x` + `BOOT2` static) |
| Link error `undefined symbol _defmt_timestamp` | missing `defmt-rtt`/`defmt` import (`use defmt_rtt as _;`) |
| ESP32 `Failed to connect` | hold BOOT while resetting; check cable is data-capable |

## ESP32 notes
Use `espflash` + the serial monitor (esp-println/`log`) for everyday work. For real step-debugging on classic ESP32 you need an external JTAG probe; ESP32-S3/C3/C6 use the built-in USB-JTAG with probe-rs directly (change `--chip esp32s3`).
