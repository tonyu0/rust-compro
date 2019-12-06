// union findを用いたdisjoint setの管理

// TODO: pubとかつけて動かす

mod disjoint_set {
  struct DisjointSet {
    parent: Vec<usize>,
    size: Vec<usize>,
  }

  impl DisjointSet {
    fn new(n: usize) -> DisjointSet {
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
        // 根に直接繋ぐ
        self.parent[x] = self.root(tmp);
        self.parent[x]
      }
    }
    fn merge(&mut self, x: usize, y: usize) {
      let x = self.root(x);
      let y = self.root(y);
      // xとyは既に一緒のグループ
      if x == y {
        return;
      }
      // マージテク・・・集合のサイズが大きい方にマージする。
      if self.size[x] > self.size[y] {
        self.size[x] += self.size[y];
        self.parent[y] = x;
      } else {
        self.size[y] += self.size[x];
        self.parent[x] = y;
      }
    }
    fn same(&mut self, x: usize, y: usize) -> bool {
      self.root(x) == self.root(y)
    }

    fn size(&mut self, x: usize) -> usize {
      let x = self.root(x);
      self.size[x]
    }
  }
}
