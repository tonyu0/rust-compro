use std::io::Read;
// dinic
// capacity: 隣接行列で辺の容量を管理
// edges: 隣接リストで辺を管理
// start: 各頂点で、何番目までの辺を調べたかのメモ (dfs開始ごとに初期化)
// level: bfsの結果、dfs時に参照する
struct MaximumFlow {
    size: usize,
    capacity: Vec<Vec<i64>>,
    edges: Vec<Vec<usize>>,
    start: Vec<usize>,
    level: Vec<i64>,
}

impl MaximumFlow {
    fn new(n: usize) -> MaximumFlow {
        MaximumFlow {
            size: n,
            capacity: vec![vec![0; n]; n],
            edges: vec![Vec::new(); n],
            start: vec![0; n],
            level: vec![0; n],
        }
    }

    fn add_edge(&mut self, src: usize, dst: usize, cap: i64, directed: bool) {
        self.capacity[src][dst] = cap;
        if directed {
            self.capacity[dst][src] = 0;
        } else {
            self.capacity[dst][src] = cap;
        }
        self.edges[src].push(dst);
        self.edges[dst].push(src);
    }

    fn bfs(&mut self, s: usize, t: usize) -> bool {
        self.level = vec![-1; self.size];
        let mut que = std::collections::VecDeque::new();
        self.level[s] = 0;
        que.push_back(s);
        while let Some(v) = que.pop_front() {
            for &nv in self.edges[v].iter() {
                if self.capacity[v][nv] > 0 && self.level[nv] < 0 {
                    self.level[nv] = self.level[v] + 1;
                    que.push_back(nv);
                }
            }
        }
        self.level[t] != -1
    }

    fn dfs(&mut self, v: usize, s: usize, mut limit: i64) -> i64 {
        if v == s {
            return limit;
        }
        let mut res = 0i64;
        while self.start[v] < self.edges[v].len() {
            let nv: usize = self.edges[v][self.start[v]];
            if self.capacity[nv][v] > 0 && self.level[v] == self.level[nv] + 1 {
                let tmp = std::cmp::min(limit, self.capacity[nv][v]);
                let d = self.dfs(nv, s, tmp);
                if d > 0 {
                    self.capacity[nv][v] -= d;
                    self.capacity[v][nv] += d;
                    res += d;
                    limit -= d;
                    if limit == 0 {
                        break;
                    }
                }
            }
            self.start[v] += 1;
        }
        res
    }

    fn calc(&mut self, s: usize, t: usize) -> i64 {
        let mut flow = 0i64;
        while self.bfs(s, t) {
            self.start = vec![0; self.size];
            flow += self.dfs(t, s, i64::max_value() / 2);
        }
        flow
    }
}

fn main() {
    let mut s: String = String::new();
    std::io::stdin().read_to_string(&mut s).ok();
    let mut itr = s.trim().split_whitespace();
    let n: usize = itr.next().unwrap().parse().unwrap();
    let g: usize = itr.next().unwrap().parse().unwrap();
    let e: usize = itr.next().unwrap().parse().unwrap();
    let p: Vec<usize> = (0..g)
        .map(|_| itr.next().unwrap().parse().unwrap())
        .collect();

    let mut mf = MaximumFlow::new(n + 1);
    let s: usize = 0;
    let t: usize = n;
    for i in 0..g {
        mf.add_edge(t, p[i], 1, false);
    }
    for _ in 0..e {
        let a: usize = itr.next().unwrap().parse().unwrap();
        let b: usize = itr.next().unwrap().parse().unwrap();
        mf.add_edge(a, b, 1, false);
    }
    println!("{}", mf.calc(s, t));
}
