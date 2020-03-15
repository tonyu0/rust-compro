use std::io::*;
// ダブリングバージョン

fn main() {
  let mut s: String = String::new();
  std::io::stdin().read_to_string(&mut s).ok();
  let mut itr = s.trim().split_whitespace();
  let n: usize = itr.next().unwrap().parse().unwrap();
  let m: usize = itr.next().unwrap().parse().unwrap();
  let d: usize = itr.next().unwrap().parse().unwrap();
  let a: Vec<usize> = (0..m)
    .map(|_| itr.next().unwrap().parse::<usize>().unwrap() - 1)
    .collect();
  let mut out = Vec::new();

  let mut map: Vec<Vec<usize>> = vec![vec![0; n]; 33];
  map[0] = (0..n).collect();
  for i in (0..m).rev() {
    map[0].swap(a[i], a[i] + 1);
  }
  for i in 1..32 {
    for j in 0..n {
      map[i][j] = map[i - 1][map[i - 1][j]];
    }
  }

  for i in 0..n {
    let mut num = i;
    for j in 0..32 {
      if d >> j & 1 == 1 {
        num = map[j][num];
      }
    }
    writeln!(out, "{}", num + 1).unwrap();
  }
  stdout().write_all(&out).unwrap();
}
