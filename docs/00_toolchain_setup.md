# 00 - Toolchain setup (Rust + probe-rs + Zed)

## 1. Rust and targets

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup target add thumbv7em-none-eabihf   # STM32F4 (Cortex-M4F)
rustup target add thumbv6m-none-eabi      # RP2040  (Cortex-M0+)
```

A *target triple* tells rustc what CPU/ABI/OS to generate code for.
`thumbv7em-none-eabihf` = Thumb-2 instruction set, Cortex-M4/M7 ("em"), **no OS** (`none`),
EABI with **h**ard **f**loat. `none` means there is no `std`: we use `#![no_std]`.

Each chip crate has a `rust-toolchain.toml`, so `cd`-ing into it selects the right toolchain.

## 2. Flashing / debugging tools

```bash
cargo install probe-rs-tools --locked     # provides: probe-rs, cargo-embed, cargo-flash
cargo install flip-link                   # optional: stack-overflow protection linker (later modules)
cargo install cargo-binutils              # cargo size / objdump (needs `llvm-tools` component)
cargo install espflash --locked           # ESP32 flashing + serial monitor
cargo install espup --locked && espup install   # Xtensa Rust toolchain for classic ESP32
. $HOME/export-esp.sh                     # run in every new shell for ESP32 work
```

| Tool | What it does |
|------|--------------|
| `probe-rs` | Talks to the SWD/JTAG probe: flash, reset, RTT logs, GDB server, DAP for editors |
| `cargo embed` | Reads `Embed.toml`; flashes and opens RTT / GDB with config in a file |
| `cargo run` | Uses the `runner` in `.cargo/config.toml` (here `probe-rs run`) to flash + stream logs |
| `espflash` | Flashes ESP32 over its USB-UART bootloader and shows the serial console |

## 3. Probes

* **STM32 Black Pill**: ST-Link V2 clone (4 wires: SWDIO, SWCLK, GND, 3V3).
* **RP2040 Pico**: a second Pico running `debugprobe` firmware (picoprobe) - 3 wires: SWCLK, SWDIO, GND. Or simply hold BOOTSEL and use `elf2uf2-rs`.
* **ESP32 (classic)**: no probe needed for flashing (USB-UART). Debugging with probe-rs needs an external JTAG probe (GPIO12-15); ESP32-S3/C3/C6 have built-in USB-JTAG that probe-rs can use.

### Linux permissions
Install probe-rs udev rules (see <https://probe.rs/docs/getting-started/probe-setup/>), then `sudo udevadm control --reload && sudo udevadm trigger`. Add yourself to `dialout` for serial ports.

## 4. Zed editor

1. Install Zed; the **Rust extension** is built in (rust-analyzer is downloaded automatically).
2. **Open the chip crate folder** (e.g. `01_blink_led/stm32`), *not* the repo root. rust-analyzer analyses one target per project; opening the root would mix thumbv7em, thumbv6m and Xtensa crates.
3. Each crate has `.zed/settings.json` (rust-analyzer: clippy on save, no `--all-targets` because `#![no_std] #![no_main]` binaries have no test harness) and `.zed/tasks.json`.
4. Run tasks with `Cmd/Ctrl-Shift-P` -> `task: spawn` -> *flash + RTT (run)*.
5. The status bar will show `rust-analyzer` errors like "can't find crate for `test`" if it tries to build tests for a `no_std` target - that is why `allTargets` is `false`.
6. Zed also supports debugging via DAP. probe-rs ships a DAP server (`probe-rs dap-server`), so you can add a debug configuration in `.zed/debug.json` that launches it; see <https://probe.rs/docs/tools/debugger/>. The tasks in this repo give you a text-mode workflow that always works.

## 5. Sanity check

```bash
cd 01_blink_led/rp2040
cargo build --release      # compiles for thumbv6m-none-eabi
cargo run --release        # flashes through the probe and streams defmt logs
```
