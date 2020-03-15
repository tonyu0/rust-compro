use std::io::Read;

// 1 <= a1 <= a2 ... <= ak <= n
// これは1 ~ nである数字からk個重複を許して選ぶ方法 (難しく考えることはない)

mod enumeration {
    pub struct Enumeration {
        fact: Vec<u64>,
        finv: Vec<u64>,
        modulo: u64,
    }

    impl Enumeration {
        pub fn new(n: usize, modulo: u64) -> Enumeration {
            let mut fact: Vec<u64> = vec![1; n];
            let mut inv: Vec<u64> = vec![1; n];
            let mut finv: Vec<u64> = vec![1; n];
            for i in 2..n {
                fact[i] = fact[i - 1] * (i as u64) % modulo;
                inv[i] = modulo - (modulo / (i as u64)) * inv[(modulo as usize % i)] % modulo;
                finv[i] = inv[i] * finv[i - 1] % modulo;
            }
            Enumeration {
                fact: fact,
                finv: finv,
                modulo: modulo,
            }
        }

        pub fn c(&self, n: usize, k: usize) -> u64 {
            if n < k {
                return 0;
            }
            self.fact[n] * self.finv[n - k] % self.modulo * self.finv[k] % self.modulo
        }
        pub fn h(&self, n: usize, k: usize) -> u64 {
            self.c(n + k - 1, k)
        }
    }
}

fn main() {
    let mut s: String = String::new();
    std::io::stdin().read_to_string(&mut s).ok();
    let mut itr = s.trim().split_whitespace();
    let n: usize = itr.next().unwrap().parse().unwrap();
    let k: usize = itr.next().unwrap().parse().unwrap();
    let en = enumeration::Enumeration::new(200010, 1_000_000_007);
    println!("{}", en.h(n, k));
}
