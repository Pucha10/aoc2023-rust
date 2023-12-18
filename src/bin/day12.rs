use std::collections::HashMap;

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
fn change_line(line: &Vec<char>, numbers: &Vec<usize>) -> usize {
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
            result += change_line(&line, numbers);
            line[index] = '#';
            result += change_line(&line, numbers);
            line[index] = '?';
            break;
        }
    }
    result
}
fn possible_solution(
    line: &Vec<char>,
    numbers: &Vec<usize>,
    current_symbol: usize,
    current_number: usize,
    hash_counter: usize,
    result_combination: &mut HashMap<(usize, usize, usize), usize>,
) -> usize {
    let mask: (usize, usize, usize) = (current_symbol, current_number, hash_counter);
    let mut result = 0;
    if result_combination.contains_key(&mask) {
        return *result_combination.get(&mask).unwrap();
    }
    if current_number == numbers.len() {
        if line[current_symbol..].iter().all(|&x| x != '#') {
            return 1;
        } else {
            return 0;
        };
    }
    if current_symbol == line.len() {
        if (current_number == numbers.len() - 1 && hash_counter == numbers[current_number])
            || current_number == numbers.len()
        {
            return 1;
        } else {
            return 0;
        }
    }

    if line[current_symbol] == '.' || line[current_symbol] == '?' {
        if hash_counter == 0 || hash_counter == numbers[current_number] {
            result += possible_solution(
                line,
                numbers,
                current_symbol + 1,
                current_number + (hash_counter != 0) as usize,
                0,
                result_combination,
            )
        }
    }
    if line[current_symbol] == '#' || line[current_symbol] == '?' {
        if hash_counter < numbers[current_number] {
            result += possible_solution(
                line,
                numbers,
                current_symbol + 1,
                current_number,
                hash_counter + 1,
                result_combination,
            )
        }
    }
    result_combination.insert(mask, result);
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
        result += change_line(&element.0, &element.1);
    }
    result
}

#[timed::timed]
fn part2(data: &str) -> usize {
    let parsed_data = data
        .lines()
        .map(|x| {
            let x = x.split_once(" ").unwrap();
            let mut line = format!("{}?", x.0).repeat(5).chars().collect::<Vec<_>>();
            line.pop();
            let numbers = format!("{},", x.1)
                .repeat(5)
                .split(',')
                .filter_map(|x| x.parse::<usize>().ok())
                .collect::<Vec<_>>();
            (line, numbers)
        })
        .collect::<Vec<_>>();
    let mut result = 0;
    for element in parsed_data {
        result += possible_solution(&element.0, &element.1, 0, 0, 0, &mut HashMap::new())
    }
    result
}
fn main() {
    let data = read_data(12, InputType::NotTest);
    // println!("Part 1: {}", part1(&data));
    println!("Part 2: {}", part2(&data));
}
