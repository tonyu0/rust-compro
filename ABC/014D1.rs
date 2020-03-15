use std::io::*;

//
mod tree {
    use std::ops::*;
    pub struct Tree<T> {
        tree: Vec<Vec<(usize, T)>>,
        depth: Vec<usize>,
        dist: Vec<T>,
        parent: Vec<Vec<usize>>,
        logn: usize,
    }
    impl<T: Copy + Clone + Default + Add<Output = T> + Sub<Output = T>> Tree<T> {
        // calc depth, cost of each vertices
        pub fn new(n: usize, tree: &Vec<Vec<(usize, T)>>) -> Tree<T> {
            let mut logn = 0;
            while 1 << logn < n {
                logn += 1;
            }

            let parent: Vec<Vec<usize>> = vec![vec![n; n]; logn];

            let mut t = Tree {
                tree: tree.clone(),
                depth: vec![0; n],
                dist: vec![Default::default(); n],
                parent: parent,
                logn: logn,
            };
            t.init(0, 0, 0, Default::default());

            // doubling parents
            for i in 0..logn - 1 {
                for j in 0..n {
                    t.parent[i + 1][j] = t.parent[i][t.parent[i][j]];
                }
            }
            t
        }

        fn init(&mut self, v: usize, p: usize, d: usize, c: T) {
            self.parent[0][v] = p;
            self.depth[v] = d;
            self.dist[v] = c;

            for &(nv, cost) in self.tree[v].clone().iter() {
                if p != nv {
                    self.init(nv, v, d + 1, c + cost);
                }
            }
        }

        fn find_lca(&self, mut x: usize, mut y: usize) -> usize {
            // follow parents of y
            if self.depth[x] > self.depth[y] {
                x ^= y;
                y ^= x;
                x ^= y;
            }

            for i in (0..self.logn).rev() {
                if (self.depth[y] - self.depth[x]) >> i & 1 == 1 {
                    y = self.parent[i][y];
                }
            }
            if x == y {
                return y;
            }

            for i in (0..self.logn).rev() {
                let px = self.parent[i][x];
                let py = self.parent[i][y];
                if px != py {
                    x = px;
                    y = py;
                }
            }
            self.parent[0][y]
        }

        pub fn dist(&self, x: usize, y: usize) -> T {
            let lca = self.find_lca(x, y);
            self.dist[x] + self.dist[y] - (self.dist[lca] + self.dist[lca])
        }
    }
}

fn main() {
    let mut s: String = String::new();
    std::io::stdin().read_to_string(&mut s).ok();
    let mut itr = s.trim().split_whitespace();
    let n: usize = itr.next().unwrap().parse().unwrap();

    let mut graph: Vec<Vec<(usize, i64)>> = vec![Vec::new(); n];
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
        writeln!(out, "{}", tree.dist(x, y) + 1).ok();
    }
    stdout().write_all(&out).unwrap();
}
