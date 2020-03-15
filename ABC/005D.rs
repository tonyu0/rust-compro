use std::io::Read;
fn main() {
    let mut s: String = String::new();
    std::io::stdin().read_to_string(&mut s).ok();
    let mut itr = s.trim().split_whitespace();
    let n: usize = itr.next().unwrap().parse().unwrap();
    let d: Vec<Vec<i32>> = (0..n)
        .map(|_| {
            (0..n)
                .map(|_| itr.next().unwrap().parse().unwrap())
                .collect()
        })
        .collect();
    let q: usize = itr.next().unwrap().parse().unwrap();
    let p: Vec<usize> = (0..q)
        .map(|_| itr.next().unwrap().parse().unwrap())
        .collect();

    let mut s: Vec<Vec<i32>> = vec![vec![0; 2525]; 2525];
    for i in 0..n {
        for j in 0..n {
            s[i + 1][j + 1] = s[i][j + 1] + d[i][j];
        }
    }
    for i in 0..n {
        for j in 0..n {
            s[i + 1][j + 1] += s[i + 1][j];
        }
    }

    let mut ans: Vec<i32> = vec![0; 2525];
    for i in 0..n {
        for j in i + 1..n + 1 {
            for a in 0..n {
                for b in a + 1..n + 1 {
                    let area = (j - i) * (b - a);
                    ans[area] = std::cmp::max(ans[area], s[j][b] + s[i][a] - s[i][b] - s[j][a]);
                }
            }
        }
    }
    for i in 1..2525 {
        ans[i] = std::cmp::max(ans[i], ans[i - 1]);
    }
    for i in 0..q {
        println!("{}", ans[p[i]]);
    }
}
