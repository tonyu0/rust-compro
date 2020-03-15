use std::io::Read;

fn dfs(st: &mut Vec<usize>, n: usize, k: usize, g: &Vec<Vec<bool>>) -> usize {
    let mut res = 0;
    if n == k {
        let mut ok = true;
        for i in 0..st.len() {
            for j in i + 1..st.len() {
                if !g[st[i]][st[j]] {
                    ok = false;
                }
            }
        }
        if ok {
            res = st.len();
        }
    } else {
        st.push(k);
        res = std::cmp::max(res, dfs(st, n, k + 1, &g));
        st.pop();
        res = std::cmp::max(res, dfs(st, n, k + 1, &g));
    }
    res
}

fn main() {
    let mut s: String = String::new();
    std::io::stdin().read_to_string(&mut s).ok();
    let mut itr = s.trim().split_whitespace();
    let n: usize = itr.next().unwrap().parse().unwrap();
    let m: usize = itr.next().unwrap().parse().unwrap();
    let mut g: Vec<Vec<bool>> = vec![vec![false; n]; n];
    for _i in 0..m {
        let x: usize = itr.next().unwrap().parse::<usize>().unwrap() - 1;
        let y: usize = itr.next().unwrap().parse::<usize>().unwrap() - 1;
        g[x][y] = true;
        g[y][x] = true;
    }

    let mut st = Vec::new();
    println!("{}", dfs(&mut st, n, 0, &g));
}
