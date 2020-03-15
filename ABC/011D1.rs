use std::io::Read;
// 010D 大ジャンプ log2バージョン

fn main() {
    let mut s: String = String::new();
    std::io::stdin().read_to_string(&mut s).ok();
    let mut itr = s.trim().split_whitespace();
    let n: usize = itr.next().unwrap().parse().unwrap();
    let d: i64 = itr.next().unwrap().parse().unwrap();
    let mut x: i64 = itr.next().unwrap().parse().unwrap();
    let mut y: i64 = itr.next().unwrap().parse().unwrap();
    if x % d != 0 || y % d != 0 || x / d + y / d > n as i64 {
        println!("0");
        return;
    }
    x /= d;
    y /= d;
    if x < 0 {
        x *= -1;
    }
    if y < 0 {
        y *= -1;
    }

    let mut log: Vec<f64> = vec![0.0; 1010];
    for i in 1..1010 {
        log[i] = log[i - 1] + (i as f64).log2();
    }
    let nck = |n, k| log[n] - log[k] - log[n - k];

    let mut ans: f64 = 0.0;
    // 横向きにi回
    for i in 0..n + 1 {
        if i < x as usize || (i - x as usize) % 2 != 0 {
            continue;
        }
        // 横向きiかいでxにたどり着くには
        // x + (i - x) / 2;
        // y + (j - y) / 2;
        let j = n - i;
        if j < y as usize || (j - y as usize) % 2 != 0 {
            continue;
        }
        let tmp = nck(n, i) + nck(i, (i - x as usize) / 2) + nck(j, (j - y as usize) / 2)
            - (n as f64) * 2.0;
        ans += 2.0_f64.powf(tmp);
    }
    println!("{:.15}", ans);
}
