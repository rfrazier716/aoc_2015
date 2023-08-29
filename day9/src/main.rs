use itertools::{self, Itertools};
use regex::Regex;
use std::{collections::HashMap, fs};

#[derive(Default)]
struct AdjacencyMatrix {
    node_count: usize,
    adjacencies: Vec<f32>,
}

impl AdjacencyMatrix {
    pub fn new(n_nodes: usize) -> Self {
        let mut out = Self {
            node_count: n_nodes,
            adjacencies: vec![f32::INFINITY; n_nodes * n_nodes],
        };

        // self traversal is free
        for i in 0..n_nodes {
            out.adjacencies[i * (n_nodes + 1)] = 0.0;
        }
        return out;
    }

    pub fn new_from_builder() -> AdjacencyMatrixBuilder {
        return AdjacencyMatrixBuilder::default();
    }

    pub fn edge_weight(&self, origin: usize, destination: usize) -> f32 {
        return self.adjacencies[origin * self.node_count + destination];
    }

    pub fn calculate_traversal_cost(&self, path: impl Iterator<Item = usize>) -> f32 {
        path.tuple_windows()
            .fold(0.0, |sum, (from, to)| sum + self.edge_weight(from, to))
    }

    pub fn find_longest_traversal(&self) -> f32 {
        (0..self.node_count)
        .permutations(self.node_count)
        .map(|path| self.calculate_traversal_cost(path.into_iter()))
        .reduce(f32::max)
        .unwrap_or(0.0) 
    }
    pub fn find_shortest_traversal(&self) -> f32 {
        (0..self.node_count)
            .permutations(self.node_count)
            .map(|path| self.calculate_traversal_cost(path.into_iter()))
            .reduce(f32::min)
            .unwrap_or(f32::INFINITY)
    }
}

#[derive(Debug)]
struct Adjacency {
    origin: usize,
    destination: usize,
    weight: f32,
}

#[derive(Default)]
struct AdjacencyMatrixBuilder {
    node_map: HashMap<String, usize>,
    adjacencies: Vec<Adjacency>,
}

impl AdjacencyMatrixBuilder {
    pub fn with_edge(mut self, origin: String, destination: String, weight: f32) -> Self {
        // get the adjacency indices or insert them
        let next_idx = self.node_map.len();
        let orig_idx = *self.node_map.entry(origin).or_insert(next_idx);

        let next_idx = self.node_map.len();
        let dest_idx = *self.node_map.entry(destination).or_insert(next_idx);

        // update the adjecency vec
        self.adjacencies.push(Adjacency {
            origin: orig_idx,
            destination: dest_idx,
            weight: weight,
        });

        self.adjacencies.push(Adjacency {
            origin: dest_idx,
            destination: orig_idx,
            weight: weight,
        });

        // return self
        self
    }

    pub fn build(self) -> AdjacencyMatrix {
        let mut result = AdjacencyMatrix::new(self.node_map.len());
        for adjacency in self.adjacencies.into_iter() {
            result.adjacencies[adjacency.origin * result.node_count + adjacency.destination] =
                adjacency.weight;
        }
        return result;
    }
}

fn main() {
    let re = Regex::new(r"([A-Za-z]+) to ([a-zA-Z]+) = ([0-9]+)").unwrap();
    let input = fs::read_to_string("day9/input.txt").expect("Expected input file");
    let routes = input
        .lines()
        .filter_map(|line| re.captures(line))
        .map(|cap| {
            (
                cap[1].to_string(),
                cap[2].to_string(),
                cap[3].parse::<f32>().unwrap(),
            )
        });

    let graph = routes
        .fold(
            AdjacencyMatrix::new_from_builder(),
            |builder, (from, to, weight)| builder.with_edge(from, to, weight),
        )
        .build();

    println!("Part One Solution: {}", graph.find_shortest_traversal());
    println!("Part Two Solution: {}", graph.find_longest_traversal());

}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_part_one() {
        let matrix = AdjacencyMatrix::new_from_builder()
            .with_edge("London".into(), "Dublin".into(), 464.0)
            .with_edge("London".into(), "Belfast".into(), 518.0)
            .with_edge("Belfast".into(), "Dublin".into(), 141.0)
            .build();

        assert_eq!(matrix.find_shortest_traversal(), 605.0)
    }
}
