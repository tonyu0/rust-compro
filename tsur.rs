use std::io::*;

// mul(a_x*y, b_z*w) = c_x*w
fn mul(a: Vec<Vec<u64>>, b: Vec<Vec<u64>>) -> Vec<Vec<u64>> {
    let mut res: Vec<Vec<u64>> = vec![vec![0; b[0].len()]; a.len()];

    for i in 0..a.len() {
        for j in 0..b[0].len() {
            let mut val = 0;
            for k in 0..b.len() {
                val ^= a[i][k] & b[k][j];
            }
            res[i][j] = val;
        }
    }
    res
}

// 正方行列のみ
fn pow(mut a: Vec<Vec<u64>>, mut e: u64) -> Vec<Vec<u64>> {
    let mut res: Vec<Vec<u64>> = vec![vec![0; a.len()]; a.len()];
    for i in 0..a.len() {
        res[i][i] = !0u64;
    }
    while e > 0 {
        if e & 1 == 1 {
            res = mul(res, a.clone());
        }
        a = mul(a.clone(), a.clone());
        e >>= 1;
    }
    res
}

fn main() {
    let mut s: String = String::new();
    std::io::stdin().read_to_string(&mut s).ok();
    let mut itr = s.trim().split_whitespace();
    let k: usize = itr.next().unwrap().parse().unwrap();
    let m: u64 = itr.next().unwrap().parse().unwrap();
    let a: Vec<u64> = (0..k)
        .map(|_| itr.next().unwrap().parse().unwrap())
        .collect();
    let c: Vec<u64> = (0..k)
        .map(|_| itr.next().unwrap().parse().unwrap())
        .collect();

    let mut matc: Vec<Vec<u64>> = vec![vec![0; k]; k];
    let mut mata: Vec<Vec<u64>> = vec![vec![0; 1]; k];
    for i in 0..k {
        matc[0][i] = c[i];
        if i + 1 < k {
            matc[i + 1][i] = !0u64;
        }
    }
    for i in 0..k {
        mata[k - i - 1][0] = a[i];
    }

    let matc = pow(matc, m - 1);
    let ans = mul(matc, mata);
    println!("{}", ans[k - 1][0]);
}
