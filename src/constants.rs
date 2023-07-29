//! # CONSTANTS
//!
//! Stores common mathematical and physical constants as well as
//!
//! Again each constant is stored as an f64 type and uncertainty of these
//! constants are specified in the links.
//!
//! [Sources for mathematical constants](https://en.wikipedia.org/wiki/List_of_mathematical_constants)
//! [Sources for physical constants](https://en.wikipedia.org/wiki/List_of_physical_constants)
//! [More Sources](https://cosmologist.info/teaching/Cosmology/Physical_constants.pdf)


/// **Acceleration due to gravity**\
/// unit: ms<sup>-2</sup>
/// value: -9.806_65
pub const EARTH_GRAVITY: f64 = -9.806_65;

/// **Mass of planet Earth**
/// unit: kg
/// value: 5.972_168e24
/// [More Info](https://en.wikipedia.org/wiki/Earth)
pub const EARTH_MASS: f64 = 5.972_168e24;

/// **Radius of planet Earth**
/// unit: m
/// value: 6_371e3
/// [More Info](https://en.wikipedia.org/wiki/Earth)
pub const EARTH_RADIUS: f64 = 6_371e3;

/// **Newtonian gravitational constant**\
/// unit: m<sup>3</sup>kg<sup>-1</sup>s<sup>-2<sup>\
/// value: 6.674_30e-11
/// [More Info](https://en.wikipedia.org/wiki/Gravitational_constant)
pub const G: f64 = 6.674_30e-11;

/// **The constant PI**\
/// unit: dimensionless\
/// value: 3.14159_26535_89793
/// [More Info](https://en.wikipedia.org/wiki/Pi)
pub const PI: f64 = 3.14159_26535_89793;

/// **The constant Tau**
/// value: 6.28318_53071_79586
/// [More Info](https://tauday.com/tau-manifesto)
pub const TAU: f64 = 6.28318_53071_79586;

/// **The Speed of light in a vacuum**\
/// unit: ms<sup>-1</sup>\
/// value: 299_792_458
/// [More Info](https://en.wikipedia.org/wiki/Speed_of_light)
pub const C: usize = 299_792_458;

/// **Euler's Number**\
/// value: 2.718_281_828_459_045
/// [More Info](https://en.wikipedia.org/wiki/E_(mathematical_constant))
pub const E: f64 = 2.718_281_828_459_045;

/// **Vacuum Permeability**\
/// unit: m.kg.s<sup>-2</sup>A<sup>-2</sup>\
/// value: 1.256_637_06e-6
/// [More Info](https://en.wikipedia.org/wiki/Vacuum_permeability)
pub const VACUUM_PERMEABILITY: f64 = 1.256_637_06e-6;

/// **Vacuum Permittivity**\
/// unit: Fm<sup>-1</sup>\
/// value: 8.854_187_812_8e-12
/// [More Info](https://en.wikipedia.org/wiki/Vacuum_permittivity)
pub const VACUUM_PERMITTIVITY: f64 = 8.854_187_812_8e-12;