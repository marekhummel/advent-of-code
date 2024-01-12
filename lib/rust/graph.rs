use std::{
    collections::{BinaryHeap, HashMap, HashSet, VecDeque},
    hash::Hash,
};

use itertools::Itertools;

pub fn floyd_marshall<V: Clone + Eq + Hash>(graph: &HashMap<V, HashSet<V>>) -> HashMap<(V, V), i32> {
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

pub fn dijkstra<V: Eq + Hash + Clone + Ord>(graph: &HashMap<V, HashSet<V>>, start: &V) -> HashMap<V, Vec<V>> {
    let vertices = graph.keys().cloned().collect_vec();
    let mut prev = HashMap::new();
    let mut dist = vertices
        .iter()
        .map(|v| (v.clone(), i32::MAX - 1))
        .collect::<HashMap<_, _>>();
    *dist.get_mut(start).unwrap() = 0;

    let mut queue: BinaryHeap<(i32, &V)> = vertices.iter().map(|v| (-dist[&v], v)).collect();

    while let Some((_, u)) = queue.pop() {
        for v in graph[u].iter() {
            let alt = dist[&u] + 1;
            if alt < dist[&v] {
                *dist.get_mut(v).unwrap() = alt;
                prev.entry(v.clone()).and_modify(|e| *e = u).or_insert(u);
                queue.push((-alt, v))
            }
        }
    }

    let mut paths = HashMap::new();
    for v in vertices.iter() {
        if v == start {
            continue;
        }

        let mut curr = v;
        let mut path = vec![v.clone()];
        while curr != start {
            curr = prev[&curr];
            path.push(curr.clone());
        }
        path.reverse();

        paths.insert(v.clone(), path);
    }

    paths
}

pub fn components<V: Eq + Hash + Clone>(graph: &HashMap<V, HashSet<V>>) -> Vec<HashSet<V>> {
    let mut seen = HashSet::new();
    let mut components = Vec::new();

    while seen.len() < graph.len() {
        let vertex = graph.keys().find(|p| !seen.contains(*p)).unwrap();

        let mut comp = HashSet::new();
        let mut queue = VecDeque::from([vertex.clone()]);
        while let Some(u) = queue.pop_front() {
            comp.insert(u.clone());
            if seen.contains(&u) {
                continue;
            }

            seen.insert(u.clone());

            for w in graph[&u].iter() {
                queue.push_back(w.clone());
            }
        }

        components.push(comp);
    }

    components
}
