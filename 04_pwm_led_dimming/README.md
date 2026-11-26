# 04 - PWM LED dimming ("breathing" LED)

## Learning objectives
1. Explain PWM: frequency, duty cycle, resolution, average voltage.
2. Configure a hardware timer as a PWM generator on STM32, RP2040 and ESP32 (LEDC).
3. Use `embedded_hal::pwm::SetDutyCycle` to write chip-independent code.
4. Apply **gamma correction** because eye brightness is non-linear.
5. Avoid floating point on chips without an FPU (RP2040/Cortex-M0+).

## Concept
PWM switches a pin fully on/off fast. The LED (and your eye) averages it:
```
 duty 25%:  ┌─┐       ┌─┐       ┌─┐        average ≈ 0.25 x 3.3 V
            │ └───────┘ └───────┘ └──
 duty 75%:  ┌───────┐ ┌───────┐ ┌───────
            │       └─┘       └─┘
            |<-period = 1/f ->|
```
A hardware timer counts 0..ARR. The output is HIGH while `counter < CCR` (compare value).
`duty = CCR / (ARR+1)`. The timer does this **with no CPU involvement**, so your code can sleep.

* Frequency: >= ~200 Hz to avoid visible flicker (we use 1-5 kHz).
* Resolution: 16-bit on STM32/RP2040 counters, 8-bit chosen here for LEDC. Higher frequency <-> fewer bits at a given clock.
* Gamma: duty 50% looks ~75% bright; `out = in^2` approximates correction.

## Wiring
```
 Output pin ──[330 Ω]──►|── GND

 STM32 Black Pill : PA6 (TIM3_CH1) -> LED          (any TIMx_CHy pin works; see datasheet AF table)
 Pico             : GP25 on-board LED (PWM4 B) - nothing to wire
 ESP32            : GPIO2 (LEDC can route to ANY output pin)
```

## Folder structure
`shared/` = triangle wave + gamma (tests) · `stm32/` `rp2040/` `esp32/` = glue.

## Build
`(cd shared && cargo test)` then `cd <chip> && cargo run --release`.

## Code walkthrough
| Code | Explanation |
|------|-------------|
| `fn breathe<P: SetDutyCycle, D: DelayNs>` | Portable loop. Any PWM channel + any delay. |
| `pwm.max_duty_cycle()` | The count representing 100%; hardware decides (65535, 255, ...). Never hard-code it. |
| `triangle(step, STEPS, max)` | Linear ramp up and down, integer maths. |
| `gamma2(level, max)` | `level^2 / max`: darker low end. |
| `pwm.set_duty_cycle(duty as u16)` | Writes the compare register (CCR). |
| STM32: `Channel1::new(gpioa.pa6)` | Moves the pin into the "TIM3 channel 1" role (AF2). |
| STM32: `dp.TIM3.pwm_hz(channels, 1.kHz(), &clocks)` | Computes PSC/ARR from clock and requested frequency. |
| STM32: `.split()`, `pwm.enable()` | Separate per-channel handle; starts output. |
| RP2040: slice/channel mapping | Pin -> slice is fixed: GPIO n -> slice (n/2)%8, channel A if even else B. |
| RP2040: `set_ph_correct()` | Phase-correct (up/down) counting. |
| RP2040: `channel.output_to(pin)` | Connects PWM output to pad (consumes the pin). |
| ESP32: `ledc.timer`, `.configure(...)` | LEDC timer: resolution + frequency. |
| ESP32: `ledc.channel(Channel0, led)` | Binds timer to a pin; any GPIO possible via the GPIO matrix. |

## Exercises
1. Change PWM frequency to 100 Hz: see flicker. Try 20 kHz.
2. Disable gamma (use `linear`) and compare to your eye.
3. Make breathing speed adjustable by button (module 02).
4. Drive an RGB LED with three channels; cycle hue (integer HSV->RGB function in `shared`, with tests).
5. Compute by hand: STM32 at 84 MHz, PSC=83, ARR=999 -> output frequency? (1 kHz). Verify using `max_duty_cycle()`.
6. Use a scope/logic analyser to measure frequency and duty.
7. Replace `delay_ms(4)` by a timer interrupt after module 19.

## Common mistakes
| Mistake | Effect | Fix |
|---------|--------|-----|
| Wrong pin for timer channel | no output | check the datasheet alternate-function table |
| Two channels sharing a timer expecting different frequencies | frequency is per timer/slice | use separate timers/slices |
| Hard-coded max duty 255 | wrong brightness | `max_duty_cycle()` |
| Floating point on RP2040 | slow, large code | integer maths |
| Duty > max | wraps / full-on | clamp |
| Forgetting `enable()` (STM32/RP2040) | static LED | enable the channel |
| ESP32: wrong clock/resolution combo | `configure` returns Err | lower frequency or bits |
| Driving a motor/relay from the pin | damage | transistor/MOSFET driver (module 26) |

## Debugging tips
* Log `max` and duty with defmt. Check slice/channel numbers if RP2040 shows nothing.
* Logic analyser: confirm frequency first, then duty.
