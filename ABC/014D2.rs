use std::io::*;

mod tree {
    use std::cmp::*;
    pub struct Tree {
        tree: Vec<Vec<(usize, u64)>>,
        dist_from_root: Vec<u64>,
        size_of_subtree: Vec<usize>,
        vertex_id: Vec<usize>,
        id: usize,
        head: Vec<usize>,
        parent_of: Vec<usize>,
    }
    impl Tree {
        pub fn new(n: usize, tree: &Vec<Vec<(usize, u64)>>) -> Tree {
            let mut t = Tree {
                tree: tree.clone(),
                dist_from_root: vec![0; n],
                size_of_subtree: vec![1; n],
                vertex_id: vec![0; n],
                id: 0,
                head: vec![0; n],
                parent_of: vec![0; n],
            };
            t.init(0, 0, 0);
            t.decompose(0, 0, 0);
            t
        }

        fn init(&mut self, v: usize, p: usize, c: u64) {
            self.parent_of[v] = p;
            self.dist_from_root[v] = c;
            for &(nv, cost) in self.tree[v].clone().iter() {
                if p != nv {
                    self.init(nv, v, c + cost);
                    self.size_of_subtree[v] += self.size_of_subtree[nv];
                }
            }
        }
        fn decompose(&mut self, v: usize, p: usize, h: usize) {
            self.vertex_id[v] = self.id;
            self.head[v] = h;
            self.id += 1;

            let mut heavy = (0, 0);
            for &(nv, _) in self.tree[v].clone().iter() {
                if p != nv {
                    heavy = max(heavy, (self.size_of_subtree[nv], nv));
                }
            }
            if heavy.1 != 0 {
                self.decompose(heavy.1, v, h);
            }
            for &(nv, _) in self.tree[v].clone().iter() {
                if nv != p && nv != heavy.1 {
                    self.decompose(nv, v, nv);
                }
            }
        }

        fn find_lca(&self, mut x: usize, mut y: usize) -> usize {
            loop {
                if self.vertex_id[x] > self.vertex_id[y] {
                    x ^= y;
                    y ^= x;
                    x ^= y;
                }

                if self.head[x] == self.head[y] {
                    return x;
                }
                y = self.parent_of[self.head[y]];
            }
        }
        pub fn distance_between(&self, x: usize, y: usize) -> u64 {
            let lca = self.find_lca(x, y);
            self.dist_from_root[x] + self.dist_from_root[y] - self.dist_from_root[lca] * 2
        }
    }
}

fn main() {
    let mut s: String = String::new();
    std::io::stdin().read_to_string(&mut s).ok();
    let mut itr = s.trim().split_whitespace();
    let n: usize = itr.next().unwrap().parse().unwrap();
    let mut graph: Vec<Vec<(usize, u64)>> = vec![Vec::new(); n];
    for _ in 0..n - 1 {
        let a: usize = itr.next().unwrap().parse::<usize>().unwrap() - 1;
        let b: usize = itr.next().unwrap().parse::<usize>().unwrap() - 1;
        graph[a].push((b, 1));
        graph[b].push((a, 1));
    }
    let tree = tree::Tree::new(n, &graph);
    let q: usize = itr.next().unwrap().parse().unwrap();
    let mut out = Vec::new();
    for _ in 0..q {
        let x: usize = itr.next().unwrap().parse::<usize>().unwrap() - 1;
        let y: usize = itr.next().unwrap().parse::<usize>().unwrap() - 1;
        writeln!(out, "{}", tree.distance_between(x, y) + 1).ok();
    }
    stdout().write_all(&out).unwrap();
}
