use std::io::*;
// F(A, B) = F(0, A − 1) ⊕ F(0, B)
// と考えると楽っぽい。

fn f(x: i64) -> i64 {
    let y = x % 4;
    match y {
        0 => x,
        1 => 1,
        2 => x ^ 1,
        _ => 0,
    }
}

fn main() {
    let mut s: String = String::new();
    std::io::stdin().read_to_string(&mut s).ok();
    let mut itr = s.trim().split_whitespace();
    let a: i64 = itr.next().unwrap().parse().unwrap();
    let b: i64 = itr.next().unwrap().parse().unwrap();

    println!("{}", f(a - 1) ^ f(b));
}
