use aoc2023::utils::*;
use std::collections::HashSet;
fn directions(
    parsed_data: &Vec<Vec<char>>,
    (x, y): (usize, usize),
) -> ((usize, usize), (usize, usize)) {
    if y >= parsed_data.len() || x >= parsed_data[0].len() {
        return ((0, 0), (0, 0));
    }
    let item = parsed_data[y][x];
    match item {
        '|' => ((x, y.wrapping_sub(1)), (x, y + 1)),
        '-' => ((x.wrapping_sub(1), y), (x + 1, y)),
        'L' => ((x, y.wrapping_sub(1)), (x + 1, y)),
        'J' => ((x.wrapping_sub(1), y), (x, y.wrapping_sub(1))),
        '7' => ((x.wrapping_sub(1), y), (x, y + 1)),
        'F' => ((x, y + 1), (x + 1, y)),
        '.' => ((0, 0), (0, 0)),
        'S' => ((0, 0), (0, 0)),
        _ => unreachable!(),
    }
}

fn calculate_pipes(parsed_data: &Vec<Vec<char>>) -> Vec<(usize, usize)> {
    let mut start = (0, 0);
    for y in 0..parsed_data.len() {
        for x in 0..parsed_data[0].len() {
            if parsed_data[y][x] == 'S' {
                start = (x, y);
            }
        }
    }
    let mut current = start;
    let neighbors = vec![
        (current.0, current.1),
        (current.0 + 1, current.1),
        (current.0, current.1),
        (current.0, current.1 + 1),
    ];
    for i in neighbors {
        if i == start {
            continue;
        }
        let tmp = directions(parsed_data, i);
        match tmp {
            (c1, c2) => {
                if c1 == current || c2 == current {
                    current = i;
                    break;
                }
            }
        };
    }
    let mut pipes: Vec<(usize, usize)> = vec![start];
    while parsed_data[current.1][current.0] != 'S' {
        let (c1, c2) = directions(parsed_data, current);
        let next;
        if c1 == *pipes.last().unwrap() {
            next = c2;
        } else {
            next = c1;
        }
        pipes.push(current);
        current = next;
    }
    pipes
}
fn mark_area(
    parsed_data: Vec<Vec<char>>,
    current: (usize, usize),
    pipes: &HashSet<(usize, usize)>,
) -> Vec<Vec<char>> {
    if current.1 >= parsed_data.len() || current.0 >= parsed_data[0].len() {
        return parsed_data;
    }
    if parsed_data[current.1][current.0] == 'I' {
        return parsed_data;
    }
    if pipes.contains(&current) {
        return parsed_data;
    }
    let neighbors = vec![
        (current.0.wrapping_sub(1), current.1),
        (current.0 + 1, current.1),
        (current.0, current.1.wrapping_sub(1)),
        (current.0, current.1 + 1),
    ];
    let mut new_map = parsed_data;
    new_map[current.1][current.0] = 'I';
    for n in neighbors {
        new_map = mark_area(new_map, n, pipes);
    }
    new_map
}

#[timed::timed]
fn part1(data: &str) -> usize {
    let mut parsed_data: Vec<Vec<char>> = data
        .lines()
        .collect::<Vec<_>>()
        .iter()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .map(|line| ".".to_owned() + line + ".")
        .map(|line| line.chars().collect())
        .collect();
    parsed_data.insert(
        0,
        std::iter::repeat('.').take(parsed_data[0].len()).collect(),
    );
    parsed_data.push(std::iter::repeat('.').take(parsed_data[0].len()).collect());
    calculate_pipes(&parsed_data).len() / 2
}
#[timed::timed]
fn part2(data: &str) -> usize {
    let mut parsed_data: Vec<Vec<char>> = data
        .lines()
        .collect::<Vec<_>>()
        .iter()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .map(|line| ".".to_owned() + line + ".")
        .map(|line| line.chars().collect())
        .collect();
    parsed_data.insert(
        0,
        std::iter::repeat('.').take(parsed_data[0].len()).collect(),
    );
    parsed_data.push(std::iter::repeat('.').take(parsed_data[0].len()).collect());
    let mut start = (0, 0);
    let tmp = parsed_data.clone();
    for y in 0..parsed_data.len() {
        for x in 0..parsed_data[0].len() {
            if parsed_data[y][x] == 'S' {
                start = (x, y);
            }
        }
    }
    let pipes = calculate_pipes(&parsed_data);
    let mut previous = (start.0 as i64, start.1 as i64);
    let mut answer_vec = vec![];
    for i in 0..pipes.len() {
        let pipe = pipes[i];
        let current = (pipe.0 as i64, pipe.1 as i64);
        match (current.0 - previous.0, current.1 - previous.1) {
            (-1, 0) => {
                answer_vec.push((pipe.0, pipe.1.wrapping_sub(1)));
                answer_vec.push((pipe.0 + 1, pipe.1.wrapping_sub(1)));
            }
            (0, -1) => {
                answer_vec.push((pipe.0 + 1, pipe.1));
                answer_vec.push((pipe.0 + 1, pipe.1 + 1));
            }
            (1, 0) => {
                answer_vec.push((pipe.0, pipe.1 + 1));
                answer_vec.push((pipe.0.wrapping_sub(1), pipe.1 + 1));
            }
            (0, 1) => {
                answer_vec.push((pipe.0.wrapping_sub(1), pipe.1.wrapping_sub(1)));
                answer_vec.push((pipe.0.wrapping_sub(1), pipe.1));
            }

            _ => {}
        }
        previous = current;
    }
    for point in answer_vec {
        parsed_data = mark_area(
            parsed_data,
            point,
            &pipes.clone().into_iter().collect::<HashSet<_>>(),
        );
    }
    let mut ile = 0;
    for line in parsed_data {
        for c in line {
            if c == 'I' {
                ile += 1;
            }
        }
    }
    tmp.len() * tmp[0].len() - ile - pipes.len()
}
fn main() {
    let data = read_data(10, InputType::Test);
    println!("Part 1: {}", part1(&data));
    println!("Part 2: {}", part2(&data));
}
