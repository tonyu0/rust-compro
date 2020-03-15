use std::cmp::*;
use std::io::*;
// 征討してないけどこういう考えもある。
// xを決める場合、上からi桁目まではkと一致させる。
// 制約上、２進数ではせいぜい40桁なので、確実にkより小さくする桁を全探索できる。
// xがkと完全に一致する場合に注意。

fn main() {
    let mut s: String = String::new();
    std::io::stdin().read_to_string(&mut s).ok();
    let mut itr = s.trim().split_whitespace();
    let n: u64 = itr.next().unwrap().parse().unwrap();
    let k: u64 = itr.next().unwrap().parse().unwrap();
    let a: Vec<u64> = (0..n)
        .map(|_| itr.next().unwrap().parse().unwrap())
        .collect();

    let mut bit: Vec<u64> = vec![0; 44];
    for i in a.iter() {
        for j in 0..41 {
            bit[j] += i >> j & 1;
        }
    }

    let mut ans: u64 = 0;
    for i in (0..41).rev() {
        if k >> i & 1 == 0 {
            continue;
        }

        let mut tmp: u64 = 0;
        for j in (0..41).rev() {
            let mul = 1u64 << j;
            if j > i {
                if k >> j & 1 == 1 {
                    tmp += (n - bit[j]) * mul;
                } else {
                    tmp += bit[j] * mul;
                }
            } else if j == i {
                tmp += bit[j] * mul;
            } else {
                tmp += max(bit[j], n - bit[j]) * mul;
            }
        }
        ans = max(ans, tmp);
    }

    let tmp = a.iter().fold(0, |xor, x| xor ^ x);
    println!("{}", max(ans, tmp));
}
