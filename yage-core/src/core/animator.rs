use crate::{Animate, Update};

///
/// Base component for animations.
///
/// An animator counts from 0.0 to 1.0 in a given amount of time.
/// The duration specifies the number of seconds between 0.0 and 1.0.
/// If bouncing, the animator counts back to 0.0 after reaching 1.0.
/// If looped, the animator starts again at the beginning after the
/// animation would have otherwise finished.
///
/// For creating specific animations with different data types
/// and value ranges, see [`Animation`].
///
/// [`Animation`]: struct.Animation.html
///
pub struct Animator {
    value: f64, // Current value ([0..1])
    duration: f64, // Number of seconds between 0.0 and 1.0
    looped: bool, // Is the animation looped?
    bouncing: bool, // Will the animation go back and forth?
    running: bool, // Current status
    back: bool // Is the animation on its way back? (in case of bouncing)
}

impl Animator {
    ///
    /// Create animator.
    ///
    /// # Returns
    /// A new instance of Animator.
    ///
    pub fn new() -> Self {
        Self {
            value: 0.0,
            duration: 1.0,
            looped: false,
            bouncing: false,
            running: false,
            back: false
        }
    }

    ///
    /// Get current value.
    ///
    /// # Returns
    /// Value ([0.0 .. 1.0])
    ///
    pub fn get_value(&self) -> f64 {
        self.value
    }
}

impl Animate for Animator {
    fn get_duration(&self) -> f64 {
        self.duration
    }

    fn set_duration(&mut self, duration: f64) {
        // Do not accept empty or negative duration
        if duration > 0.0 {
            self.duration = duration;
        }
    }

    fn is_looped(&self) -> bool {
        self.looped
    }

    fn set_looped(&mut self, looped: bool) {
        self.looped = looped;
    }

    fn is_bouncing(&self) -> bool {
        self.bouncing
    }

    fn set_bouncing(&mut self, bouncing: bool) {
        self.bouncing = bouncing;
    }

    fn is_running(&self) -> bool {
        self.running
    }

    fn has_finished(&self) -> bool {
        !self.running && (
            (!self.bouncing && self.value == 1.0) ||
            (self.bouncing && self.value == 0.0)
        )
    }

    fn start(&mut self) {
        self.running = true;
    }

    fn stop(&mut self) {
        self.running = false;
    }

    fn reset(&mut self) {
        self.value = 0.0;
        self.back = false;
    }
}

impl Update for Animator {
    fn needs_update(&self) -> bool {
        self.running
    }

    fn update(&mut self, time_delta: f64) {
        // Check if animation is still active
        if !self.running {
            return;
        }

        // Calculate delta
        let delta = time_delta / self.duration;

        // Forward animation
        if !self.back {
            // Apply delta
            self.value += delta;

            // Reached 1.0?
            if self.value >= 1.0 {
                // Get remainder
                let remainder = self.value - 1.0;

                // Bouncing: begin back journey
                if self.bouncing {
                    self.value = 1.0 - remainder;
                    self.back = true;
                }

                // Looped: start again at the beginning
                else if self.looped {
                    self.value = remainder;
                }

                // Neither: stop animation
                else {
                    self.value = 1.0;
                    self.running = false;
                }
            }
        }

        // Backward animation
        else {
            // Apply delta
            self.value -= delta;

            // Reached 0.0?
            if self.value <= 0.0 {
                // Stop backward animation
                self.back = false;

                // Get remainder
                let remainder = -self.value;

                // Looped: start again at the beginning
                if self.looped {
                    self.value = remainder;
                }

                // Not looped: stop animation
                else {
                    self.value = 0.0;
                    self.running = false;
                }
            }
        }
    }
}
