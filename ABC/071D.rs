use std::io::*;
// 初期状態を決めて遷移させると速く溶けるよ。

fn main() {
    let mut s: String = String::new();
    std::io::stdin().read_to_string(&mut s).ok();
    let mut itr = s.trim().split_whitespace();
    let n: usize = itr.next().unwrap().parse().unwrap();
    let s: Vec<char> = itr.next().unwrap().chars().collect();
    let t: Vec<char> = itr.next().unwrap().chars().collect();

    let mut op = Vec::new();
    const MOD: u64 = 1_000_000_007;

    let mut i = 0;
    while i < n {
        if s[i] != t[i] {
            op.push(0);
            i += 1;
        } else {
            op.push(1);
        }
        i += 1;
    }

    let mut ans = 0u64;
    if op[0] == 0 {
        ans += 6;
    } else {
        ans += 3;
    }
    for i in 1..op.len() {
        if op[i - 1] == 0 {
            if op[i] == 0 {
                ans = ans * 3 % MOD;
            }
        } else {
            ans = ans * 2 % MOD;
        }
    }
    println!("{}", ans);
}
