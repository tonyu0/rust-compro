use std::io::*;
// 剰余の周期性　・　片方固定
// 割る数 * 商 <= 割られる数

fn main() {
    let mut s: String = String::new();
    std::io::stdin().read_to_string(&mut s).ok();
    let mut itr = s.trim().split_whitespace();
    let n: u64 = itr.next().unwrap().parse().unwrap();
    let k: u64 = itr.next().unwrap().parse().unwrap();
    let mut ans = 0u64;
    for b in k + 1..n + 1 {
        ans += n / b * (b - k);
        if n % b >= k {
            ans += n % b - k + 1;
        }
    }
    if k == 0 {
        ans -= n;
    }
    println!("{}", ans);
}
