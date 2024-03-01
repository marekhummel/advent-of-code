use std::cmp::Reverse;
use std::collections::hash_map::Iter;
use std::fmt::Debug;
use std::{
    collections::{BinaryHeap, HashMap, HashSet, VecDeque},
    hash::Hash,
};

use itertools::Itertools;

#[derive(Debug, Clone)]
pub struct Graph<V> {
    adjacency: HashMap<V, HashMap<V, i64>>,
}

impl<V: Eq + Hash> FromIterator<(V, HashSet<V>)> for Graph<V> {
    fn from_iter<T: IntoIterator<Item = (V, HashSet<V>)>>(iter: T) -> Self {
        Graph {
            adjacency: iter
                .into_iter()
                .map(|(vertex, adjacent)| (vertex, adjacent.into_iter().map(|a| (a, 1)).collect()))
                .collect(),
        }
    }
}

impl<V: Clone + Eq + Hash> Graph<V> {
    pub fn empty() -> Self {
        Graph {
            adjacency: HashMap::new(),
        }
    }

    pub fn vertices(&self) -> Vec<V> {
        self.adjacency
            .keys()
            .chain(self.adjacency.values().flat_map(|adj| adj.keys()))
            .unique()
            .cloned()
            .collect()
    }

    pub fn sinks(&self) -> Vec<V> {
        self.vertices()
            .into_iter()
            .filter(|v| self.adjacency.get(v).map_or(true, |adj| adj.is_empty()))
            .collect()
    }

