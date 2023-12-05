use aoc2023::utils::*;

#[timed::timed]
fn part1(data: &str) -> i64 {
    let mut parsed_data = data.split("\r\n\r\n");
    let mut seeds = parsed_data
        .next()
        .unwrap()
        .split(" ")
        .map(|x| x.parse::<i64>().unwrap())
        .collect::<Vec<_>>();
    let parsed_data = parsed_data
        .map(|x| {
            x.lines()
                .skip(1)
                .map(|x| {
                    x.split(" ")
                        .map(|x| x.parse::<i64>().unwrap())
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    for soil_type in parsed_data {
        for j in 0..seeds.len() {
            for i in 0..soil_type.len() {
                if seeds[j] >= soil_type[i][1] && seeds[j] <= soil_type[i][1] + soil_type[i][2] - 1
                {
                    seeds[j] -= soil_type[i][1] - soil_type[i][0];
                    break;
                }
            }
        }
    }
    seeds.into_iter().min().unwrap()
}

#[timed::timed]
fn part2(data: &String) -> i64 {
    let mut parsed_data = data.split("\r\n\r\n");
    let mut seeds = parsed_data
        .next()
        .unwrap()
        .split(" ")
        .map(|x| x.parse::<i64>().unwrap())
        .collect::<Vec<_>>();
    let parsed_data = parsed_data
        .map(|x| {
            x.lines()
                .skip(1)
                .map(|x| {
                    x.split(" ")
                        .map(|x| x.parse::<i64>().unwrap())
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let mut result = i64::MAX;
    for k in (1..seeds.len()).step_by(2) {
        for _l in 0..seeds[k] {
            let mut tmp = seeds[k - 1];
            for soil_type in &parsed_data {
                for i in 0..soil_type.len() {
                    if tmp >= soil_type[i][1] && tmp <= soil_type[i][1] + soil_type[i][2] - 1 {
                        tmp -= soil_type[i][1] - soil_type[i][0];
                        break;
                    }
                }
            }
            if result > tmp {
                result = tmp;
            }
            seeds[k - 1] += 1;
        }
    }
    result
}

fn main() {
    let data = read_data(5, InputType::NotTest);
    println!("Part 1: {}", part1(&data));
    println!("Part 2: {}", part2(&data));
}
