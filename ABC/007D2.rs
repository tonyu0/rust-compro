use std::io::Read;

fn rec(dp: &mut Vec<Vec<Vec<i64>>>, s: &Vec<usize>, i: usize, j: usize, k: usize) -> i64 {
    let mut res: i64 = 0;
    if dp[i][j][k] != -1 {
        res = dp[i][j][k]
    } else if i == s.len() {
        if k == 1 {
            res = 1;
        }
    } else {
        // 一つまえがギリギリ && 今の数字と同じ→次もギリギリ
        for num in 0..10 {
            if j == 1 && num > s[i] {
                break;
            }
            if j == 1 && num == s[i] {
                if num == 4 || num == 9 {
                    res += rec(dp, &s, i + 1, 1, 1);
                } else {
                    res += rec(dp, &s, i + 1, 1, k);
                }
            } else {
                if num == 4 || num == 9 {
                    res += rec(dp, &s, i + 1, 0, 1);
                } else {
                    res += rec(dp, &s, i + 1, 0, k);
                }
            }
        }
    }
    dp[i][j][k] = res;
    res
}

fn main() {
    let mut s: String = String::new();
    std::io::stdin().read_to_string(&mut s).ok();
    let mut itr = s.trim().split_whitespace();
    let a: Vec<usize> = itr
        .next()
        .unwrap()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .collect();
    let b: Vec<usize> = itr
        .next()
        .unwrap()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .collect();

    let mut dp1: Vec<Vec<Vec<i64>>> = vec![vec![vec![-1; 11]; 2]; 20];
    let mut dp2: Vec<Vec<Vec<i64>>> = vec![vec![vec![-1; 11]; 2]; 20];
    let mut ans = rec(&mut dp1, &b, 0, 1, 0) - rec(&mut dp2, &a, 0, 1, 0);
    if a.contains(&4) || a.contains(&9) {
        ans += 1;
    }
    println!("{}", ans);
}
