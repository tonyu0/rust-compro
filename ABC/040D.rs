use std::io::*;

mod disjoint_set {
    pub struct DisjointSet {
        parent: Vec<usize>,
        size: Vec<usize>,
    }
    impl DisjointSet {
        pub fn new(n: usize) -> DisjointSet {
            DisjointSet {
                parent: (0..n).collect(),
                size: vec![1; n],
            }
        }
        fn root(&mut self, x: usize) -> usize {
            if x == self.parent[x] {
                x
            } else {
                let tmp = self.parent[x];
                self.parent[x] = self.root(tmp);
                self.parent[x]
            }
        }
        pub fn merge(&mut self, x: usize, y: usize) {
            let x = self.root(x);
            let y = self.root(y);
            if x == y {
                return;
            }
            if self.size[x] > self.size[y] {
                self.size[x] += self.size[y];
                self.parent[y] = x;
            } else {
                self.size[y] += self.size[x];
                self.parent[x] = y;
            }
        }
        pub fn size(&mut self, x: usize) -> usize {
            let x = self.root(x);
            self.size[x]
        }
    }
}

fn main() {
    let mut s: String = String::new();
    std::io::stdin().read_to_string(&mut s).ok();
    let mut itr = s.trim().split_whitespace();
    let n = itr.next().unwrap().parse::<usize>().unwrap();
    let m = itr.next().unwrap().parse::<usize>().unwrap();
    let mut road: Vec<(usize, usize, usize)> = Vec::new();
    for _ in 0..m {
        let a = itr.next().unwrap().parse::<usize>().unwrap() - 1;
        let b = itr.next().unwrap().parse::<usize>().unwrap() - 1;
        let y = itr.next().unwrap().parse::<usize>().unwrap();
        road.push((y, a, b));
    }
    road.sort_by(|a, b| b.0.cmp(&a.0));
    let q: usize = itr.next().unwrap().parse().unwrap();
    let mut query: Vec<(usize, usize, usize)> = Vec::new();
    for i in 0..q {
        let a = itr.next().unwrap().parse::<usize>().unwrap() - 1;
        let b = itr.next().unwrap().parse::<usize>().unwrap();
        query.push((b, a, i));
    }
    query.sort_by(|a, b| b.0.cmp(&a.0));
    let mut ds = disjoint_set::DisjointSet::new(n + 1);

    let mut road_idx: usize = 0;
    let mut ans: Vec<usize> = vec![0; q];
    for i in 0..q {
        while road_idx < m && road[road_idx].0 > query[i].0 {
            ds.merge(road[road_idx].1, road[road_idx].2);
            road_idx += 1;
        }
        ans[query[i].2] = ds.size(query[i].1);
    }

    let mut out = Vec::new();
    for i in 0..q {
        writeln!(out, "{}", ans[i]).unwrap();
    }
    stdout().write_all(&out).unwrap();
}
