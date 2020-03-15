use std::io::*;

fn main() {
    let mut s: String = String::new();
    std::io::stdin().read_to_string(&mut s).ok();
    let mut itr = s.trim().split_whitespace();
    let q: usize = itr.next().unwrap().parse().unwrap();
    let mut out = Vec::new();

    let mut is_prime: Vec<bool> = vec![true; 100010];
    let mut cumsum: Vec<u64> = vec![0; 100010];
    is_prime[0] = false;
    is_prime[1] = false;
    for i in 2..100005 {
        if is_prime[i] {
            let mut j = i * 2;
            while j < 100005 {
                is_prime[j] = false;
                j += i;
            }
        }
    }
    for i in 1..100005 {
        if is_prime[i] && is_prime[(i + 1) >> 1] {
            cumsum[i] += 1;
        }
    }
    for i in 0..100005 {
        cumsum[i + 1] += cumsum[i];
    }

    for _ in 0..q {
        let l: usize = itr.next().unwrap().parse().unwrap();
        let r: usize = itr.next().unwrap().parse().unwrap();
        writeln!(out, "{}", cumsum[r] - cumsum[l - 1]).unwrap();
    }
    stdout().write_all(&out).unwrap();
}
