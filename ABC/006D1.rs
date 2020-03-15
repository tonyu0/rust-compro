use std::io::Read;
// 想定解法？: lower_boundでdpテーブルを更新

fn lower_bound(x: usize, a: &Vec<usize>) -> usize {
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

const INF: usize = 100000;
fn main() {
  let mut s: String = String::new();
  std::io::stdin().read_to_string(&mut s).ok();
  let mut itr = s.trim().split_whitespace();
  let n: usize = itr.next().unwrap().parse().unwrap();
  let a: Vec<usize> = (0..n)
    .map(|_| itr.next().unwrap().parse().unwrap())
    .collect();

  let mut dp: Vec<usize> = vec![INF; n + 1];
  for i in 0..n {
    let idx = lower_bound(a[i], &dp);
    dp[idx] = a[i];
  }
  println!("{}", n - lower_bound(INF, &dp));
}
