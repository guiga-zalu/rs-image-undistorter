use crate::matrix::Matrix2x2;
use crate::point::{Point, Points};
use crate::util::{lerp, unlerp};
use num_traits::{One, Zero};
use polynomial::Polynomial;
use std::cmp::PartialOrd;
use std::ops::{Div, Mul, Neg, Sub};
use std::vec::IntoIter;

pub struct Environment<T> {
    side_top: Polynomial<T>,
    side_right: Polynomial<T>,
    side_bottom: Polynomial<T>,
    side_left: Polynomial<T>,
    lagrange_top: Polynomial<T>,
    lagrange_right: Polynomial<T>,
    lagrange_bottom: Polynomial<T>,
    lagrange_left: Polynomial<T>,
    bottom_x_delta: T,
    bottom_x_min: T,
    top_x_delta: T,
    top_x_min: T,
    left_y_delta: T,
    left_y_min: T,
    right_y_delta: T,
    right_y_min: T,
}

impl<T> Environment<T> {
    pub fn from_distorted(self: &Self, x: T, y: T) -> Point<T>
    where
        T: Copy + Clone + Zero + One + Sub<T, Output = T> + Div<T, Output = T>,
    {
        // P = A o (X) * X
        let mx = lerp(
            self.side_left.eval(y * self.left_y_delta + self.left_y_min),
            self.side_right
                .eval(y * self.right_y_delta + self.right_y_min),
            unlerp(self.lagrange_left.eval(y), self.lagrange_right.eval(y), x),
        );
        let my = lerp(
            self.side_bottom
                .eval(x * self.bottom_x_delta + self.bottom_x_min),
            self.side_top.eval(x * self.top_x_delta + self.top_x_min),
            unlerp(self.lagrange_bottom.eval(x), self.lagrange_top.eval(x), y),
        );
        Point { x: mx, y: my }
    }
    pub fn to_distorted(self: &Self, p_p: Point<T>, p_q: Point<T>, iters: Option<u32>) -> Point<T>
    where
        T: Copy
            + PartialEq
            + Zero
            + One
            + Neg<Output = T>
            + Sub<T, Output = T>
            + Div<T, Output = T>,
    {
        /*
		P = AX, X = (1/A) P
        P = A o (X) * X
        X = (1/A) o (X) * P
        X_0 = P
        X_1 = 1 / (A o (P)) * P
        X_2 = 1 / (A o (1 / (A o (X_1)) * P)) * P
        X_n = 1 / (A o X_{n - 1}) * P
         */
        let iters = iters.unwrap_or(10);
        let mut p_x = self.from_distorted(p_p.x, p_p.y);
        let mut p_z = self.from_distorted(p_q.x, p_q.y);

        // X_n = (A o (X_{n - 1}, Z_{n - 1})).invert() * P
        // Z_n = (A o (X_{n - 1}, Z_{n - 1})).invert() * Q
        for _ in 1..iters {
            let a = get_transformation_matrix_from_points(p_p, p_x, p_q, p_z)
                .invert()
                .unwrap();
            p_x = a * p_p;
            p_z = a * p_q;
        }

        p_x
    }
}

fn get_transformation_matrix_from_points<T>(
    p0: Point<T>,
    p1: Point<T>,
    q0: Point<T>,
    q1: Point<T>,
) -> Matrix2x2<T>
where
    T: Copy + PartialEq + Zero + One + Sub<T, Output = T> + Div<T, Output = T>,
{
    /*
    A P0 ~ P1
    A Q0 ~ Q1
    [[a b] [c d]] [p0x p0y] = [p1x p1y]
    [[a b] [c d]] [q0x q0y] = [q1x q1y]
    p1x = a p0x + b p0y
    q1x = a q0x + b q0y
    p1y = c p0x + d p0y
    q1y = c q0x + d q0y */
    let m = Matrix2x2::<T>::new(p0.x, p0.y, q0.x, q0.y);
    let [a, b] = solve_2_variable_eqn_sys(&m, p1.x, q1.x)
        .expect("The matrix' lines are Linearly Dependent\n\t<= P0 = k Q0");
    let [c, d] = solve_2_variable_eqn_sys(&m, p1.y, q1.y)
        .expect("The matrix' lines are Linearly Dependent\n\t<= P0 = k Q0");
    Matrix2x2 { a, b, c, d }
}

