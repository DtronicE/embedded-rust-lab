//! Breathing-LED maths in pure integer arithmetic (no floats: Cortex-M0+ of the RP2040 has no FPU).

#![cfg_attr(not(test), no_std)]

/// Triangle wave 0..=max..=0 over `steps` steps per half cycle.
/// step runs 0..2*steps; returns a *linear* level 0..=max.
pub fn triangle(step: u32, steps: u32, max: u32) -> u32 {
    let p = step % (2 * steps);
    let up = if p < steps { p } else { 2 * steps - p };
    up * max / steps
}

/// Approximate gamma 2.0 correction: out = max * (level/max)^2.
/// Human brightness perception is logarithmic, so linear duty looks "too bright too early".
pub fn gamma2(level: u32, max: u32) -> u32 {
    // u64 avoids overflow for 16-bit duty values (65535^2 > u32::MAX is false, but be safe).
    ((level as u64 * level as u64) / max as u64) as u32
}

/// Convert a duty value to a percentage 0..=100.
pub fn percent(duty: u32, max: u32) -> u32 {
    duty * 100 / max
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn triangle_shape() {
        assert_eq!(triangle(0, 4, 100), 0);
        assert_eq!(triangle(4, 4, 100), 100);
        assert_eq!(triangle(8, 4, 100), 0);
        assert_eq!(triangle(6, 4, 100), 50);
    }

    #[test]
    fn gamma_is_monotonic_and_bounded() {
        let mut prev = 0;
        for l in 0..=255 {
            let g = gamma2(l, 255);
            assert!(g >= prev && g <= 255);
            prev = g;
        }
        assert_eq!(gamma2(255, 255), 255);
        assert_eq!(gamma2(128, 255), 64);
    }
}
