use crate::point::Point;
use num_traits::{One, Zero};
use std::ops::{Add, Div, Mul, Neg, Sub};

pub struct Matrix2x2<T>
where
    T: Copy,
{
    pub a: T,
    pub b: T,
    pub c: T,
    pub d: T,
}

impl<T: Copy> Matrix2x2<T> {
    pub fn new(a: T, b: T, c: T, d: T) -> Self {
        Self { a, b, c, d }
    }
    pub fn extract(&self) -> [T; 4] {
        [self.a, self.b, self.c, self.d]
    }
}

impl<T: Copy + Sub<Output = T> + Mul<Output = T>> Matrix2x2<T> {
    pub fn det(&self) -> T {
        self.a * self.d - self.b * self.c
    }
}

impl<
        T: Copy
            + PartialEq
            + Zero<Output = T>
            + One<Output = T>
            + Neg<Output = T>
            + Sub<T, Output = T>
            + Mul<T, Output = T>
            + Div<T, Output = T>,
    > Matrix2x2<T>
{
    pub fn invert(self) -> Option<Self> {
        let det = self.det();
        if det.is_zero() {
            return None;
        }
        let one: T = One::one();
        let q = one / det;
        Some(Self::new(self.d * q, -self.b * q, -self.c * q, self.a * q))
    }
}

impl<T: Copy + Add<T, Output = T>> Add for Matrix2x2<T> {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self::new(
            self.a + other.a,
            self.b + other.b,
            self.c + other.c,
            self.d + other.d,
        )
    }
}

impl<T: Copy + Zero + One + Sub<T, Output = T> + Div<T, Output = T>> One for Matrix2x2<T> {
    fn one() -> Self {
        let one: T = One::one();
        let zero: T = Zero::zero();
        Self::new(one, zero, zero, one)
    }
}

impl<T: Copy + PartialEq + Zero + Sub<T, Output = T> + Mul<Output = T>> Zero for Matrix2x2<T> {
    fn zero() -> Self {
        let zero: T = Zero::zero();
        Self::new(zero, zero, zero, zero)
    }
    fn is_zero(&self) -> bool {
        self.det().is_zero()
    }
}

impl<T: Copy + Add<T, Output = T> + Mul<T, Output = T>> Mul for Matrix2x2<T> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        // [[a b] [c d]] [[e f] [g h]]
        // [[(ae + bg) (af + bh)] [(ce + df) (cg + dh)]]
        let [a, b, c, d] = self.extract();
        let [e, f, g, h] = rhs.extract();
        Self::new(a * e + b * g, a * f + b * h, c * e + d * f, c * g + d * h)
    }
}

impl<T: Copy + Add<T, Output = T> + Mul<T, Output = T>> Mul<Point<T>> for Matrix2x2<T> {
    type Output = Point<T>;
    fn mul(self, rhs: Point<T>) -> Point<T> {
        // [[a b] [c d]] [[e f] [g h]]
        // [[(ae + bg) (af + bh)] [(ce + df) (cg + dh)]]
        let [a, b, c, d] = self.extract();
        let [x, y] = rhs.extract();
        Point::new(a * x + b * y, c * x + d * y)
    }
}

impl<
        T: Copy
            + PartialEq
            + Zero<Output = T>
            + One<Output = T>
            + Neg<Output = T>
            + Sub<T, Output = T>
            + Mul<T, Output = T>
            + Div<T, Output = T>,
    > Div for Matrix2x2<T>
{
    type Output = Option<Matrix2x2<T>>;

    fn div(self, rhs: Self) -> Self::Output {
        if let Some(div) = rhs.invert() {
            Some(self * div)
        } else {
            None
        }
    }
}

impl<T: Copy> Clone for Matrix2x2<T> {
    fn clone(&self) -> Self {
        Matrix2x2::new(self.a, self.b, self.c, self.d)
    }
}

impl<T: Copy> Copy for Matrix2x2<T> {}
