use std::io::Read;

fn check(a: String, b: String) -> String {
  if a == "unko" {
    b
  } else if a.len() > b.len() {
    a
  } else if a.len() < b.len() {
    b
  } else {
    std::cmp::max(a, b)
  }
}

fn main() {
  let mut s: String = String::new();
  std::io::stdin().read_to_string(&mut s).ok();
  let mut itr = s.trim().split_whitespace();
  let n: usize = itr.next().unwrap().parse().unwrap();
  let m: usize = itr.next().unwrap().parse().unwrap();
  let a: Vec<usize> = (0..m)
    .map(|_| itr.next().unwrap().parse().unwrap())
    .collect();
  let need: Vec<usize> = [0, 2, 5, 5, 4, 5, 6, 3, 7, 6].to_vec();
  let mut dp: Vec<String> = vec!["unko".to_string(); n + 9];
  dp[0] = "".to_string();
  for i in 0..n {
    for &j in a.iter() {
      if dp[i] == "unko" {
        continue;
      }
      dp[i + need[j]] = check(dp[i + need[j]].clone(), dp[i].clone() + &j.to_string());
    }
  }
  println!("{}", dp[n]);
}
