use std::io::Read;

fn main() {
    let mut s: String = String::new();
    std::io::stdin().read_to_string(&mut s).ok();
    let mut itr = s.trim().split_whitespace();
    let n: usize = itr.next().unwrap().parse().unwrap();
    let c: usize = itr.next().unwrap().parse().unwrap();

    let mut cumsum: Vec<Vec<i64>> = vec![vec![0; 100010]; c + 1];
    for _ in 0..n {
        let s: usize = itr.next().unwrap().parse::<usize>().unwrap() - 1;
        let t: usize = itr.next().unwrap().parse::<usize>().unwrap() - 1;
        let c: usize = itr.next().unwrap().parse::<usize>().unwrap() - 1;
        cumsum[c][s] += 1;
        cumsum[c][t + 1] -= 1;
    }
    for i in 0..c {
        for j in 0..100005 {
            cumsum[i][j + 1] += cumsum[i][j];
        }
    }

    let mut ans: i64 = 0;
    for i in 0..100005 {
        let mut cnt: i64 = 0;
        for j in 0..c {
            if cumsum[j][i] > 0 {
                cnt += 1;
            }
        }
        ans = std::cmp::max(ans, cnt);
    }
    println!("{}", ans);
}
