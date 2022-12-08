use num_traits::Zero;
use std::ops::{Add, Mul, Sub};

/// #[derive(Debug, Copy, Clone, PartialEq)]
pub struct Point<T>
where
    T: Copy,
{
    pub x: T,
    pub y: T,
}

pub type Points<T> = Vec<Point<T>>;

impl<T: Copy> Point<T> {
    pub fn new(x: T, y: T) -> Point<T> {
        Point { x, y }
    }
    pub fn extract(self: Self) -> [T; 2] {
        [self.x, self.y]
    }
}

impl<T: Copy> Clone for Point<T> {
    fn clone(&self) -> Self {
        Point::new(self.x, self.y)
    }
}

impl<T: Copy> Copy for Point<T> {}

impl<T: Copy + Add<T, Output = T>> Add for Point<T> {
    type Output = Point<T>;
    fn add(self, rhs: Self) -> Self::Output {
        Point::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl<T: Copy + Sub<T, Output = T>> Sub for Point<T> {
    type Output = Point<T>;
    fn sub(self, rhs: Self) -> Self::Output {
        Point::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl<T: Copy + Mul<T, Output = T>> Mul<T> for Point<T> {
    type Output = Point<T>;
    fn mul(self, rhs: T) -> Self::Output {
        Point::new(self.x * rhs, self.y * rhs)
    }
}

impl<T: Copy + Zero> Zero for Point<T> {
    fn zero() -> Self {
        let zero: T = Zero::zero();
        Point::new(zero, zero)
    }
    fn is_zero(&self) -> bool {
        self.x.is_zero() && self.y.is_zero()
    }
}
