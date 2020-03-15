use std::io::Read;
const MOD: u64 = 1_000_000_007;
// 頂点に値を持たせるとメモらなくて良い。

fn dfs(v: usize, p: usize, g: &Vec<Vec<usize>>) -> (u64, u64) {
  // res := (頂点vが白の場合, 頂点vが黒の場合)
  let mut res = (1, 1);
  for &nv in g[v].iter() {
    if nv == p {
      continue;
    }
    let tmp = dfs(nv, v, &g);
    // 頂点vが白だったら次は白も黒もありえる
    res.0 = res.0 * (tmp.0 + tmp.1) % MOD;
    res.1 = res.1 * tmp.0 % MOD;
  }
  res
}

fn main() {
  let mut s = String::new();
  std::io::stdin().read_to_string(&mut s).unwrap();
  let mut itr = s.trim().split_whitespace();
  let n: usize = itr.next().unwrap().parse().unwrap();

  let mut g: Vec<Vec<usize>> = vec![Vec::new(); n];
  for _ in 1..n {
    let a: usize = itr.next().unwrap().parse::<usize>().unwrap() - 1;
    let b: usize = itr.next().unwrap().parse::<usize>().unwrap() - 1;
    g[a].push(b);
    g[b].push(a);
  }
  let ans = dfs(0, 0, &g);
  println!("{}", (ans.0 + ans.1) % MOD);
}
