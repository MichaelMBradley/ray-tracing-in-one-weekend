use num_traits::Float;
use std::ops::{
    Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub, SubAssign,
};

#[derive(Debug, Copy, Clone)]
pub struct Vec3<T: Float> {
    x: T,
    y: T,
    z: T,
}

impl<T: Float> Vec3<T> {
    /// Creates a new `Vec3` with the given `x`, `y`, and `z` values.
    pub fn new(x: T, y: T, z: T) -> Self {
        Self { x, y, z }
    }

    /// Creates a new `vec3` with identical values in each dimension.
    pub fn same(x: T) -> Self {
        Self { x, y: x, z: x }
    }

    /// Returns the length of the `Vec3` squared.
    /// Useful for comparing relative lengths without incurring the cost of a `sqrt`.
    pub fn length_squared(&self) -> T {
        self.dot(self)
    }

    /// Returns the distance from the origin to the vector.
    pub fn length(&self) -> T {
        self.length_squared().sqrt()
    }

    /// Creates a new, normalised `Vec3`.
    pub fn get_normalised(&self) -> Self {
        *self / self.length()
    }

    /// Normalises this `Vec3` in place.
    /// Returns itself.
    pub fn normalise(&mut self) -> &mut Self {
        *self /= self.length();
        self
    }

    /// The dot product of this and another `Vec3`.
    pub fn dot(&self, other: &Self) -> T {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    /// The cross product of this and another `Vec3`.
    pub fn cross(&self, other: &Self) -> Self {
        Self {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    /// The `x` value of the vector.
    /// Equivalent to `this[0]`.
    pub fn x(&self) -> T {
        self.x
    }

    /// The `y` value of the vector.
    /// Equivalent to `this[1]`.
    pub fn y(&self) -> T {
        self.y
    }

    /// The `z` value of the vector.
    /// Equivalent to `this[2]`.
    pub fn z(&self) -> T {
        self.z
    }
}

impl<T: Float> Add<Vec3<T>> for Vec3<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl<T: Float> AddAssign<Vec3<T>> for Vec3<T> {
    fn add_assign(&mut self, rhs: Self) {
        self.x = self.x + rhs.x;
        self.y = self.y + rhs.y;
        self.z = self.z + rhs.z;
    }
}

impl<T: Float> Neg for Vec3<T> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl<T: Float> Sub<Vec3<T>> for Vec3<T> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self + -rhs
    }
}

impl<T: Float> SubAssign<Vec3<T>> for Vec3<T> {
    fn sub_assign(&mut self, rhs: Self) {
        *self += -rhs
    }
}

impl<T: Float> Mul<T> for Vec3<T> {
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl<T: Float> MulAssign<T> for Vec3<T> {
    fn mul_assign(&mut self, rhs: T) {
        self.x = self.x * rhs;
        self.y = self.y * rhs;
        self.z = self.z * rhs;
    }
}

impl<T: Float> Div<T> for Vec3<T> {
    type Output = Self;

    fn div(self, rhs: T) -> Self::Output {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl<T: Float> DivAssign<T> for Vec3<T> {
    fn div_assign(&mut self, rhs: T) {
        self.x = self.x / rhs;
        self.y = self.y / rhs;
        self.z = self.z / rhs;
    }
}

impl<T: Float> Index<i8> for Vec3<T> {
    type Output = T;

    fn index(&self, index: i8) -> &Self::Output {
        match index {
            0 | -3 => &self.x,
            1 | -2 => &self.y,
            2 | -1 => &self.z,
            _ => panic!(),
        }
    }
}

impl<T: Float> IndexMut<i8> for Vec3<T> {
    fn index_mut(&mut self, index: i8) -> &mut Self::Output {
        match index {
            0 | -3 => &mut self.x,
            1 | -2 => &mut self.y,
            2 | -1 => &mut self.z,
            _ => panic!(),
        }
    }
}
