use std::fmt;
use std::ops::*;
use crate::vector2d::Vector2D;

/// Represents a mathematical vector in 3 Dimensional space.
#[derive(Debug, PartialEq, Clone, Copy, Default)]
pub struct Vector3D {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vector3D {

    /// Returns a new vector with the passed components.\
    /// x is in the direction of the i-unit vector.\
    /// y is in the direction of the j-unit vector.\
    /// z is in the direction of the k-unit vector.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use i_mth::vector3d::Vector3D;
    /// 
    /// let vec3d = Vector3D::new(1.0, 1.0, 1.0);
    /// 
    /// assert_eq!(1.0, vec3d.x);
    /// assert_eq!(1.0, vec3d.y);
    /// assert_eq!(1.0, vec3d.z);
    /// ```
    #[inline]
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    /// Creates a new vector with the x, y, and z components set to the value passed.
    /// 
    /// # Example
    /// ```rust
    /// use i_mth::vector3d::Vector3D;
    /// 
    /// let vec3d = Vector3D::set(1.0);
    /// 
    /// assert_eq!(1.0, vec3d.x);
    /// assert_eq!(1.0, vec3d.y);
    /// assert_eq!(1.0, vec3d.z);
    /// ```
    #[inline]
    pub fn set(value: f64) -> Self {
        Self {
            x: value,
            y: value,
            z: value,
        }
    }
    /// Return the unit vector i == (i + 0j + 0k)
    #[inline]
    pub fn i() -> Self {
        Self { x: 1.0, y: 0.0, z: 0.0 }
    }

    #[inline]
    /// Returns the unit vector j == (0i + j + 0k)
    pub fn j() -> Self {
        Self { x: 0.0, y: 1.0, z: 0.0 }
    }

    /// Returns the unit vector k == (0i + 0j + k)
    #[inline]
    pub fn k() -> Self {
        Self { x: 0.0, y: 0.0, z: 1.0 }
    }

    /// Returns a vector pointing to the origin of the coordinate system
    /// (0i + 0j + 0k)
    #[inline]
    pub fn origin() -> Self {
        Self { x: 0.0, y: 0.0, z: 0.0 }
    }
    
    /// Returns a vector with the selected component set to the passed value,
    /// while other components gets set to zero.
    /// If an invalid component label like "a" is selected, None is returned.
    /// 
    /// Valid component labels are i, j, k or x, y, z
    /// 
    /// # Example
    /// ```rust
    /// use i_mth::vector3d::Vector3D;
    /// 
    /// // "j" can be used instead of y
    /// let acc_due_to_gravity = Vector3D::select("y", -9.81);
    /// assert_eq!(-9.81, acc_due_to_gravity.unwrap().y);

    #[inline]
    pub fn select(comp: &str, value: f64) -> Option<Vector3D> {
        match comp {
            "i" | "x" => Some(Vector3D{x: value, y: 0.0, z: 0.0}),
            "j" | "y" => Some(Vector3D{x: 0.0, y: value, z: 0.0}),
            "k" | "z" => Some(Vector3D{x: 0.0, y: 0.0, z: value}),
                _     => None,
        }
    }
    /// Returns the dot product of this vector and the passed vector
    #[inline]
    pub fn dot(&self, other: Vector3D) -> f64 {
        (self.x * other.x) + (self.y * other.y) + (self.z * other.z) 
    }

    /// Returns the cross product of this vector and the passed vector
    #[inline]
    pub fn cross(&self, other: Vector3D) -> Self {
        Self {
            x: ((self.y * other.z) + (-self.z * other.y)),
            y: ((self.z * other.x) + (-self.x * other.z)),
            z: ((self.x * other.y) + (-self.y * other.x)),
        }
    }

