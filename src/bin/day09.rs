use aoc2023::utils::*;
fn calc(line: &Vec<i32>) -> Vec<i32> {
    let mut vec = vec![];
    for i in 0..line.len() - 1 {
        vec.push(line[i + 1] - line[i]);
    }
    vec
}
#[timed::timed]
fn part1(data: &str) -> i32 {
    let mut result = 0;
    let parsed_data = data
        .lines()
        .map(|x| {
            x.split(" ")
                .flat_map(|x| x.parse::<i32>())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    for line in parsed_data {
        let mut answer = line.clone();
        while !answer.iter().all(|x| *x == 0) {
            result += answer.last().unwrap();
            answer = calc(&answer);
        }
    }
    result
}
#[timed::timed]
fn part2(data: &String) -> i32 {
    let mut result = 0;
    let parsed_data = data
        .lines()
        .map(|x| {
            x.split(" ")
                .flat_map(|x| x.parse::<i32>())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    for line in parsed_data {
        let mut i = 0;
        let mut tmp = 0;
        let mut answer = line.clone();
        while !answer.iter().all(|x| *x == 0) {
            if i % 2 == 0 {
                tmp += answer.first().unwrap();
            } else {
                tmp -= answer.first().unwrap();
            }
            answer = calc(&answer);
            i += 1;
        }
        result += tmp;
    }
    result
}

fn main() {
    let data = read_data(9, InputType::NotTest);
    println!("Part 1: {}", part1(&data));
    println!("Part 2: {}", part2(&data));
}
