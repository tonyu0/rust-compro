use std::io::Read;

fn main() {
    let mut s: String = String::new();
    std::io::stdin().read_to_string(&mut s).ok();
    let mut itr = s.trim().split_whitespace();
    let a: f64 = itr.next().unwrap().parse().unwrap();
    let b: f64 = itr.next().unwrap().parse().unwrap();
    let c: f64 = itr.next().unwrap().parse().unwrap();

    let check = |x: f64| a * x + b * ((std::f64::consts::PI * x * c).sin()) >= 100.0;
    let mut ng: f64 = 0.0;
    let mut ok: f64 = 1000.0;
    for _ in 0..100 {
        let mid = (ng + ok) / 2.0;
        if check(mid) {
            ok = mid;
        } else {
            ng = mid;
        }
    }
    println!("{:.10}", ok);
}
