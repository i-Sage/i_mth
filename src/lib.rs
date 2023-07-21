//! # i_mth
//!
//! This is a crate designed for use in easy to complex statics and dynamics problems.
//!
//! The crate is made as lean as possible and method names are directly correlated to what they
//! actually do.
//!
//! All types currently in the crate and features that will be implemented later will all use the
//! f64 data type because well I am a freak of nature and I have accuracy issues.
//!
//!## Usage
//!
//!Add this to `Cargo.toml`
//!
//! ```toml
//! [dependencies]
//! i_mth = "0.0.1"
//! ```
//!
//! ## Current Crate available features
//!
//! 1. Support for both 2D and 3D vectors
//!
//! ### Examples
//!
//! 1. Creating 2D and 3D vectors with i_mth
//!
//! ```rust
//!    use i_mth::vector3d::Vector3D;
//!
//!    fn main() {
//!       // there are several available methods to create vectors
//!        let v3d_0 = Vector3D::new(1.0, 1.0, 1.0);
//!        let v3d_1 = Vector3D::set(1.0);
//!
//!        assert_eq!(v3d_0, v3d_1);
//!    }
//! ```
//!
//! 1. Moment of a force About a **O**: The moment of a force **F** about  **O** can be defined as the vector product (cross product) of **r** and **F**.
//! Where **r** is the position vector between the point of application of the force to the fixed reference point **O**.
//! ie **Moment** = **r x F**
//!
//! ```rust
//!    use i_mth::vector2d::Vector2D;
//!
//!    fn main() {
//!        let f = Vector3D::new(400.0, 693.0, 0.0);
//!        let r = Vector3D::new(-0.2, 0.16, 0.0);
//!
//!        let moment = r.cross(f);
//!        assert_eq!(-202.6, moment.z);
//!    }
//! ```
//!

mod vectors;
pub use vectors::vector2d;
pub use vectors::vector3d;