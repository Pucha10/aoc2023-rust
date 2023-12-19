use std::usize;

use aoc2023::utils::*;

fn find_vertices(parsed_data: &Vec<(&str, i64)>) -> Vec<(i64, i64)> {
    let mut vec = vec![];
    let mut x = 0;
    let mut y = 0;
    for element in parsed_data {
        vec.push((x, y));
        if element.0 == "R" {
            x += element.1;
        } else if element.0 == "L" {
            x -= element.1
        } else if element.0 == "U" {
            y -= element.1
        } else if element.0 == "D" {
            y += element.1
        }
    }
    vec
}

#[timed::timed]

fn part1(data: &str) -> i64 {
    let parsed_data = data
        .lines()
        .map(|x| {
            let x = x.split(" ").collect::<Vec<_>>();
            let line = x[0];
            let number = x[1].parse::<i64>().unwrap();
            (line, number)
        })
        .collect::<Vec<_>>();
    let mut b = 0;
    for line in parsed_data.clone() {
        b += line.1 as usize;
    }
    let vertices = find_vertices(&parsed_data);
    let mut area = 0;
    let n = parsed_data.len();
    for i in 0..n {
        let j = (i + 1) % n;
        area += (vertices[i].0 * vertices[j].1) as i64;
        area -= (vertices[i].1 * vertices[j].0) as i64;
    }
    area /= 2;
    area - (b / 2) as i64 + 1 + b as i64
}

#[timed::timed]
fn part2(data: &str) -> i64 {
    let parsed_data = data
        .lines()
        .map(|x| {
            let x = x.split(" ").collect::<Vec<_>>();
            let line = x[2].chars().collect::<Vec<_>>();
            let text = line[2..=6].iter().collect::<String>();
            let number = i64::from_str_radix(&text, 16).unwrap();
            let direction = match line[7] {
                '0' => "R",
                '1' => "D",
                '2' => "L",
                '3' => "U",
                _ => unreachable!(),
            };
            (direction, number)
        })
        .collect::<Vec<_>>();
    let mut b = 0;
    for line in parsed_data.clone() {
        b += line.1 as usize;
    }
    let vertices = find_vertices(&parsed_data);
    let mut area = 0;
    let n = parsed_data.len();

    for i in 0..n {
        let j = (i + 1) % n;
        area += (vertices[i].0 * vertices[j].1) as i64;
        area -= (vertices[i].1 * vertices[j].0) as i64;
    }
    area /= 2;
    area - (b / 2) as i64 + 1 + b as i64
}
fn main() {
    let data = read_data(18, InputType::NotTest);
    println!("Part 1: {}", part1(&data));
    println!("Part 2: {}", part2(&data));
}
