use num::Num;
use std::{
    fmt::Display,
    ops::{Add, Div, Mul, Sub},
};

/// A generic 4D Vector implementation.
/// Takes 4 generic numbers (all 4 must be same type).
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Vec4<T>
where
    T: Num + Clone + Copy,
{
    pub x: T,
    pub y: T,
    pub z: T,
    pub w: T,
}

impl<T> Vec4<T>
where
    T: Num + Clone + Copy,
{
    /// Create a 4D Vector with the given XYZ values
    pub fn new(x: T, y: T, z: T, w: T) -> Self {
        Self { x, y, z, w }
    }

    /// Return the dot product of two 4D Vectors
    ///
    /// `a.x * b.x + a.y * b.y + a.z * b.z + a.w * b.w`
    pub fn dot(&self, rhs: &Vec4<T>) -> T {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z + self.w * rhs.w
    }

    // 4D Vectors cannot have a cross product
    // https://math.stackexchange.com/questions/2317604/cross-product-of-4d-vectors

    // TODO: Swizzle?
}

impl Vec4<f32> {
    /// Create a 4D vector with all values initialised to 0.0
    pub fn zero() -> Vec4<f32> {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            w: 0.0,
        }
    }
}

/// Allows for the following syntax:
/// ```rust
/// # use lamar::vector::Vec4;
/// let a = Vec4::new(2, 4, 8, 10);
/// let b = Vec4::new(16, 32, 64, 100);
/// let c = a + b;
///
/// assert_eq!(c, Vec4::new(a.x + b.x, a.y + b.y, a.z + b.z, a.w + b.w));
/// ```
impl<T> Add for Vec4<T>
where
    T: Num + Clone + Copy,
{
    type Output = Vec4<T>;

    fn add(self, other: Vec4<T>) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
            w: self.w + other.w,
        }
    }
}

/// Allows for the following syntax:
/// ```rust
/// # use lamar::vector::Vec4;
/// let a = Vec4::new(4, 7, 8, 10);
/// let b = 2;
/// let c = a + b;
///
/// assert_eq!(c, Vec4::new(4 + 2, 7 + 2, 8 + 2, 10 + 2));
/// ```
impl<T> Add<T> for Vec4<T>
where
    T: Num + Clone + Copy,
{
    type Output = Vec4<T>;

    fn add(self, other: T) -> Self::Output {
        Self {
            x: self.x + other,
            y: self.y + other,
            z: self.z + other,
            w: self.w + other,
        }
    }
}

/// Allows for the following syntax:
/// ```rust
/// # use lamar::vector::Vec4;
/// let a = Vec4::new(2, 4, 8, 10);
/// let b = Vec4::new(16, 32, 64, 100);
/// let c = a - b;
///
/// assert_eq!(c, Vec4::new(a.x - b.x, a.y - b.y, a.z - b.z, a.w - b.w));
/// ```
impl<T> Sub for Vec4<T>
where
    T: Num + Clone + Copy,
{
    type Output = Vec4<T>;

    fn sub(self, other: Vec4<T>) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
            w: self.w - other.w,
        }
    }
}

/// Allows for the following syntax:
/// ```rust
/// # use lamar::vector::Vec4;
/// let a = Vec4::new(4, 7, 8, 10);
/// let b = 2;
/// let c = a - b;
///
/// assert_eq!(c, Vec4::new(4 - 2, 7 - 2, 8 - 2, 10 - 2));
/// ```
impl<T> Sub<T> for Vec4<T>
where
    T: Num + Clone + Copy,
{
    type Output = Vec4<T>;

    fn sub(self, other: T) -> Self::Output {
        Self {
            x: self.x - other,
            y: self.y - other,
            z: self.z - other,
            w: self.w - other,
        }
    }
}

/// Allows for the following syntax:
/// ```rust
/// # use lamar::vector::Vec4;
/// let a = Vec4::new(4, 7, 8, 10);
/// let b = 2;
/// let c = a * b;
///
/// assert_eq!(c, Vec4::new(4 * 2, 7 * 2, 8 * 2, 10 * 2));
/// ```
impl<T> Mul<T> for Vec4<T>
where
    T: Num + Clone + Copy,
{
    type Output = Vec4<T>;

    fn mul(self, other: T) -> Self::Output {
        Self {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
            w: self.w * other,
        }
    }
}

/// Allows for the following syntax:
/// ```rust
/// # use lamar::vector::Vec4;
/// let a = Vec4::new(4, 6, 12, 10);
/// let b = 2;
/// let c = a / b;
///
/// assert_eq!(c, Vec4::new(4 / 2, 6 / 2, 12 / 2, 10 / 2));
/// ```
impl<T> Div<T> for Vec4<T>
where
    T: Num + Clone + Copy,
{
    type Output = Vec4<T>;

    fn div(self, other: T) -> Self::Output {
        Self {
            x: self.x / other,
            y: self.y / other,
            z: self.z / other,
            w: self.w / other,
        }
    }
}

impl<T> Display for Vec4<T>
where
    T: Display + Num + Clone + Copy,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "x: {}\ny: {}\nz: {}\nw: {}",
            self.x, self.y, self.z, self.w
        )
    }
}

#[cfg(test)]
mod test {
    use crate::vector::Vec4;

    #[test]
    fn zero_vec3_test() {
        assert_eq!(Vec4::zero(), Vec4::new(0.0, 0.0, 0.0, 0.0));
    }

    #[test]
    fn dot_product_test() {
        let lhs = Vec4::new(2, 4, 8, 10);
        let rhs = Vec4::new(16, 32, 64, 100);

        assert_eq!(lhs.dot(&rhs), 1672);
    }

    #[test]
    fn add_test() {
        let lhs = Vec4::new(2, 4, 8, 10);
        let rhs = Vec4::new(16, 32, 64, 100);

        assert_eq!(lhs + rhs, Vec4::new(18, 36, 72, 110));
    }

    #[test]
    fn sub_test() {
        let lhs = Vec4::new(2, 4, 8, 10);
        let rhs = Vec4::new(16, 32, 64, 100);

        assert_eq!(lhs - rhs, Vec4::new(-14, -28, -56, -90));
    }

    #[test]
    fn add_scalar_test() {
        let lhs = Vec4::new(32, 64, 128, 10);
        let rhs = 10;

        assert_eq!(lhs + rhs, Vec4::new(42, 74, 138, 20));
    }

    #[test]
    fn sub_scalar_test() {
        let lhs = Vec4::new(32, 64, 128, 10);
        let rhs = 10;

        assert_eq!(lhs - rhs, Vec4::new(22, 54, 118, 0));
    }

    #[test]
    fn mul_scalar_test() {
        let lhs = Vec4::new(32, 64, 128, 10);
        let rhs = 10;

        assert_eq!(lhs * rhs, Vec4::new(320, 640, 1280, 100));
    }

    #[test]
    fn div_scalar_test() {
        let lhs = Vec4::new(32, 64, 128, 12);
        let rhs = 4;

        assert_eq!(lhs / rhs, Vec4::new(8, 16, 32, 3));
    }
}
