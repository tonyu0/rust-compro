use std::io::*;
const MOD: u64 = 1_000_000_007;
// メモ化再帰・・・更新順序が非自明な場合のDPと言える。
// 今回は有向グラフの経路の選び方の数え上げだが、始点が決まらないので、端から決まるDPをやる必要がある。
// ある頂点が根である場合の経路数は、葉から決まる。
// 今回は根が簡単に決まらないので、再帰DPによりある節を根とした時の経路数をメモ化しておき、
// その根が節になった時に足し合わせられるようにする。

fn dfs(x: usize, y: usize, w: usize, h: usize, grid: &Vec<usize>, memo: &mut Vec<u64>) -> u64 {
    if memo[y * w + x] != 0 {
        return memo[y * w + x];
    }

    let mut res = 1u64;
    let dx: Vec<isize> = [0, -1, 0, 1].to_vec();
    let dy: Vec<isize> = [1, 0, -1, 0].to_vec();
    for i in 0..4 {
        let nx = x as isize + dx[i];
        let ny = y as isize + dy[i];
        if 0 <= nx
            && nx < w as isize
            && 0 <= ny
            && ny < h as isize
            && grid[y * w + x] < grid[ny as usize * w + nx as usize]
        {
            res = (res + dfs(nx as usize, ny as usize, w, h, &grid, memo)) % MOD;
        }
    }
    memo[y * w + x] = res;
    res
}

fn main() {
    let mut s: String = String::new();
    std::io::stdin().read_to_string(&mut s).ok();
    let mut itr = s.trim().split_whitespace();
    let h: usize = itr.next().unwrap().parse().unwrap();
    let w: usize = itr.next().unwrap().parse().unwrap();
    let a: Vec<usize> = (0..h * w)
        .map(|_| itr.next().unwrap().parse().unwrap())
        .collect();

    let mut ans = 0u64;
    let mut memo: Vec<u64> = vec![0; h * w];
    for i in 0..h {
        for j in 0..w {
            ans = (ans + dfs(j, i, w, h, &a, &mut memo)) % MOD;
        }
    }
    println!("{}", ans);
}
