use std::io::*;
// hは最終的に0未満になってる可能性があることに注意!

fn main() {
    let mut s: String = String::new();
    std::io::stdin().read_to_string(&mut s).ok();
    let mut itr = s.trim().split_whitespace();
    let n: usize = itr.next().unwrap().parse().unwrap();
    let mut h: i64 = itr.next().unwrap().parse().unwrap();

    let mut a: Vec<i64> = vec![0; n];
    let mut b: Vec<i64> = vec![0; n];
    for i in 0..n {
        a[i] = itr.next().unwrap().parse().unwrap();
        b[i] = itr.next().unwrap().parse().unwrap();
    }
    a.sort();
    a.reverse();
    b.sort();
    b.reverse();

    let mut idx = 0u64;
    while idx < n as u64 && h > 0 && b[idx as usize] > a[0] {
        h -= b[idx as usize];
        idx += 1;
    }
    if h > 0 {
        idx += ((h + a[0] - 1) / a[0]) as u64
    }
    println!("{}", idx);
}
