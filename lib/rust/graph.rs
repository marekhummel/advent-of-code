use std::collections::{BinaryHeap, HashMap, HashSet};

use itertools::Itertools;

fn floyd_marshall(graph: &HashMap<String, HashSet<String>>) -> HashMap<(String, String), i32> {
    let vertices = graph.keys().cloned().collect_vec();
    let n = vertices.len();
    let mut dist = vec![vec![i32::MAX as i64; n]; n];

    for (u, neighbors) in graph {
        let ui = vertices.iter().position(|vert| u == vert).unwrap();
        for v in neighbors {
            let vi = vertices.iter().position(|vert| v == vert).unwrap();
            dist[ui][vi] = 1;
        }
    }

    #[allow(clippy::needless_range_loop)]
    for vi in 0..n {
        dist[vi][vi] = 0;
    }

    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                if dist[i][j] > dist[i][k] + dist[k][j] {
                    dist[i][j] = dist[i][k] + dist[k][j];
                }
            }
        }
    }

    (0..n)
        .tuple_combinations()
        .flat_map(|(ui, vi)| {
            [
                ((vertices[ui].clone(), vertices[vi].clone()), dist[ui][vi] as i32),
                ((vertices[vi].clone(), vertices[ui].clone()), dist[vi][ui] as i32),
            ]
        })
        .collect()
}

fn dijkstra(graph: &HashMap<String, HashSet<String>>, start: &str) -> HashMap<String, Vec<String>> {
    let vertices = graph.keys().cloned().collect_vec();
    let mut prev = HashMap::new();
    let mut dist = vertices
        .iter()
        .map(|v| (v.as_str(), i32::MAX - 1))
        .collect::<HashMap<_, _>>();
    *dist.get_mut(start).unwrap() = 0;

    let mut queue: BinaryHeap<(i32, &str)> = vertices.iter().map(|v| (-dist[v.as_str()], v.as_str())).collect();

    while let Some((_, u)) = queue.pop() {
        for v in graph[u].iter() {
            let alt = dist[u] + 1;
            if alt < dist[v.as_str()] {
                *dist.get_mut(v.as_str()).unwrap() = alt;
                prev.entry(v.as_str()).and_modify(|e| *e = u).or_insert(u);
                queue.push((-alt, v))
            }
        }
    }

    let mut paths = HashMap::new();
    for v in vertices.iter() {
        if *v == start {
            continue;
        }

        let mut curr = v.as_str();
        let mut path = vec![v.clone()];
        while curr != start {
            curr = prev[curr];
            path.push(curr.to_string());
        }
        path.reverse();

        paths.insert(v.clone(), path);
    }

    paths
}