    /// Multiples the x, y, and z components of this vector by the x, y, z components
    /// of the passed vector.
    #[inline]
    pub fn scale_comps_by_comps(&self, other: Vector3D) -> Self {
        Self {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
    }

    /// Returns the triple scalar product of this vector and the passed vectors.
    /// ### NOTE: the arrangement is -> this_vector . (oth_1 x oth_2)
    #[inline]
    pub fn triple_scalar_prod(&self, oth_1: Vector3D, oth_2: Vector3D) -> f64 {
        self.x * ((oth_1.y * oth_2.z) - (oth_1.z * oth_2.y)) +
        self.y * ((oth_1.z * oth_2.x) - (oth_1.x * oth_2.z)) + 
        self.z * ((oth_1.x * oth_2.y) - (oth_1.y * oth_2.x))
    }

    /// Returns the triple vector product of this vector and the passed vectors.
    /// ### NOTE: the arrangement is -> this_vector x (oth_1 x oth_2)
    #[inline]
    pub fn triple_vector_prod(&self, oth_1: Vector3D, oth_2: Vector3D) -> Self {
        let temp = oth_1.cross(oth_2);
        self.cross(temp)
    }

    /// Returns the squared magnitude of this vector.
    #[inline]
    pub fn squared_magnitude(&self) -> f64 {
        (self.x * self.x) + (self.y * self.y) + (self.z * self.z)
    }

    /// Returns the magnitude of this vector.
    #[inline]
    pub fn magnitude(&self) -> f64 {
        self.squared_magnitude().sqrt()
    }

    /// Returns a vector with the absolute values of this vectors components
    #[inline]
    pub fn abs(&self) -> Self {
        Self {
            x: self.x.abs(),
            y: self.y.abs(),
            z: self.z.abs(),
        }
    }

    /// Returns a vector with this vector's components scaled by the passed value
    #[inline]
    pub fn scale(&self, value: f64) -> Self {
        Self {
            x: self.x * value,
            y: self.y * value,
            z: self.z * value,
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
            self.z *= inv_mag;
        }
    }

    /// Returns the normalized(unit) version of this vector if arithmetically possible
    /// else it returns None. This operation can fail if you have a zero vector.
    #[inline]
    pub fn normalized(&self) -> Option<Self> {
        let mag = self.magnitude();
        if mag > 0.0 {
            let inv_mag = 1.0 / mag;
            return Some(Vector3D {
                x: self.x * inv_mag,
                y: self.y * inv_mag,
                z: self.z * inv_mag,
            })
        }
        None
    }

    /// Scales the passed vector by the passed value and performs vector
    /// addition on this vector and the other vector.
    #[inline]
    pub fn add_scaled(self, other: Vector3D, value: f64) -> Self {
        self + other.scale(value)
    }

    /// Scales this vector by the passed value and performs vector
    /// addition on this vector and the passed vector.
    #[inline]
    pub fn scale_add(&self, value: f64, other: Vector3D) -> Vector3D {
        self.scale(value) + other
    }
    
    /// Returns a 2 Dimensional vector by truncating the z component of this vector.
    #[inline]
    pub fn to_2d(&self) -> Vector2D {
        Vector2D {
            x: self.x,
            y: self.y,
        }
    }

    /// Returns true if this vector is equal to the passed vector.
    #[inline]
    pub fn is_equal_to(&self, other: Vector3D) -> bool {
        (self.x == other.x) && (self.y == other.y) && (self.z == other.z)
    }

    /// Returns true if this vector has a greater magnitude(length) than the passed vector. 
    #[inline]
    pub fn is_greater_than(&self, other: Vector3D) -> bool {
        self.squared_magnitude() > other.squared_magnitude()
    }

    /// If the components are matched x to x, y to y, and z to z, the method returns true only
    /// if every component of this vector is greater than every component of the passed vector.
    #[inline]
    pub fn comp_wise_gt(&self, other: Vector3D) -> bool {
        (self.x > other.x) && (self.y > other.y) && (self.z > other.z)
    }

    /// Converts this vector from cartesian to cylindrical components
    #[inline]
    pub fn as_cylindrical(&mut self) {
        self.x = ((self.x * self.x) + (self.y * self.y)).sqrt();
        self.y = (self.y/ self.x).atan();

    }

    /// Converts this vector from cartesian to spherical components
    #[inline]
    pub fn as_spherical(&mut self) {
        self.x = self.magnitude();
        self.y = (self.z / self.magnitude()).acos();
        self.z = (self.y / self.x).atan();

    }
}


impl Add for Vector3D {
    type Output = Self;
    #[inline]
    fn add(self, rhs: Self) -> Self {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }

}

impl AddAssign for Vector3D {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl Sub for Vector3D {
    type Output = Self;
    #[inline]
    fn sub(self, rhs: Self) -> Self {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl SubAssign for Vector3D {
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

impl Mul for Vector3D {
    type Output = Self;
    #[inline]
    fn mul(self, rhs: Self) -> Self {
        Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

impl MulAssign for Vector3D {
    #[inline]
    fn mul_assign(&mut self, rhs: Self) {
        self.x *= rhs.x;
        self.y *= rhs.y;
        self.z *= rhs.z;
    }
}

impl Div for Vector3D {
    type Output = Self;
    #[inline]
    fn div(self, rhs: Self) -> Self {
        Self {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
            z: self.z / rhs.z,
        }
    }
}

impl DivAssign for Vector3D {
    #[inline]
    fn div_assign(&mut self, rhs: Self) {
        self.x /= rhs.x;
        self.y /= rhs.y;
        self.z /= rhs.z;
    }
}


impl Index<usize> for Vector3D {
    type Output = f64;
    fn index(&self, index: usize) -> &f64 {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("Index out of bounds"),
        }
    }
}

impl fmt::Display for Vector3D {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f,"{}i + {}j + {}k", self.x, self.y, self.z)
    }
}

impl fmt::Binary for Vector3D {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let magnitude = self.magnitude();
        let decimals = f.precision().unwrap_or(4);
        let string = format!("{magnitude:.decimals$}");
        f.pad_integral(true, "", &string)
    }
}