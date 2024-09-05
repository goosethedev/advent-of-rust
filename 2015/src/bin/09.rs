advent_of_code::solution!(9);

use itertools::Itertools;
use ndarray::{prelude::*, Array, Ix2};
use std::collections::HashSet;

// Traveling Salesman - naive approach
type Graph = Array<u32, Ix2>;

fn build_graph(input: &str) -> Option<Graph> {
    let mut set = HashSet::new();
    let mut tuples = vec![];

    for line in input.trim().lines() {
        let mut elems = line.split(" = ");
        let left = elems.next()?;
        let right = elems.next()?;

        let mut cities = left.split(" to ");
        let city_a = cities.next()?;
        let city_b = cities.next()?;
        let distance = right.parse::<u32>().ok()?;
        set.insert(city_a);
        set.insert(city_b);
        tuples.push((city_a, city_b, distance));
    }

    let n = set.len();
    let list: Vec<&str> = set.into_iter().collect();
    let mut graph: Graph = Array::from_elem((n, n), 0);
    for (city_a, city_b, dist) in tuples {
        let a_idx = list.iter().position(|c| *c == city_a).unwrap();
        let b_idx = list.iter().position(|c| *c == city_b).unwrap();
        let mut slice = graph.slice_mut(s![a_idx, b_idx]);
        slice.fill(dist);
        let mut slice = graph.slice_mut(s![b_idx, a_idx]);
        slice.fill(dist);
    }
    Some(graph)
}

// FIXME: This returns different min/max values per run lol
fn find_route_cost(graph: Graph, min: bool) -> u32 {
    let initial = 0;
    let range = 0..graph.row(0).len();
    let vertex: Vec<usize> = range.into_iter().filter(|v| *v != initial).collect();

    let mut total_cost = if min { u32::MAX } else { 0 };
    let permutations = vertex.iter().permutations(vertex.len()).unique();
    for row in permutations {
        let mut cur_cost = 0;

        let mut k = initial;
        for j in row {
            cur_cost += graph[[k, *j]];
            k = *j;
        }
        total_cost = if min {
            total_cost.min(cur_cost)
        } else {
            total_cost.max(cur_cost)
        };
    }
    total_cost
}

pub fn part_one(input: &str) -> Option<u32> {
    let graph = build_graph(input)?;
    Some(find_route_cost(graph, true))
}

pub fn part_two(input: &str) -> Option<u32> {
    let graph = build_graph(input)?;
    Some(find_route_cost(graph, false))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(605));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(982));
    }
}
