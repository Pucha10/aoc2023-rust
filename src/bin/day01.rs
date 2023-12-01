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
    let parsed_data: Vec<&str> = data.lines().collect();
    let mut first_number = 0;
    let mut second_number = 0;
    let mut result = 0;

    for element in parsed_data {
        let mut is_first = true;
        for i in 0..element.len() {
            if element.chars().nth(i).unwrap() >= '0'
                && element.chars().nth(i).unwrap() <= '9'
                && is_first
            {
                first_number = element.chars().nth(i).unwrap().to_digit(10).unwrap() as i32;
                is_first = false;
            } else if element.chars().nth(i).unwrap() >= '0'
                && element.chars().nth(i).unwrap() <= '9'
            {
                second_number = element.chars().nth(i).unwrap().to_digit(10).unwrap() as i32;
            }
        }
        if second_number == 0 {
            result += first_number * 10 + first_number;
        } else {
            result += first_number * 10 + second_number;
        }
        first_number = 0;
        second_number = 0;
    }
    result
}

#[timed::timed]
fn part2(data: &String) -> i32 {
    let parsed_data: Vec<&str> = data.lines().collect();
    let mut result = 0;

    for element in parsed_data {
        let mut three_letters = String::new();
        let mut four_letters = String::new();
        let mut five_letters = String::new();
        let mut is_first = true;
        let mut first_number = 0;
        let mut second_number = 0;
        for c in element.chars() {
            if three_letters.len() < 3 {
                three_letters.push(c);
            } else {
                three_letters = format!("{}{}", &three_letters[1..], c);
            }
            if four_letters.len() < 4 {
                four_letters.push(c);
            } else {
                four_letters = format!("{}{}", &four_letters[1..], c);
            }
            if five_letters.len() < 5 {
                five_letters.push(c)
            } else {
                five_letters = format!("{}{}", &five_letters[1..], c);
            }
            if c >= '0' && c <= '9' && is_first {
                first_number = c as i32 - 48;
                is_first = false;
            } else if c >= '0' && c <= '9' {
                second_number = c as i32 - 48;
            } else if word_to_number(&three_letters) > 0
                && word_to_number(&three_letters) <= 9
                && is_first
            {
                first_number = word_to_number(&three_letters);
                is_first = false;
            } else if word_to_number(&three_letters) > 0 && word_to_number(&three_letters) <= 9 {
                second_number = word_to_number(&three_letters);
            } else if word_to_number(&four_letters) > 0
                && word_to_number(&four_letters) <= 9
                && is_first
            {
                first_number = word_to_number(&four_letters);
                is_first = false;
            } else if word_to_number(&four_letters) > 0 && word_to_number(&four_letters) <= 9 {
                second_number = word_to_number(&four_letters);
            } else if word_to_number(&five_letters) > 0
                && word_to_number(&five_letters) <= 9
                && is_first
            {
                first_number = word_to_number(&five_letters);
                is_first = false;
            } else if word_to_number(&five_letters) > 0 && word_to_number(&five_letters) <= 9 {
                second_number = word_to_number(&five_letters);
            }
        }
        if second_number == 0 {
            result += first_number * 10 + first_number;
        } else {
            result += first_number * 10 + second_number;
        }
    }
    result
}

fn main() {
    let data = read_data(1, InputType::NotTest);
    println!("Part 1: {}", part1(&data));
    println!("Part 2: {}", part2(&data));
}
