use std::io::*;

fn main() {
    let mut s: String = String::new();
    std::io::stdin().read_to_string(&mut s).ok();
    let mut itr = s.trim().split_whitespace();
    let h: usize = itr.next().unwrap().parse().unwrap();
    let w: usize = itr.next().unwrap().parse().unwrap();
    let s: Vec<Vec<char>> = (0..h)
        .map(|_| itr.next().unwrap().chars().collect())
        .collect();

    let mut dist: Vec<Vec<usize>> = vec![vec![500; w + 1]; h + 1];
    let mut que = std::collections::VecDeque::new();
    let dx = [1, 0, -1, 0].to_vec();
    let dy = [0, -1, 0, 1].to_vec();

    dist[0][0] = 1;
    que.push_back((0, 0));

    while let Some((y, x)) = que.pop_front() {
        for i in 0..4 {
            let nx = x as isize + dx[i];
            let ny = y as isize + dy[i];
            if 0 <= nx
                && nx < w as isize
                && 0 <= ny
                && ny < h as isize
                && s[ny as usize][nx as usize] == '.'
                && dist[ny as usize][nx as usize] == 500
            {
                dist[ny as usize][nx as usize] = dist[y][x] + 1;
                que.push_back((ny as usize, nx as usize));
            }
        }
    }
    if dist[h - 1][w - 1] == 500 {
        println!("-1");
        return;
    }
    let mut ans = 0;
    for c in s {
        for i in c {
            if i == '.' {
                ans += 1;
            }
        }
    }
    println!("{}", ans - dist[h - 1][w - 1]);
}
