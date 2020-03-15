use std::io::Read;
// O(n^2)だが一応通る、普通のdp

fn main() {
  let mut s: String = String::new();
  std::io::stdin().read_to_string(&mut s).ok();
  let mut itr = s.trim().split_whitespace();
  let n: usize = itr.next().unwrap().parse().unwrap();
  let a: Vec<usize> = (0..n)
    .map(|_| itr.next().unwrap().parse().unwrap())
    .collect();

  // 最後にiを使ったときのLISの最大値
  let mut dp: Vec<usize> = vec![0; n + 1];
  for i in 0..n {
    for j in 0..n + 1 {
      if a[i] > j {
        dp[a[i]] = std::cmp::max(dp[a[i]], dp[j] + 1);
      }
    }
  }
  println!("{}", n - dp.iter().max().unwrap());
}
