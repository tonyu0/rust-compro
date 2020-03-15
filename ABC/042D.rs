use std::io::Read;

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

        pub fn nck(&self, n: usize, k: usize) -> u64 {
            if n < k {
                return 0;
            }
            self.fact[n] * self.finv[n - k] % self.modulo * self.finv[k] % self.modulo
        }
    }
}

fn main() {
    let mut s: String = String::new();
    std::io::stdin().read_to_string(&mut s).ok();
    let mut itr = s.trim().split_whitespace();
    let h: usize = itr.next().unwrap().parse().unwrap();
    let w: usize = itr.next().unwrap().parse().unwrap();
    let a: usize = itr.next().unwrap().parse().unwrap();
    let b: usize = itr.next().unwrap().parse().unwrap();

    const MOD: u64 = 1_000_000_007;
    let comb = enumeration::Enumeration::new(200010, MOD);

    let mut ans: u64 = 0;
    for i in b..w {
        ans = (ans + comb.nck(h - 1 - a + i, i) * comb.nck(a - 1 + w - 1 - i, a - 1) % MOD) % MOD
    }
    println!("{}", ans);
}
