use std::io::*;
// 愚直に元画像を収縮、その後オッケーかかくにん。

fn main() {
    let mut s: String = String::new();
    std::io::stdin().read_to_string(&mut s).ok();
    let mut itr = s.trim().split_whitespace();
    let h: isize = itr.next().unwrap().parse().unwrap();
    let w: isize = itr.next().unwrap().parse().unwrap();
    let grid: Vec<Vec<char>> = (0..h)
        .map(|_| itr.next().unwrap().chars().collect())
        .collect();

    let dx: Vec<isize> = [0, -1, 0, 1, -1, 1, -1, 1].to_vec();
    let dy: Vec<isize> = [1, 0, -1, 0, -1, 1, 1, -1].to_vec();

    let mut ans: Vec<Vec<char>> = vec![vec!['.'; w as usize]; h as usize];
    let mut check: Vec<Vec<bool>> = vec![vec![false; w as usize]; h as usize];
    for y in 0..h {
        for x in 0..w {
            if grid[y as usize][x as usize] != '#' {
                continue;
            }
            let mut is_black = true;
            for k in 0..8 {
                let nx = x + dx[k];
                let ny = y + dy[k];
                if 0 <= nx && nx < w && 0 <= ny && ny < h && grid[ny as usize][nx as usize] != '#' {
                    is_black = false;
                }
            }
            if is_black {
                ans[y as usize][x as usize] = '#';
                check[y as usize][x as usize] = true;
                for k in 0..8 {
                    let nx = x + dx[k];
                    let ny = y + dy[k];
                    if 0 <= nx && nx < w && 0 <= ny && ny < h {
                        check[ny as usize][nx as usize] = true;
                    }
                }
            }
        }
    }

    for i in 0..h {
        for j in 0..w {
            if (check[i as usize][j as usize] && grid[i as usize][j as usize] == '.')
                || (!check[i as usize][j as usize] && grid[i as usize][j as usize] == '#')
            {
                println!("impossible");
                return;
            }
        }
    }

    let mut out = Vec::new();
    println!("possible");
    for i in 0..h {
        let tmp: String = ans[i as usize].clone().into_iter().collect();
        writeln!(out, "{}", tmp).ok();
    }
    stdout().write_all(&out).unwrap();
}
