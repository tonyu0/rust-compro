use std::io::Read;
// 桁DP

fn calc(mut n: u64) -> u64 {
  let mut a: Vec<u64> = Vec::new();
  while n > 0 {
    a.push(n % 10);
    n /= 10;
  }
  a.reverse();

  let mut dp1: Vec<Vec<u64>> = vec![vec![0; 2]; 20];
  let mut dp2: Vec<Vec<u64>> = vec![vec![0; 2]; 20];
  dp2[0][0] = 1;
  for i in 0..a.len() {
    for j in 0..10 {
      for k in 0..2 {
        dp1[i + 1][k | (j == 4 || j == 9) as usize] += dp1[i][k];
        if a[i] == j {
          dp2[i + 1][k | (j == 4 || j == 9) as usize] += dp2[i][k];
        } else if a[i] > j {
          dp1[i + 1][k | (j == 4 || j == 9) as usize] += dp2[i][k];
        }
      }
    }
  }
  dp1[a.len()][1] + dp2[a.len()][1]
}

fn main() {
  let mut s: String = String::new();
  std::io::stdin().read_to_string(&mut s).ok();
  let mut itr = s.trim().split_whitespace();
  let a: u64 = itr.next().unwrap().parse().unwrap();
  let b: u64 = itr.next().unwrap().parse().unwrap();

  println!("{}", calc(b) - calc(a - 1));
}
