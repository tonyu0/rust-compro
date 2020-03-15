use std::io::Read;
// 桁DP解

fn main() {
  let mut s: String = String::new();
  std::io::stdin().read_to_string(&mut s).ok();
  let mut itr = s.trim().split_whitespace();
  let s: Vec<u32> = itr
    .next()
    .unwrap()
    .chars()
    .map(|c| c.to_digit(10).unwrap())
    .collect();

  // 1: loose, 2: tight
  let mut dp1: Vec<Vec<u32>> = vec![vec![0; 11]; 11];
  let mut dp2: Vec<Vec<u32>> = vec![vec![0; 11]; 11];
  dp2[0][0] = 1;
  for i in 0..s.len() {
    for j in 0..10 {
      for k in 0..10 {
        dp1[i + 1][k + (j == 1) as usize] += dp1[i][k];
        if s[i] == j {
          dp2[i + 1][k + (j == 1) as usize] += dp2[i][k];
        } else if s[i] > j {
          dp1[i + 1][k + (j == 1) as usize] += dp2[i][k];
        }
      }
    }
  }
  let mut ans: u32 = 0;
  for i in 0..10 {
    ans += (dp1[s.len()][i] + dp2[s.len()][i]) * i as u32;
  }
  println!("{}", ans);
}
