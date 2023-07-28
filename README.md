# i_mth

This is a crate designed for use in easy to complex statics and dynamics problems.

The crate is made as lean as possible and method names are directly correlated to what they
actually do.

All types currently in the crate and features that will be implemented later will all use the
`f64` data type because well I am a freak of nature and I have accuracy issues.

## Usage

Add this to `Cargo.toml`

```toml
[dependencies]
i_mth = "0.1.0"
```

## Current Crate available features

1. Support for both 2D and 3D vectors.
1. Common Constants.
1. A utils module for calculating escape velocity and acceleration of gravity of other celestial bodies. (other functions will be added in the future).

## Currently in the working

1. Support For matrices, operations on matrices and transformations
1. Numerical Calculus

### Examples

1. Creating 2D and 3D vectors with i_mth

```rust
    use i_mth::vector3d::Vector3D;

    fn main() {
        // there are several available methods to create vectors
        let v3d_0 = Vector3D::new(1.0, 1.0, 1.0);
        let v3d_1 = Vector3D::set(1.0);

        assert_eq!(v3d_0, v3d_1);
    }
```

2. Moment of a force About a **O**: The moment of a force **F** about  **O** can be defined as the vector product (cross product) of **r** and **F**.
Where **r** is the position vector between the point of application of the force to the fixed reference point **O**.
ie **Moment** = **r x F**

```rust
    use i_mth::vector3d::Vector3D;

    fn main() {
        let f = Vector3D::new(400.0, 693.0, 0.0);
        let r = Vector3D::new(-0.2, 0.16, 0.0);

        let moment = r.cross(f);
        assert_eq!(-202.6, moment.z);
    }
```

3. Using the utils to calculate [escape velocity](https://en.wikipedia.org/wiki/Escape_velocity#:~:text=More%20generally%2C%20escape%20velocity%20is,orbit%20(of%20any%20radius).): Escape velocity or escape speed is the minimum speed needed for a free, non-propelled object to escape from the gravitational influence of a primary body, thus reaching an infinite distance from it.

```rust
    use i_mth::utils::calc_escape_velocity;

    fn main() {
        let mass_of_moon = 7.342e22;
        let radius_of_moon = 1737.4e3;

        let escape_vel_of_moon = calc_escape_velocity(mass_of_moon, radius_of_moon);

        println!("{} km/s", escape_vel_of_moon / 1000.0);
    }
```

### Supports fmt

```rust
    use i_mth::vector3d::Vector3D;

    fn main() {
        let unit_i = Vector3D::i();
        let unit_j = Vector3D::j();

        let unit_k = unit_i.cross(unit_j);

        println!("{}", unit_k);
    }
```
