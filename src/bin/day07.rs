use aoc2023::utils::*;
use std::cmp::Ordering;
use std::collections::HashMap;

fn rank_type(cards: &str) -> u8 {
    let card_count = cards.chars().fold(HashMap::new(), |mut counts, card| {
        *counts.entry(card).or_insert(0) += 1;
        counts
    });
    let min_count = card_count.values().min().unwrap();
    let max_count = card_count.values().max().unwrap();
    let type_count = card_count.len();
    match (type_count, min_count, max_count) {
        (1, _, _) => 7,
        (2, 1, 4) => 6,
        (2, 2, 3) => 5,
        (3, 1, 3) => 4,
        (3, 1, 2) => 3,
        (4, _, _) => 2,
        (5, _, _) => 1,
        _ => unreachable!(),
    }
}
fn get_card_value(card: char) -> u8 {
    match card {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'J' => 11,
        'T' => 10,
        'Y' => 0,
        _ => card.to_digit(10).unwrap() as u8,
    }
}
#[timed::timed]
fn part1(data: &str) -> i32 {
    let mut hands = data
        .lines()
        .map(|x| {
            let tmp = x.split(" ").collect::<Vec<_>>();
            (tmp[0], tmp[1].parse::<i32>().unwrap(), rank_type(tmp[0]))
        })
        .collect::<Vec<_>>();
    hands.sort_by(|a, b| {
        if a.2.cmp(&b.2) == Ordering::Equal {
            let charsa = a.0.chars().collect::<Vec<_>>();
            let charsb = b.0.chars().collect::<Vec<_>>();
            for i in 0..5 {
                if get_card_value(charsa[i]) != get_card_value(charsb[i]) {
                    return get_card_value(charsa[i]).cmp(&get_card_value(charsb[i]));
                }
            }
            unreachable!();
        } else {
            a.2.cmp(&b.2)
        }
    });
    hands
        .into_iter()
        .enumerate()
        .fold(0, |acc, (index, value)| acc + (index + 1) as i32 * value.1)
}
fn rank_type_part2(cards: &str) -> u8 {
    let mut result = 0;
    for i in "AKQT98765432".chars() {
        let tmp = cards.replace("Y", &i.to_string());
        let card_count = tmp.chars().fold(HashMap::new(), |mut counts, card| {
            *counts.entry(card).or_insert(0) += 1;
            counts
        });
        let min_count = card_count.values().min().unwrap();
        let max_count = card_count.values().max().unwrap();
        let type_count = card_count.len();
        result = result.max(match (type_count, min_count, max_count) {
            (1, _, _) => 7,
            (2, 1, 4) => 6,
            (2, 2, 3) => 5,
            (3, 1, 3) => 4,
            (3, 1, 2) => 3,
            (4, _, _) => 2,
            (5, _, _) => 1,
            _ => unreachable!(),
        });
    }
    result
}
#[timed::timed]
fn part2(data: &String) -> i32 {
    let mut hands = data
        .lines()
        .map(|x| {
            let tmp = x.split(" ").collect::<Vec<_>>();
            let hand = tmp[0].replace("J", "Y");
            (
                hand.clone(),
                tmp[1].parse::<i32>().unwrap(),
                rank_type_part2(&hand),
            )
        })
        .collect::<Vec<_>>();
    hands.sort_by(|a, b| {
        if a.2.cmp(&b.2) == Ordering::Equal {
            let charsa = a.0.chars().collect::<Vec<_>>();
            let charsb = b.0.chars().collect::<Vec<_>>();
            for i in 0..5 {
                if get_card_value(charsa[i]) != get_card_value(charsb[i]) {
                    return get_card_value(charsa[i]).cmp(&get_card_value(charsb[i]));
                }
            }
            unreachable!();
        } else {
            a.2.cmp(&b.2)
        }
    });
    hands
        .into_iter()
        .enumerate()
        .fold(0, |acc, (index, value)| acc + (index + 1) as i32 * value.1)
}

fn main() {
    let data = read_data(7, InputType::NotTest);
    println!("Part 1: {}", part1(&data));
    println!("Part 2: {}", part2(&data));
}
