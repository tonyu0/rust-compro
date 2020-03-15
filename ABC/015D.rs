use std::io::Read;
// 10倍ぐらい速い人がいたけど、オーダーレベルで早くなる？

fn main() {
  let mut s: String = String::new();
  std::io::stdin().read_to_string(&mut s).ok();
  let mut itr = s.trim().split_whitespace();
  let w: usize = itr.next().unwrap().parse().unwrap();
  let n: usize = itr.next().unwrap().parse().unwrap();
  let m: usize = itr.next().unwrap().parse().unwrap();

  let mut a: Vec<usize> = Vec::new();
  let mut b: Vec<usize> = Vec::new();
  for _ in 0..n {
    let c: usize = itr.next().unwrap().parse().unwrap();
    let d: usize = itr.next().unwrap().parse().unwrap();
    a.push(c);
    b.push(d);
  }

  let mut dp: Vec<Vec<usize>> = vec![vec![0; n + 1]; w + 1];
  for i in 1..n + 1 {
    for j in (a[i - 1]..w + 1).rev() {
      for k in 1..m + 1 {
        dp[j][k] = std::cmp::max(dp[j][k], dp[j - a[i - 1]][k - 1] + b[i - 1]);
        dp[j][k] = std::cmp::max(dp[j][k], dp[j][k]);
      }
    }
  }
  println!("{}", dp[w][m]);
}
