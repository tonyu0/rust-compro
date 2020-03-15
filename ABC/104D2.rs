use std::io::*;
use std::ops::*;
const MOD: u64 = 1_000_000_007;

// TODO: Modint構造体が未完成

#[derive(Debug, Copy, Clone)]
struct Mint {
    value: u64,
}

impl Add for Mint {
    type Output = Mint;
    fn add(self, other: Mint) -> Mint {
        let mut res = self.value + other.value;
        if res >= MOD {
            res -= MOD;
        }
        Mint { value: res }
    }
}
impl AddAssign for Mint {
    fn add_assign(&mut self, other: Mint) {
        let mut res = self.value + other.value;
        if res >= MOD {
            res -= MOD;
        }
        *self = Mint { value: res };
    }
}

impl std::fmt::Display for Mint {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

fn main() {
    let mut s: String = String::new();
    std::io::stdin().read_to_string(&mut s).ok();
    let mut itr = s.trim().split_whitespace();
    let s: Vec<char> = itr.next().unwrap().chars().collect();

    let n = s.len();
    let mut dp: Vec<Vec<Mint>> = vec![vec![Mint { value: 0 }; 4]; n + 1];

    dp[0][0] = Mint { value: 1 };
    for i in 0..n {
        if s[i] == 'A' || s[i] == '?' {
            // use A
            dp[i + 1][1] += dp[i][0];
            // not use
            dp[i + 1][0] += dp[i][0];
            dp[i + 1][1] += dp[i][1];
            dp[i + 1][2] += dp[i][2];
            dp[i + 1][3] += dp[i][3];
        }
        if s[i] == 'B' || s[i] == '?' {
            // use B
            dp[i + 1][2] += dp[i][1];
            // not use
            dp[i + 1][0] += dp[i][0];
            dp[i + 1][1] += dp[i][1];
            dp[i + 1][2] += dp[i][2];
            dp[i + 1][3] += dp[i][3];
        }
        if s[i] == 'C' || s[i] == '?' {
            // use C
            dp[i + 1][3] += dp[i][2];
            // not use
            dp[i + 1][0] += dp[i][0];
            dp[i + 1][1] += dp[i][1];
            dp[i + 1][2] += dp[i][2];
            dp[i + 1][3] += dp[i][3];
        }
    }
    println!("{}", dp[n][3]);
}
