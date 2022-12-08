use std::ops::{Add, Div, Mul, Sub};

pub fn lerp<T>(a: T, b: T, t: T) -> <T as Add>::Output
where
    T: Copy + Add<T, Output = T> + Sub<T, Output = T> + Mul<T, Output = T>,
{
    a + (b - a) * t
}

pub fn unlerp<T>(a: T, b: T, c: T) -> T
where
    T: Copy + Sub<T, Output = T> + Div<T, Output = T>,
{
    (c - a) / (b - a)
}
