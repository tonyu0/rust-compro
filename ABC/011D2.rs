use std::io::Read;
// 010D 大ジャンプ パスカルの三角形バージョン

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

    let mut c: Vec<Vec<f64>> = vec![vec![0.0; 1010]; 1010];
    c[0][0] = 1.0;
    for i in 1..1010 {
        c[i][0] = c[i - 1][0] / 2.0;
        for j in 1..i {
            c[i][j] = (c[i - 1][j - 1] + c[i - 1][j]) / 2.0;
        }
        c[i][i] = c[i - 1][i - 1] / 2.0;
    }

    let mut ans: f64 = 0.0;
    // 横向きにi回
    for i in 0..n + 1 {
        if i < x as usize || (i - x as usize) % 2 != 0 {
            continue;
        }
        let j = n - i;
        if j < y as usize || (j - y as usize) % 2 != 0 {
            continue;
        }
        ans += c[n][i] * c[i][(i - x as usize) / 2] * c[j][(j - y as usize) / 2];
    }
    println!("{:.15}", ans);
}
