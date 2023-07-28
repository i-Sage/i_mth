//! # Utilities
//!
//! A tool box of utility functions !!!

use crate::constants::G;
/// Returns the acceleration due to gravity of the celestial
/// body with the passed values.
#[inline]
pub fn calc_acc_due_to_grav(mass_of_celestial_body: f64, radius_of_celestial_body: f64) -> f64 {
    (G * mass_of_celestial_body) / radius_of_celestial_body.powf(2.0)
}

/// Calculates the escape velocity of the celestial body with the
/// passed values.\
/// [More Info](https://en.wikipedia.org/wiki/Escape_velocity#:~:text=More%20generally%2C%20escape%20velocity%20is,orbit%20(of%20any%20radius).)
#[inline]
pub fn calc_escape_velocity(mass_of_celestial_body: f64, radius_of_celestial_body: f64) -> f64{
    ((2.0 * G * mass_of_celestial_body) / (radius_of_celestial_body)).sqrt()
}