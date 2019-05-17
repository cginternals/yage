use crate::Update;

///
/// Represents an animation.
///
pub trait Animate : Update {
    ///
    /// Get duration for which the animation is running
    ///
    /// # Returns
    /// Duration (in seconds)
    ///
    fn get_duration(&self) -> f64;

    ///
    /// Set duration for which the animation is running
    ///
    /// # Parameters
    /// - `duration`: Duration (in seconds)
    ///
    fn set_duration(&mut self, duration: f64);

    ///
    /// Check whether the animation shall be looped
    ///
    /// # Returns
    /// true if the animation is looped, else false
    ///
    fn is_looped(&self) -> bool;

    ///
    /// Set whether the animation shall be looped
    ///
    /// # Parameters
    /// - `looped`: true if the animation is looped, else false
    ///
    fn set_looped(&mut self, looped: bool);

    ///
    /// Check whether the animation shall be bouncing back
    ///
    /// # Returns
    /// true if the animation is bouncing, else false
    ///
    fn is_bouncing(&self) -> bool;

    ///
    /// Set whether the animation shall be bouncing back
    ///
    /// # Parameters
    /// - `bouncing`: true if the animation is bouncing, else false
    ///
    fn set_bouncing(&mut self, bouncing: bool);

    ///
    /// Check if the animation is running
    ///
    /// # Returns
    /// true if running, else false
    ///
    fn is_running(&self) -> bool;

    ///
    /// Check if the animation has finished
    ///
    /// # Returns
    /// true if finished, else false
    ///
    fn has_finished(&self) -> bool;

    ///
    /// Start or resume the animation
    ///
    fn start(&mut self);

    ///
    /// Stop or pause the animation
    ///
    fn stop(&mut self);

    ///
    /// Reset the animation
    ///
    fn reset(&mut self);
}
