use std::io::Read;

// 平均値最大化
// 平均値をx以上にできる場合、平均値をx-1以上にすることも可能。
// 単調性があるので二分探索ができる。
// Σ (wi * pi) / Σ wi = x (xは達成可能か判定する)
// Σ (wi * pi) - x * Σ wi >= 0 (0以上だとこの選び方は可能)
// Σ (wi * pi - x * wi) >= 0 (xに単調性がある)

fn calc(x: f64, k: usize, w: &Vec<f64>, p: &Vec<f64>) -> bool {
    let n = w.len();
    let mut val: Vec<f64> = vec![0.0; n];
    for i in 0..n {
        val[i] = w[i] * p[i] - x * w[i];
    }
    val.sort_by(|a, b| b.partial_cmp(a).unwrap());

    let mut check: f64 = 0.0;
    for i in 0..k {
        check += val[i];
    }
    check >= 0.0
}

fn main() {
    let mut s: String = String::new();
    std::io::stdin().read_to_string(&mut s).ok();
    let mut itr = s.trim().split_whitespace();
    let n: usize = itr.next().unwrap().parse().unwrap();
    let k: usize = itr.next().unwrap().parse().unwrap();
    let mut w: Vec<f64> = vec![0.0; n];
    let mut p: Vec<f64> = vec![0.0; n];
    for i in 0..n {
        w[i] = itr.next().unwrap().parse().unwrap();
        p[i] = itr.next().unwrap().parse().unwrap();
        p[i] /= 100.0;
    }

    let mut ok = 0.0;
    let mut ng = 1.0;

    for _ in 0..1000 {
        let mid = (ok + ng) / 2.0;
        if calc(mid, k, &w, &p) {
            ok = mid;
        } else {
            ng = mid;
        }
    }
    println!("{:.10}", ok * 100.0);
}
