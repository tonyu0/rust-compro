use std::cmp::*;
use std::io::*;
// 半分全列挙のスムーズな実装ができない

const INF: u64 = 1 << 60;

fn lower_bound<T: PartialOrd>(x: T, a: &Vec<T>) -> usize {
    let mut l: usize = 0;
    let mut r: usize = a.len();
    while r != l {
        let mid = (l + r) >> 1;
        if a[mid] < x {
            l = mid + 1;
        } else {
            r = mid;
        }
    }
    l
}

fn solve1(n: usize, w: usize, vw: &Vec<Vec<u64>>) {
    let mut set1: Vec<(u64, u64)> = Vec::new();
    let mut set2: Vec<(u64, u64)> = Vec::new();

    let m = n / 2;
    for i in 0..1 << m {
        let mut tv: u64 = 0;
        let mut tw: u64 = 0;
        for j in 0..m {
            if i >> j & 1 == 1 {
                tv += vw[j][0];
                tw += vw[j][1];
            }
        }
        set1.push((tw, tv));
    }
    set1.sort();
    let mut idx = 0;
    set2.push(set1[0]);
    for i in 1..set1.len() {
        if set1[idx].1 < set1[i].1 {
            set2.push(set1[i]);
            idx = i;
        }
    }

    let mut ans: u64 = 0;
    for i in 0..1 << (n - m) {
        let mut tv = 0;
        let mut tw = 0;
        for j in 0..n - m {
            if i >> j & 1 == 1 {
                tv += vw[j + m][0];
                tw += vw[j + m][1];
            }
        }

        if tw <= w as u64 {
            let vidx = lower_bound((w as u64 - tw, INF), &set2) - 1;
            if vidx != set2.len() {
                ans = max(ans, tv + set2[vidx].1);
            }
        }
    }

    println!("{}", ans);
}

fn solve2(n: usize, w: usize, vw: &Vec<Vec<u64>>) {
    let mut dp: Vec<u64> = vec![0; w + 1];
    for i in 0..n {
        for j in (vw[i][1] as usize..w + 1).rev() {
            dp[j] = max(dp[j], dp[j - vw[i][1] as usize] + vw[i][0]);
        }
    }

    println!("{}", dp[w]);
}

fn solve3(n: usize, w: usize, vw: &Vec<Vec<u64>>) {
    const V: usize = 2000;
    let mut dp: Vec<u64> = vec![INF; V * n + 1];
    dp[0] = 0;
    for i in 0..n {
        for j in (vw[i][0] as usize..V * n + 1).rev() {
            dp[j] = min(dp[j], dp[j - vw[i][0] as usize] + vw[i][1]);
        }
    }

    let mut ans = 0;
    for i in 0..V * n + 1 {
        if dp[i] <= w as u64 {
            ans = i;
        }
    }
    println!("{}", ans);
}

fn main() {
    let mut s: String = String::new();
    std::io::stdin().read_to_string(&mut s).ok();
    let mut itr = s.trim().split_whitespace();
    let n: usize = itr.next().unwrap().parse().unwrap();
    let w: usize = itr.next().unwrap().parse().unwrap();
    let vw: Vec<Vec<u64>> = (0..n)
        .map(|_| {
            (0..2)
                .map(|_| itr.next().unwrap().parse().unwrap())
                .collect()
        })
        .collect();
    let mw = vw.iter().fold(0, |m, w| max(m, w[1]));
    if n <= 30 {
        solve1(n, w, &vw);
    } else if mw <= 1000 {
        solve2(n, w, &vw);
    } else {
        solve3(n, w, &vw);
    }
}
