use crate::{
    Animate, Animator, Update, Interpolate
};

///
/// Generic animation.
///
/// An animation interpolates a value between two given values.
/// The duration specifies the number of seconds between the first
/// and the second value. If bouncing, the animation counts back to
/// the first value after reaching the second. If looped, the animation
/// starts again at the beginning after it would have otherwise finished.
///
/// An Animation can only be used for types that implement [`Interpolate`].
///
/// [`Interpolate`]: trait.Interpolate.html
///
pub struct Animation<T> {
    first: T,
    second: T,
    value: T,
    animator: Animator
}

impl<T: Interpolate<T> + Copy> Animation<T> {
    ///
    /// Create animation.
    ///
    /// # Parameters
    /// - `first`: First value
    /// - `second`: Second value
    /// - `duration`: Duration (in seconds)
    /// - `looped`: true if the animation is looped, else false
    /// - `bouncing`: true if the animation is bouncing, else false
    /// - `start`: true to start the animation right away, else false
    ///
    /// # Returns
    /// A new instance of Animation.
    ///
    pub fn new(first: T, second: T, duration: f64, looped: bool, bouncing: bool, start: bool) -> Self {
        let mut animator = Animator::new();
        animator.set_duration(duration);
        animator.set_looped(looped);
        animator.set_bouncing(bouncing);
        if start { animator.start(); }

        Self {
            first,
            second,
            value: first,
            animator
        }
    }

    ///
    /// Create animation from one value to another.
    ///
    /// # Parameters
    /// - `first`: First value
    /// - `second`: Second value
    /// - `duration`: Duration (in seconds)
    ///
    /// # Returns
    /// A new instance of Animation.
    ///
    pub fn from_to(first: T, second: T, duration: f64) -> Self {
        let mut animator = Animator::new();
        animator.set_duration(duration);

        Self {
            first,
            second,
            value: first,
            animator
        }
    }

    ///
    /// Get first value.
    ///
    /// # Returns
    /// First value.
    ///
    pub fn first_value(&self) -> T {
        self.first
    }

    ///
    /// Get second value.
    ///
    /// # Returns
    /// Second value.
    ///
    pub fn second_value(&self) -> T {
        self.second
    }

    ///
    /// Get current value.
    ///
    /// # Returns
    /// Current value.
    ///
    pub fn get_value(&self) -> T {
        self.value
    }
}

impl<T: Interpolate<T> + Copy> Animate for Animation<T> {
    fn get_duration(&self) -> f64 {
        self.animator.get_duration()
    }

    fn set_duration(&mut self, duration: f64) {
        self.animator.set_duration(duration);
    }

    fn is_looped(&self) -> bool {
        self.animator.is_looped()
    }

    fn set_looped(&mut self, looped: bool) {
        self.animator.set_looped(looped);
    }

    fn is_bouncing(&self) -> bool {
        self.animator.is_bouncing()
    }

    fn set_bouncing(&mut self, bouncing: bool) {
        self.animator.set_bouncing(bouncing);
    }

    fn is_running(&self) -> bool {
        self.animator.is_running()
    }

    fn has_finished(&self) -> bool {
        self.animator.has_finished()
    }

    fn start(&mut self) {
        self.animator.start();
    }

    fn stop(&mut self) {
        self.animator.stop();
    }

    fn reset(&mut self) {
        self.animator.reset();
    }
}

impl<T: Interpolate<T> + Copy> Update for Animation<T> {
    fn needs_update(&self) -> bool {
        self.animator.needs_update()
    }

    fn update(&mut self, time_delta: f64) {
        self.animator.update(time_delta);
        self.value = <T>::interpolate(self.first, self.second, self.animator.get_value() as f32);
    }
}
