///
/// Represents a simulation component that can be updated regularly.
///
/// Simulation means updating or advancing the simulation. For this, [`update()`]
/// is called with the time delta that has elapsed since the last simulation step.
/// To trigger a subsequent simulation update, [`needs_update()`] should return `true`.
///
/// [`update()`]: trait.Update.html#tymethod.update
/// [`needs_update()`]: trait.Update.html#tymethod.needs_update
///
pub trait Update {
    ///
    /// Check if a simulation update is needed
    ///
    /// # Returns
    /// true if an update is requested, else false
    ///
    fn needs_update(&self) -> bool;

    ///
    /// Update simulation
    ///
    /// # Parameters
    /// - `time_data`: Time delta (in seconds)
    ///
    fn update(&mut self, time_delta: f64);
}
