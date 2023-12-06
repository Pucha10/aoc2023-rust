use aoc2023::utils::*;

#[timed::timed]
fn part1(data: &str) -> i32 {
    let times = data.lines().collect::<Vec<_>>()[0];
    let distances = data.lines().collect::<Vec<_>>()[1];
    let times = times
        .split("Time: ")
        .skip(1)
        .map(|x| {
            x.split(" ")
                .map(|x| x.parse::<i32>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let distances = distances
        .split("Distance: ")
        .skip(1)
        .map(|x| {
            x.split(" ")
                .map(|x| x.parse::<i32>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let mut result = 1;
    for i in 0..times[0].len() {
        let mut how_many_ways_to_win = 0;
        for j in 0..times[0][i] {
            if j * (times[0][i] - j) > distances[0][i] {
                how_many_ways_to_win += 1;
            }
        }
        result *= how_many_ways_to_win;
    }

    result
}

#[timed::timed]
fn part2(data: &String) -> i32 {
    let times_str: String = data.lines().collect::<Vec<_>>()[0]
        .split("Time: ")
        .skip(1)
        .flat_map(|x| x.split_whitespace())
        .collect();

    let time: i64 = times_str.parse().unwrap();

    let distances_str: String = data.lines().collect::<Vec<_>>()[1]
        .split("Distance: ")
        .skip(1)
        .flat_map(|x| x.split_whitespace())
        .collect();

    let distance: i64 = distances_str.parse().unwrap();
    let mut how_many_ways = 0;
    for i in 0..time {
        if distance < i * (time - i) {
            how_many_ways += 1;
        }
    }
    how_many_ways
}

fn main() {
    let data = read_data(6, InputType::NotTest);
    println!("Part 1: {}", part1(&data));
    println!("Part 2: {}", part2(&data));
}
