use aoc2023::utils::*;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};
#[derive(Copy, Clone, Eq, PartialEq)]
struct Graph {
    cost: u32,
    position: (usize, usize),
    consecutive_steps: usize,
    last_direction: Option<(i32, i32)>,
}

impl Ord for Graph {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

impl PartialOrd for Graph {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn dijkstra(graph: &Vec<Vec<u32>>, start: (usize, usize), end: (usize, usize)) -> Option<u32> {
    let rows = graph.len();
    let cols = graph[0].len();
    let mut heap = BinaryHeap::new();
    let mut visited = HashSet::new();
    let mut cost_map = HashMap::new();

    heap.push(Graph {
        cost: 0,
        position: start,
        consecutive_steps: 0,
        last_direction: None,
    });

    while let Some(Graph {
        cost,
        position,
        consecutive_steps,
        last_direction,
    }) = heap.pop()
    {
        cost_map.insert(position, cost);
        if position == end {
            return Some(cost);
        }

        if visited.contains(&position) {
            continue;
        }

        visited.insert(position);

        let (x, y) = position;

        for (dx, dy) in &[(0, 1), (1, 0), (0, -1), (-1, 0)] {
            let new_x = x as i32 + dx;
            let new_y = y as i32 + dy;

            if new_x >= 0 && new_x < rows as i32 && new_y >= 0 && new_y < cols as i32 {
                let new_position = (new_x as usize, new_y as usize);
                let new_direction = Some((*dx, *dy));
                if let Some(last) = last_direction {
                    let diff = (last.0 + dx, last.1 + dy);
                    if diff == (0, 0) {
                        continue;
                    }
                }

                let new_consecutive_steps = if new_direction != last_direction {
                    1
                } else {
                    consecutive_steps + 1
                };

                if new_consecutive_steps <= 3 {
                    let step_cost = graph[new_x as usize][new_y as usize];
                    let new_cost = cost + step_cost;

                    if !visited.contains(&new_position) {
                        heap.push(Graph {
                            cost: new_cost,
                            position: new_position,
                            consecutive_steps: new_consecutive_steps,
                            last_direction: new_direction,
                        });
                    }
                }
            }
        }
    }

    None
}

#[timed::timed]
fn part1(data: &str) -> usize {
    let map = data
        .lines()
        .map(|x| {
            x.chars()
                .map(|x| x.to_digit(10).unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    for i in 0..map.len() {
        for j in 0..map[0].len() {
            print!("{} \t", dijkstra(&map, (0, 0), (j, i)).unwrap());
        }
        println!();
    }
    dijkstra(&map, (0, 0), (map.len() - 1, map[0].len() - 1)).unwrap() as usize
}

// #[timed::timed]
// fn part2(data: &str) -> u32 {

// }
fn main() {
    let data = read_data(17, InputType::Test);
    println!("Part 1: {}", part1(&data)); //this is not working
                                          // println!("Part 2: {}", part2(&data));
}
