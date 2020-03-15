use std::cmp::*;
use std::io::*;
// XORは繰り上がりのない和だと考えると、
// 立てるビットを決めた時のXORの結果その桁のビットが立つ数を集計すればいい。
// どれを立てて最大かは凸凹っぽいのでDPをする。
// k以下という条件があるので、kギリギリかどうかの情報を持たせてDPをする。

fn main() {
    let mut s: String = String::new();
    std::io::stdin().read_to_string(&mut s).ok();
    let mut itr = s.trim().split_whitespace();
    let n: i64 = itr.next().unwrap().parse().unwrap();
    let k: i64 = itr.next().unwrap().parse().unwrap();
    let a: Vec<i64> = (0..n)
        .map(|_| itr.next().unwrap().parse().unwrap())
        .collect();

    let mut bit: Vec<i64> = vec![0; 44];
    for i in a {
        for j in 0..41 {
            bit[j] += i >> (40 - j) & 1;
        }
    }

    let mut dp: Vec<i64> = vec![-1; 2];
    dp[1] = 0;
    for i in 0..41 {
        let mut next: Vec<i64> = dp.clone();
        let mul = 1i64 << (40 - i);
        if dp[0] != -1 {
            next[0] = max(next[0], dp[0] + max(bit[i] * mul, (n - bit[i]) * mul));
        }
        if dp[1] != -1 {
            if k >> (40 - i) & 1 == 1 {
                next[0] = max(next[0], dp[1] + bit[i] * mul);
                next[1] = max(next[1], dp[1] + (n - bit[i]) * mul);
            } else {
                next[1] = max(next[1], dp[1] + bit[i] * mul);
            }
        }
        dp = next;
    }
    println!("{}", max(dp[0], dp[1]));
}
