//! LED pattern engine. Pure functions: (pattern, tick) -> bitmask. Fully host-testable.
//! Bit n of the mask = LED n (1 = on). Supports up to 8 LEDs.

#![cfg_attr(not(test), no_std)]

use embedded_hal::digital::OutputPin;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Pattern {
    /// One LED runs left to right and wraps.
    Chase,
    /// One LED bounces back and forth (Knight Rider).
    Bounce,
    /// Shows `tick` as a binary counter.
    Binary,
    /// All LEDs blink together.
    Blink,
}

impl Pattern {
    /// Next pattern in the cycle (used by the button in main).
    pub fn next(self) -> Self {
        match self {
            Pattern::Chase => Pattern::Bounce,
            Pattern::Bounce => Pattern::Binary,
            Pattern::Binary => Pattern::Blink,
            Pattern::Blink => Pattern::Chase,
        }
    }
}

/// Compute the LED mask for `pattern` at time step `tick` with `n` LEDs (1..=8).
pub fn frame(pattern: Pattern, tick: u32, n: u8) -> u8 {
    let n = n.clamp(1, 8) as u32;
    let full: u8 = if n == 8 { 0xFF } else { (1u8 << n) - 1 }; // mask with n low bits set
    match pattern {
        Pattern::Chase => 1u8 << (tick % n),
        Pattern::Bounce => {
            if n == 1 { return 1; }
            // Period of 2*(n-1) steps: 0,1,..,n-1,n-2,..,1
            let period = 2 * (n - 1);
            let p = tick % period;
            let pos = if p < n { p } else { period - p };
            1u8 << pos
        }
        Pattern::Binary => (tick as u8) & full,
        Pattern::Blink => if tick % 2 == 0 { full } else { 0 },
    }
}

/// Drive a slice of output pins from a mask. `dyn OutputPin` lets pins of *different types*
/// (every GPIO has its own type in Rust HALs) live in one array without a heap.
pub fn apply_mask<E>(leds: &mut [&mut dyn OutputPin<Error = E>], mask: u8) {
    for (i, led) in leds.iter_mut().enumerate() {
        let _ = if mask & (1 << i) != 0 { led.set_high() } else { led.set_low() };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn chase_wraps() {
        assert_eq!(frame(Pattern::Chase, 0, 4), 0b0001);
        assert_eq!(frame(Pattern::Chase, 3, 4), 0b1000);
        assert_eq!(frame(Pattern::Chase, 4, 4), 0b0001);
    }

    #[test]
    fn bounce_goes_there_and_back() {
        let seq: Vec<u8> = (0..6).map(|t| frame(Pattern::Bounce, t, 4)).collect();
        assert_eq!(seq, vec![1, 2, 4, 8, 4, 2]);
    }

    #[test]
    fn binary_counts_and_masks() {
        assert_eq!(frame(Pattern::Binary, 5, 4), 0b0101);
        assert_eq!(frame(Pattern::Binary, 17, 4), 0b0001); // wraps at 16
    }

    #[test]
    fn blink_alternates() {
        assert_eq!(frame(Pattern::Blink, 0, 4), 0b1111);
        assert_eq!(frame(Pattern::Blink, 1, 4), 0);
    }
}
