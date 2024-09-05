use num_traits::{Float, ToPrimitive};

fn main() {
    println!("{}", solve(3.0, 4.0));

    let a: f32 = 3.0;
    let b: f32 = 4.0;

    println!("{}", solve_flexible(a, b));

    let c: f64 = 3.0;
    let d: f64 = 4.0;

    println!("{}", solve_flexible(c, d));

    let e: f32 = 3.0;
    let f: f64 = 4.0;

    println!("{}", solve_flexible_mixed_float(e, f));

    let g: i32 = 3;
    let h: f64 = 4.0;

    println!("{}", solve_any_number_type(g, h));
}

fn solve(a: f64, b: f64) -> f64 {
    (a.powi(2) + b.powi(2)).sqrt()
}

///make another version that can take f32 or f64
///but for this one, both arguments have to be the same type
fn solve_flexible<T: Float>(a: T, b: T) -> f64 {
    let a_f64 = a.to_f64().unwrap();
    let b_f64 = b.to_f64().unwrap();

    (a_f64.powi(2) + b_f64.powi(2)).sqrt()
}

///make another version that can take a mixed combination of f32 and f64
fn solve_flexible_mixed_float<T: Float, U: Float>(a: T, b: U) -> f64 {
    let a_f64 = a.to_f64().unwrap();
    let b_f64 = b.to_f64().unwrap();

    (a_f64.powi(2) + b_f64.powi(2)).sqrt()
}

///make another version that can take any number using num_traits::ToPrimitive trait bound
fn solve_any_number_type<T: ToPrimitive, U: ToPrimitive>(a: T, b: U) -> f64 {
    let a_f64 = a.to_f64().unwrap();
    let b_f64 = b.to_f64().unwrap();

    (a_f64.powi(2) + b_f64.powi(2)).sqrt()
}
