use std::io::*;
// -2進数: かなり悩んだがn進数の定義通りにやれば良い。

fn main() {
    let mut s: String = String::new();
    std::io::stdin().read_to_string(&mut s).ok();
    let mut itr = s.trim().split_whitespace();
    let mut n: i64 = itr.next().unwrap().parse().unwrap();
    let mut out = Vec::new();
    if n == 0 {
        println!("0");
    } else {
        while n != 0 {
            let mut r = n % (-2);
            r *= r;
            write!(out, "{}", r).ok();
            n = (n - r) / (-2);
        }
        out.reverse();
        stdout().write_all(&out).unwrap();
    }
}
