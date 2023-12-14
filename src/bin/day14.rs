use aoc2023::utils::*;

fn rock_on_the_north(mut map: Vec<Vec<char>>) -> Vec<Vec<char>> {
    for _i in 0..map.len() {
        for line_index in 1..map.len() {
            for c_index in 0..map[line_index].len() {
                if map[line_index][c_index] == 'O' && map[line_index - 1][c_index] == '.' {
                    map[line_index - 1][c_index] = 'O';
                    map[line_index][c_index] = '.';
                }
            }
        }
    }
    map
}
fn rock_on_the_east(mut map: Vec<Vec<char>>) -> Vec<Vec<char>> {
    for _i in 0..map.len() {
        for line_index in 0..map.len() {
            for c_index in 0..map[line_index].len() - 1 {
                if map[line_index][c_index] == 'O' && map[line_index][c_index + 1] == '.' {
                    map[line_index][c_index + 1] = 'O';
                    map[line_index][c_index] = '.';
                }
            }
        }
    }
    map
}
fn rock_on_the_south(mut map: Vec<Vec<char>>) -> Vec<Vec<char>> {
    for _i in 0..map.len() {
        for line_index in 0..map.len() - 1 {
            for c_index in 0..map[line_index].len() {
                if map[line_index][c_index] == 'O' && map[line_index + 1][c_index] == '.' {
                    map[line_index + 1][c_index] = 'O';
                    map[line_index][c_index] = '.';
                }
            }
        }
    }
    map
}
fn rock_on_the_west(mut map: Vec<Vec<char>>) -> Vec<Vec<char>> {
    for _i in 0..map.len() {
        for line_index in 0..map.len() {
            for c_index in 1..map[line_index].len() {
                if map[line_index][c_index] == 'O' && map[line_index][c_index - 1] == '.' {
                    map[line_index][c_index - 1] = 'O';
                    map[line_index][c_index] = '.';
                }
            }
        }
    }
    map
}
fn calc(map: &Vec<Vec<char>>) -> usize {
    let mut result = 0;
    for (index, line) in map.iter().enumerate() {
        let mut how_many = 0;
        for c in line {
            if c == &'O' {
                how_many += 1;
            }
        }
        result += how_many * (map.len() - index);
    }
    result
}
#[timed::timed]
fn part1(data: &str) -> usize {
    calc(&rock_on_the_north(
        data.lines()
            .map(|x| x.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>(),
    ))
}

#[timed::timed]
fn part2(data: &str) -> usize {
    let mut parsed_data = data
        .lines()
        .map(|x| x.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let mut results = Vec::new();
    for _i in 0..500 {
        results.push(calc(&parsed_data));
        parsed_data = rock_on_the_north(parsed_data);
        parsed_data = rock_on_the_west(parsed_data);
        parsed_data = rock_on_the_south(parsed_data);
        parsed_data = rock_on_the_east(parsed_data);
    }
    let mut cycle = 0;
    for i in (0..results.len() - 1).rev() {
        if results[i] == results[results.len() - 1] {
            cycle = (results.len() - 1) - i;
            break;
        }
    }
    println!("{}", cycle);
    for i in (0..results.len()).rev() {
        if i % cycle == 1000000000 % cycle {
            return results[i];
        }
    }
    0
}
fn main() {
    let data = read_data(14, InputType::NotTest);
    println!("Part 1: {}", part1(&data));
    println!("Part 2: {}", part2(&data));
}