    pub fn iter(&self) -> Iter<'_, V, HashMap<V, i64>> {
        self.adjacency.iter()
    }

    pub fn is_empty(&self) -> bool {
        self.adjacency.is_empty()
    }

    pub fn remove_vertex(&mut self, vertex: &V) {
        self.adjacency.remove(vertex);
        for (_, adj) in self.adjacency.iter_mut() {
            adj.remove(vertex);
        }
    }

    pub fn add_edge(&mut self, from: &V, to: &V, directed: bool) {
        self.add_weighted_edge(from, to, 1, directed)
    }

    pub fn add_weighted_edge(&mut self, from: &V, to: &V, weight: i64, directed: bool) {
        self.adjacency
            .entry(from.clone())
            .or_default()
            .insert(to.clone(), weight);
        if !directed {
            self.adjacency
                .entry(to.clone())
                .or_default()
                .insert(from.clone(), weight);
        }
    }

    pub fn remove_edge(&mut self, from: &V, to: &V, directed: bool) {
        self.adjacency.get_mut(from).unwrap().remove(to);
        if !directed {
            self.adjacency.get_mut(to).unwrap().remove(from);
        }
    }

    pub fn adjacent_vertices(&self, vertex: &V) -> Vec<V> {
        self.adjacency
            .get(vertex)
            .map_or(vec![], |adj| adj.keys().cloned().collect())
    }

    pub fn get_weight(&self, from: &V, to: &V) -> i64 {
        self.adjacency[from][to]
    }

    pub fn floyd_warshall(&self) -> HashMap<(V, V), i32> {
        let vertices = self.vertices();
        let n = vertices.len();
        let mut dist = vec![vec![i32::MAX as i64; n]; n];

        for (u, neighbors) in &self.adjacency {
            let ui = vertices.iter().position(|vert| u == vert).unwrap();
            for (v, weight) in neighbors.iter() {
                let vi = vertices.iter().position(|vert| v == vert).unwrap();
                dist[ui][vi] = *weight;
            }
        }

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

    /// Shortest paths from start to all other nodes
    pub fn dijkstra(&self, start: &V) -> HashMap<V, Vec<V>>
    where
        V: Ord,
    {
        let vertices = self.vertices();
        let mut prev = HashMap::new();
        let mut dist = vertices
            .iter()
            .map(|v| (v.clone(), i64::MAX - 1))
            .collect::<HashMap<_, _>>();
        *dist.get_mut(start).unwrap() = 0;

        let mut queue: BinaryHeap<(i64, &V)> = vertices.iter().map(|v| (-dist[v], v)).collect();

        while let Some((_, u)) = queue.pop() {
            for (v, weight) in self.adjacency[u].iter() {
                let alt = dist[u] + *weight;
                if alt < dist[v] {
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
                curr = prev[curr];
                path.push(curr.clone());
            }
            path.reverse();

            paths.insert(v.clone(), path);
        }

        paths
    }

    pub fn astar_no_heuristic(&self, start: &V, goal: &V) -> Option<(i64, Vec<V>)>
    where
        V: Ord,
    {
        self.astar(start, goal, |_, _| 0)
    }

    pub fn astar<F>(&self, start: &V, goal: &V, heuristic: F) -> Option<(i64, Vec<V>)>
    where
        V: Ord,
        F: Fn(&V, &V) -> i64,
    {
        let mut open = BinaryHeap::new();
        let mut closed = HashSet::new();

        open.push((Reverse(0), start.clone(), 0, vec![start.clone()]));

        // Loop until you find the end
        while let Some((_, current, g, path)) = open.pop() {
            closed.insert(current.clone());

            // Found the goal
            if &current == goal {
                return Some((g, path));
            }

            let children = &self.adjacency[&current];
            for (child, weight) in children {
                if closed.contains(child) {
                    continue;
                }

                let child_g = g + weight;
                let child_h = heuristic(child, goal);
                let child_f = child_g + child_h;

                if open.iter().any(|(_, o, og, _)| o == child && child_g >= *og) {
                    continue;
                }
                let new_path = path.iter().cloned().chain([child.clone()].into_iter()).collect();
                open.push((Reverse(child_f), child.clone(), child_g, new_path))
            }
        }

        None
    }

    pub fn components(&self) -> Vec<HashSet<V>> {
        let mut seen = HashSet::new();
        let mut components = Vec::new();

        let vertices = self.vertices();
        while seen.len() < vertices.len() {
            let vertex = vertices.iter().find(|p| !seen.contains(*p)).unwrap();

            let mut comp = HashSet::new();
            let mut queue = VecDeque::from([vertex.clone()]);
            while let Some(u) = queue.pop_front() {
                comp.insert(u.clone());
                if seen.contains(&u) {
                    continue;
                }

                seen.insert(u.clone());

                for w in self.adjacency[&u].keys() {
                    queue.push_back(w.clone());
                }
            }

            components.push(comp);
        }

        components
    }

    /// Kahn's Algorithm
    pub fn topo_sorting(&self) -> Option<Vec<V>>
    where
        V: Ord,
    {
        // Invert graph to map trg to source nodes
        let mut sources = self.invert().adjacency;

        let mut sorted = Vec::new();
        let mut next: BinaryHeap<_> = sources
            .iter()
            .filter(|(_, srcs)| srcs.is_empty())
            .map(|(trg, _)| Reverse(trg.clone())) // Use reverse to ensure smaller nodes are chosen first
            .collect();

        while let Some(Reverse(node)) = next.pop() {
            if sources.remove(&node).is_some() {
                sorted.push(node.clone());
                for (trg, srcs) in sources.iter_mut() {
                    srcs.remove(&node);
                    if srcs.is_empty() {
                        next.push(Reverse(trg.clone()));
                    }
                }
            }
        }

        if sources.is_empty() {
            Some(sorted)
        } else {
            None
        }
    }

    // Invert graph to map trg to source nodes
    pub fn invert(&self) -> Self {
        self.vertices()
            .into_iter()
            .map(|trg| {
                (
                    trg.clone(),
                    self.adjacency
                        .iter()
                        .filter(|(_, trgs)| trgs.contains_key(&trg))
                        .map(|(src, _)| src.clone())
                        .collect::<HashSet<_>>(),
                )
            })
            .collect()
    }
}
