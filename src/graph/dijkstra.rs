use std::io::Read;
const INF: i64 = 1i64 << 60;

fn dijkstra(start: usize, graph: &Vec<Vec<(usize, i64)>>) -> Vec<i64> {
  let mut dist = vec![INF; graph.len()];
  dist[start] = 0;
  let mut bh = std::collections::BinaryHeap::new();
  // (startからその頂点に辿り着くコスト, 頂点番号)を優先度付きキューで管理
  // BinaryHeapは最大値を返すので、最小コストを得るためにコストは負の数で管理する。
  bh.push((0i64, start));
  while let Some((cost, v)) = bh.pop() {
    if dist[v] < -cost {
      continue;
    }
    for e in graph[v].iter() {
      let nv = e.0;
      let nc = e.1;
      if dist[nv] > -cost + nc {
        dist[nv] = -cost + nc;
        bh.push((-dist[nv], nv));
      }
    }
  }
  dist
}

fn main() {
  let mut s = String::new();
  std::io::stdin().read_to_string(&mut s).unwrap();
  let mut itr = s.trim().split_whitespace();
  let n: usize = itr.next().unwrap().parse().unwrap();
  let m: usize = itr.next().unwrap().parse().unwrap();
  let t: i64 = itr.next().unwrap().parse().unwrap();
  let a: Vec<i64> = (0..n)
    .map(|_| itr.next().unwrap().parse().unwrap())
    .collect();

  let mut g: Vec<Vec<(usize, i64)>> = vec![Vec::new(); n];
  let mut rg: Vec<Vec<(usize, i64)>> = vec![Vec::new(); n];
  for _ in 0..m {
    let a: usize = itr.next().unwrap().parse::<usize>().unwrap() - 1;
    let b: usize = itr.next().unwrap().parse::<usize>().unwrap() - 1;
    let c: i64 = itr.next().unwrap().parse().unwrap();
    g[a].push((b, c));
    rg[b].push((a, c));
  }
  let res1: Vec<i64> = dijkstra(0, &g);
  let res2: Vec<i64> = dijkstra(0, &rg);

  let mut ans: i64 = 0;
  for i in 0..n {
    if res1[i] == INF || res2[i] == INF {
      continue;
    }
    ans = std::cmp::max(ans, (t - res1[i] - res2[i]) * a[i]);
  }
  println!("{}", ans);
}
