use aoc2023::utils::*;
fn word_to_number(word: &str) -> i32 {
    match word {
        "one" => 1,
        "two" => 2,
        "three" => 3,
        "four" => 4,
        "five" => 5,
        "six" => 6,
        "seven" => 7,
        "eight" => 8,
        "nine" => 9,
        _ => 0,
    }
}
#[timed::timed]
fn part1(data: &str) -> i32 {
    1
}

#[timed::timed]
fn part2(data: &String) -> i32 {
    1
}

fn main() {
    let data = read_data(1, InputType::NotTest);
    println!("Part 1: {}", part1(&data));
    println!("Part 2: {}", part2(&data));
}
