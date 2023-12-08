use aoc2023::utils::*;
use num_integer::lcm;
use std::collections::HashMap;
fn find_steps(start: &str, instructions: &str, nodes: &HashMap<&str, (&str, &str)>) -> u128 {
    let mut current_node = start;
    let mut steps = 0;
    for instruction in instructions.chars().cycle() {
        match instruction {
            'L' => current_node = nodes.get(current_node).unwrap().0,
            'R' => current_node = nodes.get(current_node).unwrap().1,
            _ => unreachable!(),
        }

        steps += 1;
        if current_node.chars().nth(2).unwrap() == 'Z' {
            break;
        }
    }
    steps
}
#[timed::timed]
fn part1(data: &str) -> u128 {
    let mut lines = data.lines();
    let instructions = lines.next().unwrap();
    lines.next();
    let mut nodes: HashMap<&str, (&str, &str)> = HashMap::new();
    for line in lines.clone() {
        let parts: Vec<&str> = line.split(" = ").collect();
        let node = parts[0];
        let directions: Vec<&str> = parts[1].split(", ").collect();
        let left = directions[0].trim_matches(|c| c == '(' || c == ')');
        let right = directions[1].trim_matches(|c| c == '(' || c == ')');
        nodes.insert(node, (left, right));
    }
    find_steps("AAA", instructions, &nodes)
}

#[timed::timed]
fn part2(data: &String) -> u128 {
    let mut lines = data.lines();
    let instructions = lines.next().unwrap();
    lines.next();
    let mut nodes: HashMap<&str, (&str, &str)> = HashMap::new();
    for line in lines.clone() {
        let parts: Vec<&str> = line.split(" = ").collect();
        let node = parts[0];
        let directions: Vec<&str> = parts[1].split(", ").collect();
        let left = directions[0].trim_matches(|c| c == '(' || c == ')');
        let right = directions[1].trim_matches(|c| c == '(' || c == ')');
        nodes.insert(node, (left, right));
    }
    let mut result = 1;
    let starts: Vec<&str> = nodes
        .keys()
        .filter(|&node| node.ends_with('A'))
        .cloned()
        .collect();
    for i in 0..starts.len() {
        result = lcm(find_steps(starts[i], instructions, &nodes), result)
    }
    result
}

fn main() {
    let data = read_data(8, InputType::NotTest);
    println!("Part 1: {}", part1(&data));
    println!("Part 2: {}", part2(&data));
}
