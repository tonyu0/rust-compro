use std::io::*;
// 白い部分に着目する・・・元の画像では現在白い部分の周り8方向は全部白かったはず。
// この考えがなかなか難しい。

fn main() {
    let mut s: String = String::new();
    std::io::stdin().read_to_string(&mut s).ok();
    let mut itr = s.trim().split_whitespace();
    let h: usize = itr.next().unwrap().parse().unwrap();
    let w: usize = itr.next().unwrap().parse().unwrap();
    let grid: Vec<Vec<char>> = (0..h)
        .map(|_| itr.next().unwrap().chars().collect())
        .collect();

    let dx: Vec<isize> = [0, -1, 0, 1, -1, 1, -1, 1].to_vec();
    let dy: Vec<isize> = [1, 0, -1, 0, -1, 1, 1, -1].to_vec();
    let mut ans = grid.clone();

    for y in 0..h {
        for x in 0..w {
            for i in 0..8 {
                let nx = x as isize + dx[i];
                let ny = y as isize + dy[i];
                if 0 <= nx && nx < w as isize && 0 <= ny && ny < h as isize {}
            }
        }
    }

    for y in 0..h {
        for x in 0..w {
            if grid[y][x] == '#' {
                let mut ok = ans[y][x] == '#';
                for i in 0..8 {
                    let nx = x as isize + dx[i];
                    let ny = y as isize + dy[i];
                    if 0 <= nx && nx < w as isize && 0 <= ny && ny < h as isize {
                        if ans[ny as usize][nx as usize] == '#' {
                            ok = true;
                        }
                    }
                }
                if !ok {
                    println!("impossible");
                    return;
                }
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
