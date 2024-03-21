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

pub trait PathFinding<V: Clone + Eq + Hash + Ord> {
    fn transitions(&mut self, current: &V) -> Vec<(V, i64)>;

    fn astar_no_heuristic(&mut self, start: &V, goal: &V) -> Option<(i64, Vec<V>)> {
        self.astar(start, goal, |_| 0)
    }

    fn astar_many<F, G>(&mut self, start: &V, is_goal: G, heuristic: F) -> Option<(i64, Vec<V>)>
    where
        F: Fn(&V) -> i64,
        G: Fn(&V) -> bool,
    {
        let mut open = BinaryHeap::new();
        let mut closed = HashSet::new();

        open.push((Reverse(0), start.clone(), vec![start.clone()]));
        let mut g_score_map = HashMap::from([(start.clone(), 0)]);

        // Loop until you find the end
        while let Some((_f, current, path)) = open.pop() {
            closed.insert(current.clone());

            let g = g_score_map[&current];

            // Found the goal
            if is_goal(&current) {
                return Some((g, path));
            }

            for (child, weight) in self.transitions(&current) {
                if closed.contains(&child) {
                    continue;
                }

                let child_g = g + weight;
                if *g_score_map.get(&child).unwrap_or(&i64::MAX) <= child_g {
                    continue;
                }
                g_score_map.insert(child.clone(), child_g);

                let child_h = heuristic(&child);
                let child_f = child_g + child_h;

                let mut new_path = path.clone();
                new_path.push(child.clone());
                open.push((Reverse(child_f), child.clone(), new_path))
            }
        }

        None
    }

    fn astar<F>(&mut self, start: &V, goal: &V, heuristic: F) -> Option<(i64, Vec<V>)>
    where
        F: Fn(&V) -> i64,
    {
        self.astar_many(start, |g| g == goal, heuristic)
    }

    /// Shortest paths from start to all other nodes (does not contain total cost of path)
    fn dijkstra(&mut self, start: &V) -> HashMap<V, Vec<V>>
    where
        V: Ord,
    {
        let mut prev = HashMap::new();
        let mut dist = HashMap::from([(start.clone(), 0)]);
        let mut seen_vertices = HashSet::from([start.clone()]);

        let mut queue = BinaryHeap::from([(0, start.clone())]);

        while let Some((_, u)) = queue.pop() {
            for (v, weight) in self.transitions(&u) {
                let alt = dist.get(&u).unwrap_or(&i64::MAX).saturating_add(weight);
                if alt < *dist.get(&v).unwrap_or(&i64::MAX) {
                    *dist.entry(v.clone()).or_default() = alt;
                    prev.entry(v.clone())
                        .and_modify(|e| *e = u.clone())
                        .or_insert(u.clone());
                    queue.push((-alt, v.clone()));
                    seen_vertices.insert(v.clone());
                }
            }
        }

        let mut paths = HashMap::new();
        for v in seen_vertices.into_iter() {
            if v == *start {
                continue;
            }

            let mut curr = v.clone();
            let mut path = vec![v.clone()];
            while curr != *start {
                curr = prev[&curr].clone();
                path.push(curr.clone());
            }
            path.reverse();

            paths.insert(v.clone(), path);
        }

        paths
    }
}

impl<V: Clone + Eq + Hash + Ord> PathFinding<V> for Graph<V> {
    fn transitions(&mut self, current: &V) -> Vec<(V, i64)> {
        if let Some(children) = self.adjacency.get(current) {
            children.iter().map(|(v, w)| (v.clone(), *w)).collect_vec()
        } else {
            vec![]
        }
    }
}

pub struct DynamicGraph<V: Copy> {
    #[allow(clippy::type_complexity)]
    pub adjacent: Box<dyn FnMut(&V) -> Vec<(V, i64)>>,
}

impl<V: Clone + Eq + Hash + Ord + Copy> PathFinding<V> for DynamicGraph<V> {
    fn transitions(&mut self, current: &V) -> Vec<(V, i64)> {
        (self.adjacent)(current)
    }
}