fn solve_2_variable_eqn_sys<T>(m: &Matrix2x2<T>, a: T, b: T) -> Result<[T; 2], String>
where
    T: Copy + PartialEq + Zero + Sub<T, Output = T> + Mul<T, Output = T> + Div<T, Output = T>,
{
    /*
    a = ma x + mb y
    b = mc x + md y */
    if m.is_zero() {
        Err("The determinant of the matrix is zero!".to_string())
    } else if m.a.is_zero() {
        /*
        a = mb y
        b = mc x + md y
        y = a / mb
        x = (b - md y) / mc */
        let y = a / m.b;
        let x = (b - m.d * y) / m.c;
        Ok([x, y])
    } else {
        /*
        a / ma		= x		+ mb / ma y
        b - a mc / ma= 0	+ y (md - mc mb / ma)
        y = (b - a mc / ma) / (md - mc mb / ma)
          = (ma b - mc a) / det A
        x = (a - mb y) / ma */
        let y = (m.a * b - m.c * a) / m.det();
        let x = (a - m.b * y) / m.a;
        Ok([x, y])
    }
}

pub fn get_distorted_environment<T>(
    list_bottom: &Points<T>,
    list_top: &Points<T>,
    list_left: &Points<T>,
    list_right: &Points<T>,
) -> Environment<T>
where
    T: PartialOrd
        + Copy
        + Clone
        + One
        + Zero
        + Neg<Output = T>
        + Sub<T, Output = T>
        + Div<T, Output = T>,
{
    let (bottom_x_min, bottom_x_delta, bottom_y_min, bottom_y_delta) =
        get_mins_and_deltas(list_bottom);
    let (top_x_min, top_x_delta, top_y_min, top_y_delta) = get_mins_and_deltas(list_top);
    let (left_x_min, left_x_delta, left_y_min, left_y_delta) = get_mins_and_deltas(list_left);
    let (right_x_min, right_x_delta, right_y_min, right_y_delta) = get_mins_and_deltas(list_right);

    let lagrange_bottom = Polynomial::lagrange(
        get_points_x_coord(list_bottom).as_slice(),
        get_points_y_coord(list_bottom).as_slice(),
    )
    .expect("Erro com a lista de pontos de baixo");
    let lagrange_top = Polynomial::lagrange(
        get_points_x_coord(list_top).as_slice(),
        get_points_y_coord(list_top).as_slice(),
    )
    .expect("Erro com a lista de pontos de cima");

    let lagrange_left = Polynomial::lagrange(
        get_points_y_coord(list_left).as_slice(),
        get_points_x_coord(list_left).as_slice(),
    )
    .expect("Erro com a lista de pontos da esquerda");
    let lagrange_right = Polynomial::lagrange(
        get_points_y_coord(list_right).as_slice(),
        get_points_x_coord(list_right).as_slice(),
    )
    .expect("Erro com a lista de pontos da direita");

    let side_bottom = (lagrange_bottom.clone() - Polynomial::new(vec![bottom_y_min]))
        * Polynomial::new(vec![bottom_y_delta]);

    let _1: T = One::one();
    let side_top = (lagrange_top.clone() - Polynomial::new(vec![top_y_min]))
        * Polynomial::new(vec![top_y_delta])
        + Polynomial::new(vec![_1]);

    let side_left = (lagrange_left.clone() - Polynomial::new(vec![left_x_min]))
        * Polynomial::new(vec![left_x_delta]);

    let side_right = (lagrange_left.clone() - Polynomial::new(vec![right_x_min]))
        * Polynomial::new(vec![right_x_delta])
        + Polynomial::new(vec![(_1 + _1 + _1) / (_1 + _1)]);

    Environment {
        side_top,
        side_right,
        side_bottom,
        side_left,
        lagrange_top,
        lagrange_right,
        lagrange_bottom,
        lagrange_left,
        bottom_x_delta,
        bottom_x_min,
        top_x_delta,
        top_x_min,
        left_y_delta,
        left_y_min,
        right_y_delta,
        right_y_min,
    }
}

fn get_points_x_coord<T: Copy>(points: &Points<T>) -> IntoIter<T> {
    let mut vec: Vec<T> = vec![];
    for point in points {
        vec.push(point.x);
    }
    vec.into_iter()
}

fn get_points_y_coord<T: Copy>(points: &Points<T>) -> IntoIter<T> {
    let mut vec: Vec<T> = vec![];
    for point in points {
        vec.push(point.y);
    }
    vec.into_iter()
}

fn get_mins_and_deltas<T>(list: &Points<T>) -> (T, T, T, T)
where
    T: PartialOrd<T> + Copy + Sub<Output = T>,
{
    let min_x = list
        .iter()
        .reduce(|acc, point| if acc.x < point.x { acc } else { point })
        .unwrap()
        .x;
    let max_x = list
        .iter()
        .reduce(|acc, point| if acc.x > point.x { acc } else { point })
        .unwrap()
        .x;
    let min_y = list
        .iter()
        .reduce(|acc, point| if acc.y < point.y { acc } else { point })
        .unwrap()
        .y;
    let max_y = list
        .iter()
        .reduce(|acc, point| if acc.y > point.y { acc } else { point })
        .unwrap()
        .y;
    (min_x, max_x - min_x, min_y, max_y - min_y)
}
