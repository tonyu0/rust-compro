use std::io::Read;

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
      let mut p = self.identity;
      let mut q = self.identity;
      l += self.size;
      r += self.size;
      while l < r {
        if (l & 1) == 1 {
          p = (self.operation)(p, self.data[l]);
          l += 1;
        }
        if (r & 1) == 1 {
          r -= 1;
          q = (self.operation)(self.data[r], q);
        }
        l >>= 1;
        r >>= 1;
      }
      (self.operation)(p, q)
    }
  }
}

fn main() {
  let mut s: String = String::new();
  std::io::stdin().read_to_string(&mut s).ok();
  let mut itr = s.trim().split_whitespace();
  let n: usize = itr.next().unwrap().parse().unwrap();
  let q: usize = itr.next().unwrap().parse().unwrap();
  let mut st = segment_tree::SegmentTree::new(n, (1u64 << 31) - 1, |x, y| x | y);
  for _ in 0..q {
    let t: usize = itr.next().unwrap().parse().unwrap();
    let l: usize = itr.next().unwrap().parse().unwrap();
    let r: usize = itr.next().unwrap().parse().unwrap();
    if t == 0 {
      st.update(l, r as u64);
    } else {
      println!("{}", st.query(l, r + 1));
    }
  }
}
