use std::io::Read;
const MOD: u64 = 1_000_000_007;
// 明示的にstackを使う。

fn main() {
  let mut s = String::new();
  std::io::stdin().read_to_string(&mut s).unwrap();
  let mut itr = s.trim().split_whitespace();
  let n: usize = itr.next().unwrap().parse().unwrap();

  let mut g = vec![std::collections::BTreeSet::new(); n];
  for _ in 1..n {
    let a: usize = itr.next().unwrap().parse::<usize>().unwrap() - 1;
    let b: usize = itr.next().unwrap().parse::<usize>().unwrap() - 1;
    g[a].insert(b);
    g[b].insert(a);
  }

  let mut pre_order = vec![];
  let mut stack = vec![(0, 0)];
  while let Some((v, p)) = stack.pop() {
    pre_order.push(v);
    g[v].remove(&p);
    for &nv in g[v].iter() {
      stack.push((nv, v));
    }
  }

  let mut dp = vec![(1, 1); n];
  for &parent in pre_order.iter().rev() {
    for &child in g[parent].iter() {
      dp[parent].0 = dp[parent].0 * (dp[child].0 + dp[child].1) % MOD;
      dp[parent].1 = dp[parent].1 * dp[child].0 % MOD;
    }
  }

  println!("{}", (dp[0].0 + dp[0].1) % MOD);
}
