use std::fmt;
use std::ops::*;
use crate::vector3d::Vector3D;

/// Represents a mathematical vector in 2 dimensional space.
#[derive(Debug, PartialEq, Clone, Copy, Default)]
pub struct Vector2D {
    pub x: f64,
    pub y: f64,
}

impl Vector2D {
    /// Returns a new vector with the components passed.\
    /// x is in the direction of the i-unit vector.\
    /// y is in the direction of the j-unit vector.\
    ///
    /// # Example
    ///
    /// ```rust
    /// use i_mth::vector2d::Vector2D;
    ///
    /// let vec2d = Vector2D::new(1.0, 1.0);
    ///
    /// assert_eq!(1.0, vec2d.x);
    /// assert_eq!(1.0, vec2d.y);
    /// ```
    #[inline]
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    /// Creates a new vector with the x and y components set to the value passed.
    ///
    /// # Example
    /// ```rust
    /// use i_mth::vector2d::Vector2D;
    ///
    /// let vec2d = Vector2D::set(1.0);
    ///
    /// assert_eq!(1.0, vec2d.x);
    /// assert_eq!(1.0, vec2d.y);
    /// ```
    #[inline]
    pub fn set(value: f64) -> Self {
        Self { x: value, y: value }
    }
    /// Return the unit vector i == (i + 0j)
    #[inline]
    pub fn i() -> Self {
        Self { x: 1.0, y: 0.0 }
    }

    /// Returns the unit vector j == (0i + j)
    #[inline]
    pub fn j() -> Self {
        Self { x: 0.0, y: 1.0 }
    }

    /// Returns a vector pointing to the origin of the coordinate system
    /// (0i + 0j)
    #[inline]
    pub fn origin() -> Self {
        Self { x: 0.0, y: 0.0 }
    }

    /// Returns a vector with the selected component set to the passed value,
    /// while other components gets set to zero.
    /// If an invalid component label like "a" is selected, None is returned.
    /// 
    /// Valid component labels are i, j, or x, y
    /// 
    /// # Example
    /// ```rust
    /// use i_mth::vector2d::Vector2D;
    /// 
    /// // "j" can be used instead of y
    /// let acc_due_to_gravity = Vector2D::select("y", -9.81);
    /// assert_eq!(-9.81, acc_due_to_gravity.unwrap().y);
    #[inline]
    pub fn select(comp: &str, value: f64) -> Option<Vector2D> {
        match comp {
            "i" | "x" => Some(Vector2D { x: value, y: 0.0 }),
            "j" | "y" => Some(Vector2D { x: 0.0, y: value }),
            _ => None,
        }
    }

    /// Returns the dot product of this vector and the passed vector
    #[inline]
    pub fn dot(&self, other: Vector2D) -> f64 {
        (self.x * other.x) + (self.y * other.y)
    }

    /// Multiples the x, y, and z components of this vector by the x, y, z components
    /// of the passed vector.
    #[inline]
    pub fn scale_comps_by_comps(&self, other: Vector2D) -> Self {
        Self {
            x: self.x * other.x,
            y: self.y * other.y,
        }
    }

    /// Returns a vector with the absolute values of this vectors components
    #[inline]
    pub fn abs(&self) -> Self {
        Self {
            x: self.x.abs(),
            y: self.y.abs(),
        }
    }

    /// Returns a vector with this vector's components scaled by the passed value
    #[inline]
    pub fn scale(&self, value: f64) -> Self {
        Self {
            x: self.x * value,
            y: self.y * value,
        }
    }

    /// Converts this vector to its unit form if arithmetically possible else
    /// Returns this vector unchanged if it fails. This operation can fail
    /// if you have a zero vector
    #[inline]
    pub fn to_unit(&mut self) {
        let mag = self.magnitude();
        if mag > 0.0 {
            let inv_mag = 1.0 / mag;
            self.x *= inv_mag;
            self.y *= inv_mag;
        }
    }

