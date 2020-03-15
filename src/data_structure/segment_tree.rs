use std::io::Read;

mod segment_tree {
  pub struct SegmentTree<T, F> {
    size: usize,
    data: Vec<T>,
    identity: T,
    operation: F,
  }

  impl<T: Clone + Copy, F: Fn(T, T) -> T> SegmentTree<T, F> {
    pub fn new(n: usize, id: T, op: F) -> SegmentTree<T, F> {
      let mut size = 1;
      while size < n {
        size <<= 1;
      }
      SegmentTree {
        size: size,
        data: vec![id; size * 2 - 1],
        identity: id,
        operation: op,
      }
    }

    pub fn update(&mut self, mut i: usize, x: T) {
      i += self.size - 1;
      self.data[i] = x;
      while i > 0 {
        i = (i - 1) >> 1;
        self.data[i] = (self.operation)(self.data[i * 2 + 1], self.data[i * 2 + 2]);
      }
    }

    pub fn query(&self, a: usize, b: usize) -> T {
      return self._query(a, b, 0, 0, self.size);
    }

    fn _query(&self, a: usize, b: usize, i: usize, l: usize, r: usize) -> T {
      if b <= l || r <= a {
        return self.identity;
      }
      if a <= l && r <= b {
        return self.data[i];
      }
      let v1 = self._query(a, b, i * 2 + 1, l, (l + r) / 2);
      let v2 = self._query(a, b, i * 2 + 2, (l + r) / 2, r);
      return (self.operation)(v1, v2);
    }
  }
}

fn main() {
  let mut s: String = String::new();
  std::io::stdin().read_to_string(&mut s).ok();
  let mut itr = s.trim().split_whitespace();
  let n: usize = itr.next().unwrap().parse().unwrap();
  let q: usize = itr.next().unwrap().parse().unwrap();
  let mut st = segment_tree::SegmentTree::new(n, (1u64 << 31) - 1, std::cmp::min);
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
