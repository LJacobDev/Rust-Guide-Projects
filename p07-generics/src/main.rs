
fn main() {

    println!("{}", solve(3.0,4.0));
    
}


fn solve(a: f64, b: f64) -> f64 {
    (a.powi(2) + b.powi(2)).sqrt()
}