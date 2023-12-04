use aoc2023::utils::*;
#[timed::timed]
fn part1(data: &str) -> i32 {
    let parsed_data = data
        .lines()
        .map(|x| {
            x.split(": ")
                .last()
                .unwrap()
                .split(" | ")
                .map(|x| {
                    x.split(" ")
                        .map(|x| x.parse::<i32>().unwrap())
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let mut result = 0;
    for line in parsed_data {
        let mut how_many_wins = 0;
        for i in 0..line[0].len() {
            if line[1].contains(&line[0][i]) {
                how_many_wins += 1
            }
        }
        if how_many_wins > 0 {
            result += 2_i32.pow(how_many_wins - 1);
        }
    }
    result
}

#[timed::timed]
fn part2(data: &String) -> usize {
    let parsed_data = data
        .lines()
        .map(|x| {
            x.split(": ")
                .last()
                .unwrap()
                .split(" | ")
                .map(|x| {
                    x.split(" ")
                        .map(|x| x.parse::<i32>().unwrap())
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let mut scratchcards: Vec<i32> = vec![1; parsed_data.len()];
    let mut how_many_wins = 0;
    for (line_index, line) in parsed_data.iter().enumerate() {
        for i in 0..line[0].len() {
            if line[1].contains(&line[0][i]) {
                how_many_wins += 1
            }
        }
        if how_many_wins > 0 {
            for i in line_index + 1..=line_index + how_many_wins {
                scratchcards[i] += 1 * scratchcards[line_index];
            }
        }
        how_many_wins = 0;
    }
    scratchcards.iter().map(|&x| x as usize).sum()
}

fn main() {
    let data = read_data(4, InputType::NotTest);
    println!("Part 1: {}", part1(&data));
    println!("Part 2: {}", part2(&data));
}
