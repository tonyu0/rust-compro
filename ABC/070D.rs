use std::io::*;

fn dijkstra(start: usize, graph: &Vec<Vec<(usize, i64)>>) -> Vec<i64> {
    let mut dist = vec![std::i64::MAX; graph.len()];
    dist[start] = 0;
    let mut bh = std::collections::BinaryHeap::new();
    bh.push((0i64, start));
    while let Some((cost, v)) = bh.pop() {
        if dist[v] < -cost {
            continue;
        }
        for e in graph[v].iter() {
            let nv = e.0;
            let nc = e.1;
            if dist[nv] > -cost + nc {
                dist[nv] = -cost + nc;
                bh.push((-dist[nv], nv));
            }
        }
    }
    dist
}

fn main() {
    let mut s: String = String::new();
    std::io::stdin().read_to_string(&mut s).ok();
    let mut itr = s.trim().split_whitespace();
    let n: usize = itr.next().unwrap().parse().unwrap();

    let mut graph: Vec<Vec<(usize, i64)>> = vec![Vec::new(); n];
    for _ in 0..n - 1 {
        let a: usize = itr.next().unwrap().parse::<usize>().unwrap() - 1;
        let b: usize = itr.next().unwrap().parse::<usize>().unwrap() - 1;
        let c: i64 = itr.next().unwrap().parse().unwrap();
        graph[a].push((b, c));
        graph[b].push((a, c));
    }
    let q: usize = itr.next().unwrap().parse().unwrap();
    let k: usize = itr.next().unwrap().parse().unwrap();
    let dist = dijkstra(k - 1, &graph);
    let mut out = Vec::new();
    for _ in 0..q {
        let x: usize = itr.next().unwrap().parse::<usize>().unwrap() - 1;
        let y: usize = itr.next().unwrap().parse::<usize>().unwrap() - 1;
        writeln!(out, "{}", dist[x] + dist[y]).ok();
    }
    stdout().write_all(&out).unwrap();
}
