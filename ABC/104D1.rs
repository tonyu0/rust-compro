const MOD: u64 = 1_000_000_007;
// (連続しない)部分列の数え上げ
// 典型だけど?を埋める時の処理が慣れないと混乱する

fn main() {
    let mut s: String = String::new();
    std::io::stdin().read_line(&mut s).ok();
    let s: Vec<char> = s.trim().chars().collect();
    let n = s.len();

    // 0: A, 1: A -> B, 2: A -> B -> C
    let mut dp: Vec<u64> = vec![0; 4];
    dp[0] = 1;
    for i in 0..n {
        let mut next = dp.clone();
        next[0] = dp[0];
        match s[i] {
            'A' => {
                next[1] = (dp[0] + dp[1]) % MOD;
            }
            'B' => {
                next[2] = (dp[1] + dp[2]) % MOD;
            }
            'C' => {
                next[3] = (dp[2] + dp[3]) % MOD;
            }
            _ => {
                next[0] = next[0] * 3 % MOD;
                next[1] = (dp[0] + dp[1] * 3) % MOD;
                next[2] = (dp[1] + dp[2] * 3) % MOD;
                next[3] = (dp[2] + dp[3] * 3) % MOD;
            }
        }
        dp = next;
    }
    println!("{}", dp[3]);
}
