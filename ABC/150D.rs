use std::io::*;
// 式変形、こういう問題は紙に書いて実験するに鍵る。

fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        return a;
    }
    gcd(b, a % b)
}
fn lcm(a: u64, b: u64) -> u64 {
    a / gcd(a, b) * b
}
fn gcd_array(a: &Vec<u64>) -> u64 {
    let mut res = a[0];
    for i in 1..a.len() {
        res = gcd(res, a[i]);
    }
    res
}
fn lcm_array(a: &Vec<u64>) -> u64 {
    let mut res = a[0];
    for i in 1..a.len() {
        res = lcm(res, a[i]);
    }
    res
}

fn main() {
    let mut s: String = String::new();
    std::io::stdin().read_to_string(&mut s).ok();
    let mut itr = s.trim().split_whitespace();
    let n: usize = itr.next().unwrap().parse().unwrap();
    let m: u64 = itr.next().unwrap().parse().unwrap();
    let a: Vec<u64> = (0..n)
        .map(|_| itr.next().unwrap().parse().unwrap())
        .collect();
    let l = lcm_array(&a);
    let g = gcd_array(&a);

    for i in 0..n {
        if a[i] / g % 2 == 0 {
            println!("0");
            return;
        }
    }

    // a_k / 2 * (2 * p + 1) = lcm/2 * i (lcm * i <= M)
    // あるkに関して、仮にa_kとlcmの2の約数の数が違うなら、2 * p + 1が奇数であるため、そのようなpは存在しない。
    // 存在する場合、lcm / 2の奇数倍を求めれば良い。

    println!("{}", (m / (l / 2) + 1) / 2);
}