    /// Returns the normalized(unit) version of this vector if arithmetically possible
    /// else it returns None. This operation can fail if you have a zero vector.
    #[inline]
    pub fn normalized(&self) -> Option<Self> {
        let mag = self.magnitude();
        if mag > 0.0 {
            let inv_mag = 1.0 / mag;
            return Some(Vector2D {
                x: self.x * inv_mag,
                y: self.y * inv_mag,
            });
        }
        None
    }

    /// Scales the passed vector by the passed value and performs vector
    /// addition on this vector and the other vector.
    #[inline]
    pub fn add_scaled(self, other: Vector2D, value: f64) -> Self {
        self + other.scale(value)
    }

    /// Scales this vector by the passed value and performs vector
    /// addition on this vector and the passed vector.
    #[inline]
    pub fn scale_add(&self, value: f64, other: Vector2D) -> Vector2D {
        self.scale(value) + other
    }

    /// Returns true if this vector is equal to the passed vector.
    #[inline]
    pub fn is_equal_to(&self, other: Vector2D) -> bool {
        (self.x == other.x) && (self.y == other.y)
    }

    /// Returns true if this vector has a greater magnitude(length) than the passed vector.
    #[inline]
    pub fn is_greater_than(&self, other: Vector2D) -> bool {
        self.squared_magnitude() > other.squared_magnitude()
    }

    /// If the components are matched x to x, y to y, and z to z, the method returns true only
    /// if every component of this vector is greater than every component of the passed vector.
    #[inline]
    pub fn comp_wise_gt(&self, other: Vector2D) -> bool {
        (self.x > other.x) && (self.y > other.y)
    }

    /// Converts this vector from cartesian to cylindrical components
    #[inline]
    pub fn as_cylindrical(&self) -> Self {
        Self {
            x: ((self.x * self.x) + (self.y * self.y)).sqrt(),
            y: (self.y / self.x).atan(),
        }
    }

    /// Converts this vector to a 3 Dimensional one by the addition of the
    /// z component passed
    pub fn to_3d(&self, z: f64) -> Vector3D {
        Vector3D {
            x: self.x,
            y: self.y,
            z,
        }
    }
    /// Returns the squared magnitude of this vector.
    pub fn squared_magnitude(&self) -> f64 {
        (self.x * self.x) + (self.y * self.y)
    }
    /// Returns the magnitude of this vector.
    #[inline]
    pub fn magnitude(&self) -> f64 {
        self.squared_magnitude().sqrt()
    }
}

impl Add for Vector2D {
    type Output = Self;
    #[inline]
    fn add(self, rhs: Self) -> Self {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl AddAssign for Vector2D {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl Sub for Vector2D {
    type Output = Self;
    #[inline]
    fn sub(self, rhs: Self) -> Self {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl SubAssign for Vector2D {
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl Mul for Vector2D {
    type Output = Self;
    #[inline]
    fn mul(self, rhs: Self) -> Self {
        Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
        }
    }
}

impl MulAssign for Vector2D {
    #[inline]
    fn mul_assign(&mut self, rhs: Self) {
        self.x *= rhs.x;
        self.y *= rhs.y;
    }
}

impl Div for Vector2D {
    type Output = Self;
    #[inline]
    fn div(self, rhs: Self) -> Self {
        Self {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
        }
    }
}

impl DivAssign for Vector2D {
    #[inline]
    fn div_assign(&mut self, rhs: Self) {
        self.x /= rhs.x;
        self.y /= rhs.y;
    }
}

impl fmt::Display for Vector2D {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}i + {}j", self.x, self.y)
    }
}

impl fmt::Binary for Vector2D {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let magnitude = self.magnitude();
        let decimals = f.precision().unwrap_or(4);
        let string = format!("{magnitude:.decimals$}");
        f.pad_integral(true, "", &string)
    }
}
