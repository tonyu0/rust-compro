use std::io::*;
// 区間[l, r]の和がpの倍数 =
// 区間[l, r]の和をpで割ったあまりが0 =
// sum[0, r] mod p - sum[0, l-1] mod p == 0

fn main() {
    let mut s: String = String::new();
    std::io::stdin().read_to_string(&mut s).ok();
    let mut itr = s.trim().split_whitespace();
    let n: usize = itr.next().unwrap().parse().unwrap();
    let m: u64 = itr.next().unwrap().parse().unwrap();
    let mut a: Vec<u64> = (0..n)
        .map(|_| itr.next().unwrap().parse().unwrap())
        .collect();

    a[0] %= m;
    for i in 1..n {
        a[i] = (a[i] + a[i - 1]) % m;
    }

    let mut ans = 0u64;
    let mut mp = std::collections::HashMap::new();
    mp.insert(0, 1);
    for i in 0..n {
        let cnt = mp.entry(a[i]).or_insert(0);
        ans += *cnt;
        *cnt += 1;
    }
    println!("{}", ans);
}
