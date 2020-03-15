use std::io::*;

fn main() {
    let mut s: String = String::new();
    std::io::stdin().read_to_string(&mut s).ok();
    let mut itr = s.trim().split_whitespace();
    let n: f64 = itr.next().unwrap().parse().unwrap();
    let k: f64 = itr.next().unwrap().parse().unwrap();
    // k1つ、　k2つ、 k3つ
    let one: f64 = 6.0 * (k - 1.0) * (n - k);
    let two: f64 = 3.0 * (n - 1.0);
    let three: f64 = 1.0;

    println!("{:.10}", (one + two + three) / (n * n * n));
}
