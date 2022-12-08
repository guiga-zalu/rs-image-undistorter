use image_undistorter::{get_distorted_environment, Point, Points};

fn main() {
    println!("Hello, world!");

    type Q = f32;
    let list_bottom: Points<Q> = vec![
        Point::new(0.1, 0.1),
        Point::new(0.8, 0.15),
        Point::new(0.35, 0.2),
        Point::new(0.6, 0.18),
    ];
    let list_top: Points<Q> = vec![
        Point::new(0.1, 0.9),
        Point::new(0.7, 0.85),
        Point::new(0.45, 0.85),
        Point::new(0.9, 0.88),
    ];
    let list_left: Points<Q> = vec![
        Point::new(0.1, 0.1),
        Point::new(0.12, 0.5),
        Point::new(0.1, 0.9),
    ];
    let list_right: Points<Q> = vec![
        Point::new(0.8, 0.15),
        Point::new(0.9, 0.88),
        Point::new(0.8, 0.5),
    ];

    let envi = get_distorted_environment(&list_bottom, &list_top, &list_left, &list_right);
    let _0_5 = envi.from_distorted(0.5, 0.5);
    println!("({}, {})", _0_5.x, _0_5.y);
    let p_q = _0_5 + Point::<Q>::new(0.01, 0.01);
    let mut old_p_x = _0_5;
    for i in 1..20 {
        let iters = i;
        let p_x = envi.to_distorted(_0_5, p_q, Some(iters as u32));
        let delta = p_x - old_p_x;
        println!(
            "{} = {}\t\t({}, {})\t\t({}, {})\t\t({}, {})",
            i,
            iters,
            r(p_x.x, 3),
            r(p_x.y, 3),
            r(delta.x, 4),
            r(delta.y, 4),
            r(8192.0 * delta.x, 1),
            r(8192.0 * delta.y, 1)
        );
        old_p_x = p_x;
    }
}

fn r(x: f32, n: u8) -> f32 {
    let p = 10u32.pow(n.into()) as f32;
    (x * p).round() / p
}
