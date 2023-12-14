use aoc2023::utils::*;
fn check(line: &Vec<char>, numbers: &Vec<usize>) -> bool {
    let mut flag = true;
    let mut how_many_hash = vec![];
    for c in line {
        if c == &'.' {
            flag = true;
            continue;
        } else {
            if flag {
                how_many_hash.push(0);
            }
        }
        *how_many_hash.last_mut().unwrap() += 1;
        flag = false;
    }
    if how_many_hash.len() != numbers.len() {
        return false;
    }
    for (index, element) in how_many_hash.iter().enumerate() {
        if element != &numbers[index] {
            return false;
        }
    }
    true
}
fn change_symbols(line: &Vec<char>, numbers: &Vec<usize>) -> usize {
    let mut result = 0;
    if line.iter().filter(|&&x| x != '.').count() < numbers.into_iter().sum() {
        return 0;
    }
    if !line.contains(&'?') {
        if check(&line, numbers) {
            return 1;
        } else {
            return 0;
        }
    }
    let mut line = line.clone();
    for (index, c) in line.clone().iter_mut().enumerate() {
        if c == &'?' {
            line[index] = '.';
            result += change_symbols(&line, numbers);
            line[index] = '#';
            result += change_symbols(&line, numbers);
            line[index] = '?';
            break;
        }
    }
    result
}
#[timed::timed]
fn part1(data: &str) -> usize {
    let parsed_data = data
        .lines()
        .map(|x| {
            let x = x.split_once(' ').unwrap();
            (
                x.0.chars().collect::<Vec<_>>(),
                x.1.split(',')
                    .flat_map(|x| x.parse::<usize>())
                    .collect::<Vec<_>>(),
            )
        })
        .collect::<Vec<_>>();
    let mut result = 0;
    for element in parsed_data {
        result += change_symbols(&element.0, &element.1);
    }
    result
}

// #[timed::timed]
// fn part2(data: &str) -> i64 {

// }
fn main() {
    let data = read_data(12, InputType::NotTest);
    println!("Part 1: {}", part1(&data));
    // println!("Part 2: {}", part2(&data));
}
