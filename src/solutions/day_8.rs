use std::cmp::Reverse;
use std::{
    collections::{BinaryHeap, HashMap},
    fs::read_to_string,
};

use ordered_float::OrderedFloat;

struct UnionFind {
    parent: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        let parent = (0..n).collect();
        UnionFind { parent }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]);
        }

        self.parent[x]
    }

    fn union(&mut self, a: usize, b: usize) {
        let root_a = self.find(a);
        let root_b = self.find(b);

        if root_a != root_b {
            self.parent[root_b] = root_a;
        }
    }

    fn group_total(&mut self) -> i64 {
        let mut counts: HashMap<usize, i64> = HashMap::new();

        for i in 0..self.parent.len() {
            *counts.entry(self.find(i)).or_insert(0) += 1;
        }

        let mut heap = BinaryHeap::new();

        for (_, count) in counts {
            heap.push(-count);

            while heap.len() > 3 {
                heap.pop();
            }
        }

        let product: i64 = heap.iter().product();

        product * -1
    }
}

pub fn solve_day_8_part_1() {
    let text = read_to_string("./data/input/day_8.txt").expect("Could not parse text file");
    let result = part_1(text, 1000);
    println!("Day 8 Part 1: {}", result);
}

fn part_1(text: String, n: usize) -> i64 {
    let mut points: Vec<Vec<f32>> = Vec::new();
    for line in text.trim().lines() {
        let coordinates: Vec<f32> = line.split(",").map(|x| x.trim().parse().unwrap()).collect();
        points.push(coordinates);
    }

    let distance = |point_1: &Vec<f32>, point_2: &Vec<f32>| {
        return ((point_1[0] - point_2[0]).powi(2)
            + (point_1[1] - point_2[1]).powi(2)
            + (point_1[2] - point_2[2]).powi(2))
        .sqrt();
    };

    let mut heap: BinaryHeap<(OrderedFloat<f32>, usize, usize)> = BinaryHeap::new();

    for (i, point) in points.iter().enumerate() {
        for j in (i + 1)..points.len() {
            let point_2 = &points[j];
            let curr_distance = distance(point, point_2);
            heap.push((OrderedFloat(curr_distance), i, j));

            while heap.len() > n {
                heap.pop();
            }
        }
    }

    let mut uf = UnionFind::new(points.len());

    while let Some((_, i, j)) = heap.pop() {
        uf.union(i, j);
    }

    uf.group_total()
}

pub fn solve_day_8_part_2() {
    let text = read_to_string("./data/input/day_8.txt").expect("Could not parse text file");
    let result = part_2(text);
    println!("Day 8 Part 2: {}", result);
}

fn part_2(text: String) -> i64 {
    let mut points: Vec<Vec<f32>> = Vec::new();
    for line in text.trim().lines() {
        let coordinates: Vec<f32> = line.split(",").map(|x| x.trim().parse().unwrap()).collect();
        points.push(coordinates);
    }

    let distance = |point_1: &Vec<f32>, point_2: &Vec<f32>| {
        return ((point_1[0] - point_2[0]).powi(2)
            + (point_1[1] - point_2[1]).powi(2)
            + (point_1[2] - point_2[2]).powi(2))
        .sqrt();
    };

    let mut heap = BinaryHeap::new();

    for (i, point) in points.iter().enumerate() {
        for j in (i + 1)..points.len() {
            let point_2 = &points[j];
            let curr_distance = distance(point, point_2);
            heap.push((Reverse(OrderedFloat(curr_distance)), i, j));
        }
    }

    let mut uf = UnionFind::new(points.len());
    let mut edge_count = 0;
    let mut last_nodes = (0, 0);

    while edge_count < (points.len() - 1) {
        let (_, i, j) = heap.pop().unwrap();
        last_nodes = (i, j);
        if uf.find(i) != uf.find(j) {
            edge_count += 1;
            uf.union(i, j);
        }
    }

    points[last_nodes.0][0] as i64 * points[last_nodes.1][0] as i64
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "
    162,817,812
    57,618,57
    906,360,560
    592,479,940
    352,342,300
    466,668,158
    542,29,236
    431,825,988
    739,650,466
    52,470,668
    216,146,977
    819,987,18
    117,168,530
    805,96,715
    346,949,466
    970,615,88
    941,993,340
    862,61,35
    984,92,344
    425,690,689";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(TEST_INPUT.to_string(), 10), 40);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(TEST_INPUT.to_string()), 25272);
    }
}
