use std::{
    collections::{BinaryHeap, HashSet},
    str::FromStr,
};

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
struct Coordinate {
    x: usize,
    y: usize,
}

impl Coordinate {
    fn neighbours(&self, rows: usize, cols: usize) -> Vec<Self> {
        let mut neighbours = vec![];

        if self.y > 0 {
            neighbours.push(Self {
                x: self.x,
                y: self.y - 1,
            });
        }

        if self.y < rows - 1 {
            neighbours.push(Self {
                x: self.x,
                y: self.y + 1,
            });
        }

        if self.x > 0 {
            neighbours.push(Self {
                x: self.x - 1,
                y: self.y,
            });
        }

        if self.x < cols - 1 {
            neighbours.push(Self {
                x: self.x + 1,
                y: self.y,
            });
        }

        neighbours
    }
}

#[derive(Debug)]
struct Area {
    heightmap: Vec<Vec<u8>>,
    rows: usize,
    cols: usize,
    start: Coordinate,
    end: Coordinate,
    start_candidates: Vec<Coordinate>,
}

impl FromStr for Area {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let rows = s.lines().count();
        let cols = s.lines().next().unwrap().len();

        let mut heightmap = vec![vec![0; cols]; rows];
        let mut start = Coordinate { x: 0, y: 0 };
        let mut end = Coordinate { x: 0, y: 0 };
        let mut start_candidates = vec![];

        for (row, line) in s.lines().enumerate() {
            for (col, c) in line.chars().enumerate() {
                let letter = match c {
                    'a'..='z' => c,
                    'S' => {
                        start = Coordinate { x: col, y: row };
                        'a'
                    }
                    'E' => {
                        end = Coordinate { x: col, y: row };
                        'z'
                    }
                    _ => unreachable!("Invalid input"),
                };

                let height = letter as u8 - b'a';

                if height == 0 {
                    start_candidates.push(Coordinate { x: col, y: row });
                }

                heightmap[row][col] = letter as u8 - b'a';
            }
        }

        Ok(Area {
            heightmap,
            rows,
            cols,
            start,
            end,
            start_candidates,
        })
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Node {
    cost: usize,
    coordinate: Coordinate,
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl Area {
    fn shortest_path(&self) -> usize {
        let mut costs = vec![];
        let mut start_points = self.start_candidates.clone();

        while let Some(start_point) = start_points.pop() {
            costs.push(self.shortest_path_for(start_point));
        }

        let smallest_cost = costs
            .iter()
            .filter(|&c| c.is_some())
            .map(|c| c.unwrap())
            .min()
            .unwrap();

        smallest_cost
    }

    fn shortest_path_for(&self, start_point: Coordinate) -> Option<usize> {
        let mut priority_queue = BinaryHeap::new();
        let mut visited_nodes = HashSet::new();

        priority_queue.push(Node {
            cost: 0,
            coordinate: start_point,
        });
        visited_nodes.insert(start_point);

        while let Some(Node { coordinate, cost }) = priority_queue.pop() {
            if coordinate == self.end {
                return Some(cost);
            }

            let current_height = self.heightmap[coordinate.y][coordinate.x];
            let neighbours = coordinate.neighbours(self.rows, self.cols);
            let candidates: Vec<_> = neighbours
                .iter()
                .filter(|c| {
                    let height = self.heightmap[c.y][c.x];
                    height <= current_height || height == current_height + 1
                })
                .collect();

            for candidate in candidates {
                if visited_nodes.insert(*candidate) {
                    priority_queue.push(Node {
                        cost: cost + 1,
                        coordinate: *candidate,
                    });
                }
            }
        }

        None
    }
}

fn main() -> Result<(), anyhow::Error> {
    let area = include_str!("../inputs/day12.txt").parse::<Area>()?;
    let steps = area.shortest_path();

    println!("Got to the end in {} steps ", steps);
    Ok(())
}
