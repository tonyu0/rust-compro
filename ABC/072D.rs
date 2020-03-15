use std::io::*;

fn main() {
    let mut s: String = String::new();
    std::io::stdin().read_to_string(&mut s).ok();
    let mut itr = s.trim().split_whitespace();
    let n: usize = itr.next().unwrap().parse().unwrap();
    let a: Vec<usize> = (0..n)
        .map(|_| itr.next().unwrap().parse().unwrap())
        .collect();

    let mut ans = 0;
    let mut i = 0;
    while i < n {
        if i == a[i] {
            ans += 1;
            i += 1;
        }
        i += 1;
    }
    println!("{}", ans);
}
