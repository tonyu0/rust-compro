use std::io::*;
// 半分に分けて考える。再帰構造に慣れているかどうか

fn rec(i: usize, x: u64, size: &Vec<u64>, pat: &Vec<u64>) -> u64 {
    if x == 0 {
        return 0;
    }
    if i == 0 {
        return 1;
    }
    if x < (size[i] + 1) / 2 {
        return rec(i - 1, x - 1, &size, &pat);
    }
    pat[i - 1] + 1 + rec(i - 1, x - (size[i] + 1) / 2, &size, &pat)
}

fn main() {
    let mut s: String = String::new();
    std::io::stdin().read_to_string(&mut s).ok();
    let mut itr = s.trim().split_whitespace();
    let n: usize = itr.next().unwrap().parse().unwrap();
    let x: u64 = itr.next().unwrap().parse().unwrap();

    let mut size: Vec<u64> = vec![0; n + 1];
    let mut pat: Vec<u64> = vec![0; n + 1];
    size[0] = 1;
    pat[0] = 1;
    for i in 0..n {
        size[i + 1] = size[i] * 2 + 3;
        pat[i + 1] = pat[i] * 2 + 1;
    }
    println!("{}", rec(n, x, &size, &pat));
}
