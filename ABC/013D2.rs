use std::io::*;
// 巡回置換バージョン

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

  let mut map: Vec<usize> = (0..n).collect();
  let mut used: Vec<bool> = vec![false; n];
  let mut ans: Vec<usize> = vec![0; n];
  for i in (0..m).rev() {
    map.swap(a[i], a[i] + 1);
  }

  for i in 0..n {
    if !used[i] {
      let mut cycle: Vec<usize> = Vec::new();
      let mut cur = i;
      while !used[cur] {
        used[cur] = true;
        cycle.push(cur);
        cur = map[cur];
      }

      if cycle.is_empty() {
        ans[i] = i;
      } else {
        let num = d % cycle.len();
        for j in 0..cycle.len() {
          ans[cycle[j]] = cycle[(j + num) % cycle.len()];
        }
      }
    }
  }

  for &i in ans.iter() {
    writeln!(out, "{}", i + 1).unwrap();
  }
  stdout().write_all(&out).unwrap();
}
