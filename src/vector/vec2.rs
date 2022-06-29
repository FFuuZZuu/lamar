use num::Num;
use std::{
    fmt::Display,
    ops::{Add, Div, Mul, Sub},
};

/// A generic 2D Vector implementation.
/// Takes 2 generic numbers (both must be same type).
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Vec2<T>
where
    T: Num + Copy,
{
    pub x: T,
    pub y: T,
}

impl<T> Vec2<T>
where
    T: Num + Copy,
{
    /// Create a 2D Vector with the given XY values
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }

    /// Return the dot product of two 2D Vectors
    ///
    /// `a.x * b.x + a.y + b.y`
    pub fn dot(&self, rhs: &Vec2<T>) -> T {
        self.x * rhs.x + self.y * rhs.y
    }

    /// Return the cross product of two 2D Vectors
    ///
    /// `a.x * b.y - a.y * b.x`
    pub fn cross(&self, rhs: &Vec2<T>) -> T {
        self.x * rhs.y - self.y * rhs.x
    }

    // TODO: Swizzle?
}

/// Allows for the following syntax:
/// ```rust
/// # use lamar::vector::Vec2;
/// let a = Vec2::new(10, 12);
/// let b = Vec2::new(16, 48);
/// let c = a + b;
/// // c = Vec2 { a.x + b.x, a.y + b.y }
/// ```
impl<T> Add for Vec2<T>
where
    T: Num + Copy,
{
    type Output = Vec2<T>;

    fn add(self, other: Vec2<T>) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

/// Allows for the following syntax:
/// ```rust
/// # use lamar::vector::Vec2;
/// let a = Vec2::new(10, 12);
/// let b = 10;
/// let c = a + b;
/// // c = Vec2 { 20, 22 }
/// ```
impl<T> Add<T> for Vec2<T>
where
    T: Num + Copy,
{
    type Output = Vec2<T>;

    fn add(self, other: T) -> Self::Output {
        Self {
            x: self.x + other,
            y: self.y + other,
        }
    }
}

/// Allows for the following syntax:
/// ```rust
/// # use lamar::vector::Vec2;
/// let a = Vec2::new(10, 12);
/// let b = Vec2::new(16, 48);
/// let c = a - b;
/// // c = Vec2 { a.x - b.x, a.y - b.y }
/// ```
impl<T> Sub for Vec2<T>
where
    T: Num + Copy,
{
    type Output = Vec2<T>;

    fn sub(self, other: Vec2<T>) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

/// Allows for the following syntax:
/// ```rust
/// # use lamar::vector::Vec2;
/// let a = Vec2::new(10, 12);
/// let b = 10;
/// let c = a - b;
/// // c = Vec2 { 0, 2 }
/// ```
impl<T> Sub<T> for Vec2<T>
where
    T: Num + Copy,
{
    type Output = Vec2<T>;

    fn sub(self, other: T) -> Self::Output {
        Self {
            x: self.x - other,
            y: self.y - other,
        }
    }
}

/// Allows for the following syntax:
/// ```rust
/// # use lamar::vector::Vec2;
/// let a = Vec2::new(10, 12);
/// let b = Vec2::new(16, 48);
/// let c = a * b; // the same as 'a.cross(&b)'
/// ```
impl<T> Mul for Vec2<T>
where
    T: Num + Copy,
{
    type Output = T;

    fn mul(self, other: Vec2<T>) -> Self::Output {
        self.cross(&other)
    }
}

/// Allows for the following syntax:
/// ```rust
/// # use lamar::vector::Vec2;
/// let a = Vec2::new(10, 12);
/// let b = 10;
/// let c = a * b;
/// // c = Vec2 { 100, 120 }
/// ```
impl<T> Mul<T> for Vec2<T>
where
    T: Num + Copy,
{
    type Output = Vec2<T>;

    fn mul(self, other: T) -> Self::Output {
        Self {
            x: self.x * other,
            y: self.y * other,
        }
    }
}

/// Allows for the following syntax:
/// ```rust
/// # use lamar::vector::Vec2;
/// let a = Vec2::new(10, 12);
/// let b = 2;
/// let c = a / b;
/// // c = Vec2 { 5, 6 }
/// ```
impl<T> Div<T> for Vec2<T>
where
    T: Num + Copy,
{
    type Output = Vec2<T>;

    fn div(self, other: T) -> Self::Output {
        Self {
            x: self.x / other,
            y: self.y / other,
        }
    }
}

impl<T> Display for Vec2<T>
where
    T: Display + Num + Copy,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "x: {}, y: {}", self.x, self.y)
    }
}

mod test {
    #[test]
    fn dot_product_test() {
        use super::Vec2;
        let lhs = Vec2::new(32, 64);
        let rhs = Vec2::new(12, 10);

        assert_eq!(lhs.dot(&rhs), 1024);
    }

    #[test]
    fn cross_product_test() {
        use super::Vec2;
        let lhs = Vec2::new(32, 64);
        let rhs = Vec2::new(12, 10);

        assert_eq!(lhs.cross(&rhs), -448);
    }

    #[test]
    fn cross_product_mul_trait_test() {
        use super::Vec2;
        let lhs = Vec2::new(32, 64);
        let rhs = Vec2::new(12, 10);

        assert_eq!(lhs * rhs, -448);
    }

    #[test]
    fn add_test() {
        use super::Vec2;
        let lhs = Vec2::new(32, 64);
        let rhs = Vec2::new(12, 10);

        assert_eq!(lhs + rhs, Vec2::new(44, 74));
    }

    #[test]
    fn sub_test() {
        use super::Vec2;
        let lhs = Vec2::new(32, 64);
        let rhs = Vec2::new(12, 10);

        assert_eq!(lhs - rhs, Vec2::new(20, 54));
    }

    #[test]
    fn add_scalar_test() {
        use super::Vec2;
        let lhs = Vec2::new(32, 64);
        let rhs = 10;

        assert_eq!(lhs + rhs, Vec2::new(42, 74));
    }

    #[test]
    fn sub_scalar_test() {
        use super::Vec2;
        let lhs = Vec2::new(32, 64);
        let rhs = 10;

        assert_eq!(lhs - rhs, Vec2::new(22, 54));
    }

    #[test]
    fn mul_scalar_test() {
        use super::Vec2;
        let lhs = Vec2::new(32, 64);
        let rhs = 10;

        assert_eq!(lhs * rhs, Vec2::new(320, 640));
    }

    #[test]
    fn div_scalar_test() {
        use super::Vec2;
        let lhs = Vec2::new(32, 64);
        let rhs = 4;

        assert_eq!(lhs / rhs, Vec2::new(8, 16));
    }
}
