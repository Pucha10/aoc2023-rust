use aoc2023::utils::*;

#[timed::timed]
fn part1(data: &str) -> i32 {
    let lines: Vec<&str> = data.lines().collect();
    let mut possible_ids = Vec::new();

    for line in lines {
        let parts: Vec<&str> = line.split(": ").collect();
        let cubes_info: Vec<&str> = parts[1].split("; ").collect();
        let mut possible = true;

        for cubes_set in cubes_info {
            let cubes: Vec<&str> = cubes_set.split(", ").collect();

            for cube in cubes {
                let cube_info: Vec<&str> = cube.split_whitespace().collect();
                let count = cube_info[0].parse::<i32>().unwrap_or(0);

                match cube_info[1] {
                    "red" => {
                        if count > 12 {
                            possible = false;
                            break;
                        }
                    }
                    "green" => {
                        if count > 13 {
                            possible = false;
                            break;
                        }
                    }
                    "blue" => {
                        if count > 14 {
                            possible = false;
                            break;
                        }
                    }
                    _ => {}
                }
            }
            if !possible {
                break;
            }
        }
        if possible {
            let game_id = parts[0]
                .trim_start_matches("Game ")
                .parse::<i32>()
                .unwrap_or(0);
            possible_ids.push(game_id);
        }
    }
    possible_ids.iter().sum()
}

#[timed::timed]
fn part2(data: &String) -> i32 {
    let lines: Vec<&str> = data.lines().collect();
    let mut result = 0;
    for line in lines {
        let parts: Vec<&str> = line.split(": ").collect();
        let cubes_info: Vec<&str> = parts[1].split("; ").collect();
        let mut max_red = 0;
        let mut max_green = 0;
        let mut max_blue = 0;
        for cubes_set in cubes_info {
            let cubes: Vec<&str> = cubes_set.split(", ").collect();
            for cube in cubes {
                let cube_info: Vec<&str> = cube.split_whitespace().collect();
                let count = cube_info[0].parse::<i32>().unwrap_or(0);
                match cube_info[1] {
                    "red" => {
                        if count > max_red {
                            max_red = count;
                        }
                    }
                    "green" => {
                        if count > max_green {
                            max_green = count;
                        }
                    }
                    "blue" => {
                        if count > max_blue {
                            max_blue = count
                        }
                    }
                    _ => {}
                }
            }
        }
        result += max_blue * max_green * max_red;
    }
    result
}

fn main() {
    let data = read_data(2, InputType::NotTest);
    println!("Part 1: {}", part1(&data));
    println!("Part 2: {}", part2(&data));
}
