use std::io::*;
// 部分列の数え上げ
// 累積和の考え方を理解していれば溶ける。

// ある部分列をpで割ったあまりは
// 右端を起点として差分を考えた後に必要な分だけ10でわれば良いが、
// 10と互いに素なら差分のみのpで割ったあまりを見れば良い。
// これは求める値を定式化するとわかる。

fn inv(mut x: usize, m: usize) -> usize {
    let mut e = m - 2;
    let mut res = 1;
    while e > 0 {
        if e & 1 == 1 {
            res = res * x % m;
        }
        x = x * x % m;
        e >>= 1;
    }
    res
}

fn main() {
    let mut s: String = String::new();
    std::io::stdin().read_to_string(&mut s).ok();
    let mut itr = s.trim().split_whitespace();
    let n: usize = itr.next().unwrap().parse().unwrap();
    let p: usize = itr.next().unwrap().parse().unwrap();
    let s: Vec<usize> = itr
        .next()
        .unwrap()
        .chars()
        .map(|c| c as usize - '0' as usize)
        .collect();

    // 2と5は10と互いにそでない。下1桁で判定可能
    let mut ans: usize = 0;
    if p == 2 {
        for i in 0..n {
            if s[i] % 2 == 0 {
                ans += i + 1;
            }
        }
    } else if p == 5 {
        for i in 0..n {
            if s[i] % 5 == 0 {
                ans += i + 1;
            }
        }
    } else {
        let mut memo: Vec<usize> = vec![0; p + 1];
        let mut now = 0;
        let mut ten = 1;
        let teninv = inv(10, p);
        memo[0] = 1;
        for i in 0..n {
            now = (now * 10 + s[i]) % p;
            ten = ten * teninv % p;
            ans += memo[now * ten % p];
            memo[now * ten % p] += 1;
        }
    }
    println!("{}", ans);
}
