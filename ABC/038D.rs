use std::io::*;
// wを負にしてhで昇順ソートすることでwのみのLISに帰着させるのがうまいですね。

mod fenwick_tree {
    use std::ops::*;
    pub struct FenwickTree<T, F> {
        data: Vec<T>,
        identity: T,
        operation: F,
    }
    impl<T: Copy + Clone, F: Fn(T, T) -> T> FenwickTree<T, F> {
        pub fn new(size: usize, id: T, op: F) -> FenwickTree<T, F> {
            FenwickTree {
                data: vec![id; size + 1],
                identity: id,
                operation: op,
            }
        }
        pub fn query(&self, i: usize) -> T {
            let mut res = self.identity;
            let mut idx = i;
            while idx > 0 {
                res = (self.operation)(res, self.data[idx]);
                idx -= ((idx as isize) & -(idx as isize)) as usize;
            }
            res
        }
        pub fn update(&mut self, i: usize, x: T) {
            let mut idx = i;
            while idx < self.data.len() {
                self.data[idx] = (self.operation)(self.data[idx], x);
                idx += ((idx as isize) & -(idx as isize)) as usize;
            }
        }
    }
}

fn main() {
    let mut s: String = String::new();
    std::io::stdin().read_to_string(&mut s).ok();
    let mut itr = s.trim().split_whitespace();
    let n: usize = itr.next().unwrap().parse().unwrap();
    let mut a: Vec<(i32, i32)> = (0..n)
        .map(|_| {
            (
                itr.next().unwrap().parse().unwrap(),
                -itr.next().unwrap().parse::<i32>().unwrap(),
            )
        })
        .collect();
    a.sort();
    let mut b: Vec<usize> = Vec::new();
    for i in 0..n {
        b.push((-a[i].1) as usize);
    }

    let mut ft = fenwick_tree::FenwickTree::new(100005, 0, std::cmp::max);
    let mut dp: Vec<usize> = vec![0; 100005];
    for i in 0..n {
        dp[b[i]] = ft.query(b[i] - 1) + 1;
        ft.update(b[i], dp[b[i]]);
    }
    println!("{}", ft.query(100005));
}
