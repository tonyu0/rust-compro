use std::io::Read;
const INF: u64 = 1u64 << 60;

fn main() {
    let mut s: String = String::new();
    std::io::stdin().read_to_string(&mut s).ok();
    let mut itr = s.trim().split_whitespace();
    let n: usize = itr.next().unwrap().parse().unwrap();
    let m: usize = itr.next().unwrap().parse().unwrap();
    let mut dp: Vec<Vec<u64>> = vec![vec![INF; n]; n];
    for _i in 0..m {
        let a: usize = itr.next().unwrap().parse::<usize>().unwrap() - 1;
        let b: usize = itr.next().unwrap().parse::<usize>().unwrap() - 1;
        let t: u64 = itr.next().unwrap().parse().unwrap();
        dp[a][b] = t;
        dp[b][a] = t;
    }
    for i in 0..n {
        dp[i][i] = 0;
    }

    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                dp[i][j] = std::cmp::min(dp[i][j], dp[i][k] + dp[k][j]);
            }
        }
    }

    let mut ans: u64 = INF;
    for i in 0..n {
        let mut max: u64 = 0;
        for j in 0..n {
            if i == j {
                continue;
            }
            max = std::cmp::max(max, dp[i][j]);
        }
        if max < ans {
            ans = max;
        }
    }
    println!("{}", ans);
}
