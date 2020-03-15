use std::io::*;
// XORとは繰り上がりがない和であるため、
// l ~ rのXORが和に等しいという条件には単調性がある。
// なぜなら、l ~ rまでで条件を充せてなかったらl ~ r + 1も充せないので。

fn main() {
    let mut s: String = String::new();
    std::io::stdin().read_to_string(&mut s).ok();
    let mut itr = s.trim().split_whitespace();
    let n: usize = itr.next().unwrap().parse().unwrap();
    let a: Vec<usize> = (0..n)
        .map(|_| itr.next().unwrap().parse().unwrap())
        .collect();

    let mut ans: usize = 0;
    let mut d: usize = 0;
    let mut r: usize = 0;
    for l in 0..n {
        while r < n && d & a[r] == 0 {
            d |= a[r];
            r += 1;
        }
        ans += r - l;
        d ^= a[l];
    }
    println!("{}", ans);
}
