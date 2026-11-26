//! Hardware-independent button debouncer. Runs (and is unit-tested) on your PC.
//!
//! Idea: a mechanical switch "bounces" for 1-20 ms. We only accept a new level once it has been
//! seen for `THRESHOLD` consecutive samples. Call `update` at a fixed rate (e.g. every 1 ms).

#![cfg_attr(not(test), no_std)] // no_std for the MCU, but allow `std` in `cargo test` on the host

/// Number of consecutive identical samples needed to accept a change.
pub const THRESHOLD: u8 = 20;

/// Edge reported by [`Debouncer::update`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Event {
    Pressed,
    Released,
}

#[derive(Debug, Clone, Copy)]
pub struct Debouncer {
    stable: bool,  // last accepted (debounced) state; true = pressed
    counter: u8,   // how many samples disagree with `stable`
}

impl Debouncer {
    pub const fn new() -> Self {
        Self { stable: false, counter: 0 }
    }

    /// Debounced state (true = pressed).
    pub fn is_pressed(&self) -> bool {
        self.stable
    }

    /// Feed one raw sample (`true` = pressed). Returns an event only on an accepted edge.
    pub fn update(&mut self, raw_pressed: bool) -> Option<Event> {
        if raw_pressed == self.stable {
            self.counter = 0; // agrees with the accepted state: reset the disagreement counter
            return None;
        }
        self.counter += 1; // disagrees: count how long it persists
        if self.counter >= THRESHOLD {
            self.stable = raw_pressed;
            self.counter = 0;
            Some(if raw_pressed { Event::Pressed } else { Event::Released })
        } else {
            None
        }
    }
}

impl Default for Debouncer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ignores_short_glitch() {
        let mut d = Debouncer::new();
        for _ in 0..(THRESHOLD - 1) {
            assert_eq!(d.update(true), None);
        }
        assert_eq!(d.update(false), None); // glitch ended before threshold
        assert!(!d.is_pressed());
    }

    #[test]
    fn accepts_stable_press_and_release() {
        let mut d = Debouncer::new();
        let mut ev = None;
        for _ in 0..THRESHOLD { ev = d.update(true); }
        assert_eq!(ev, Some(Event::Pressed));
        assert!(d.is_pressed());
        for _ in 0..THRESHOLD { ev = d.update(false); }
        assert_eq!(ev, Some(Event::Released));
    }

    #[test]
    fn bouncing_never_triggers() {
        let mut d = Debouncer::new();
        for i in 0..1000 {
            assert_eq!(d.update(i % 2 == 0), None);
        }
    }
}
