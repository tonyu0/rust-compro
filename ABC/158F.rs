use std::io::*;
const MOD: u64 = 998244353;

mod segment_tree {
    pub struct SegmentTree<T, F> {
        size: usize,
        data: Vec<T>,
        identity: T,
        operation: F,
    }
    impl<T: Copy + Clone, F: Fn(T, T) -> T> SegmentTree<T, F> {
        pub fn new(n: usize, id: T, op: F) -> SegmentTree<T, F> {
            let mut size = 1;
            while size < n {
                size <<= 1;
            }
            SegmentTree {
                size: size,
                data: vec![id; size * 2],
                identity: id,
                operation: op,
            }
        }
        pub fn update(&mut self, mut i: usize, x: T) {
            i += self.size;
            self.data[i] = x;
            i >>= 1;
            while i > 0 {
                self.data[i] = (self.operation)(self.data[i * 2], self.data[i * 2 + 1]);
                i >>= 1;
            }
        }
        pub fn query(&self, mut l: usize, mut r: usize) -> T {
            let mut a = self.identity;
            let mut b = self.identity;
            l += self.size;
            r += self.size;
            while l < r {
                if (l & 1) == 1 {
                    a = (self.operation)(a, self.data[l]);
                    l += 1;
                }
                if (r & 1) == 1 {
                    r -= 1;
                    b = (self.operation)(self.data[r], b);
                }
                l >>= 1;
                r >>= 1;
            }
            (self.operation)(a, b)
        }
    }
}

fn lower_bound<T: PartialOrd>(x: T, a: &Vec<T>) -> usize {
    let mut l: usize = 0;
    let mut r: usize = a.len();
    while r != l {
        let mid = (l + r) >> 1;
        if a[mid] < x {
            l = mid + 1;
        } else {
            r = mid;
        }
    }
    l
}

fn main() {
    let mut s: String = String::new();
    std::io::stdin().read_to_string(&mut s).ok();
    let mut itr = s.trim().split_whitespace();
    let n: usize = itr.next().unwrap().parse().unwrap();
    let mut a: Vec<(i64, i64)> = (0..n)
        .map(|_| {
            (
                itr.next().unwrap().parse().unwrap(),
                itr.next().unwrap().parse().unwrap(),
            )
        })
        .collect();
    a.sort();

    let mut x: Vec<i64> = vec![0; n];
    let mut d: Vec<i64> = vec![0; n];
    for i in 0..n {
        x[i] = a[i].0;
        d[i] = a[i].1;
    }
    // dp: iを選ぶ時、選ばない時で遷移
    // 選ばない時はi+1に遷移
    // iを選んだ時は同時に動く最大のもの+1に遷移
    // 同時に動く最大のもの: 再帰的に動きは起こるので、後ろから決めていく
    let mut st = segment_tree::SegmentTree::new(n, 0, std::cmp::max);
    let mut to: Vec<usize> = (0..n).collect();
    for i in (0..n).rev() {
        let j = lower_bound(x[i] + d[i], &x);
        if j > i + 1 {
            to[i] = st.query(i, j);
        }
        st.update(i, to[i]);
    }

    let mut dp: Vec<u64> = vec![0; n + 1];
    dp[0] = 1;
    for i in 0..n {
        dp[i + 1] = (dp[i + 1] + dp[i]) % MOD;
        dp[to[i] + 1] = (dp[to[i] + 1] + dp[i]) % MOD;
    }
    println!("{}", dp[n]);
}
