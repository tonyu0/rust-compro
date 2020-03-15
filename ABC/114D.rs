use std::io::Read;

fn main() {
  let mut s: String = String::new();
  std::io::stdin().read_to_string(&mut s).ok();
  let mut itr = s.trim().split_whitespace();
  let n: usize = itr.next().unwrap().parse().unwrap();

  let mut table: Vec<u64> = vec![0; n + 1];
  for mut i in 2..n + 1 {
    let mut j = 2;
    while j * j <= i {
      while i % j == 0 {
        table[j] += 1;
        i /= j;
      }
      j += 1;
    }
    if i != 1 {
      table[i] += 1;
    }
  }
  let mut cnt75 = 0;
  let mut cnt25 = 0;
  let mut cnt15 = 0;
  let mut cnt5 = 0;
  let mut cnt3 = 0;
  for i in 2..n + 1 {
    if table[i] >= 2 {
      cnt3 += 1;
    }
    if table[i] >= 4 {
      cnt5 += 1;
    }
    if table[i] >= 14 {
      cnt15 += 1;
    }
    if table[i] >= 24 {
      cnt25 += 1;
    }
    if table[i] >= 74 {
      cnt75 += 1;
    }
  }
  let mut ans = cnt75;
  ans += cnt25 * (cnt3 - 1);
  ans += cnt15 * (cnt5 - 1);
  ans += cnt5 * (cnt5 - 1) * (cnt3 - 2) / 2;
  println!("{}", ans);
}
