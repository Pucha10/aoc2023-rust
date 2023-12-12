use aoc2023::utils::*;
fn transpose(matrix: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let row_count = matrix.len();
    let col_count = matrix.get(0).map_or(0, |row| row.len());

    let mut transposed = vec![vec![' '; row_count]; col_count];
    for (i, row) in matrix.iter().enumerate() {
        for (j, &val) in row.iter().enumerate() {
            transposed[j][i] = val;
        }
    }

    transposed
}
fn get_universe(data: &str) -> Vec<Vec<char>> {
    let mut grid = data
        .lines()
        .map(|x| x.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let mut universe = Vec::new();
    for row in grid {
        if row.iter().all(|&c| c == '.') {
            universe.push(row.clone());
        }

        universe.push(row.clone());
    }
    grid = transpose(&universe);
    universe.clear();
    for col in grid {
        if col.iter().all(|&c| c == '.') {
            universe.push(col.clone());
        }

        universe.push(col.clone());
    }

    transpose(&universe)
}
fn found_galaxies(universe: Vec<Vec<char>>) -> Vec<(usize, usize)> {
    let mut galaxies = Vec::new();

    universe.iter().enumerate().for_each(|(i, row)| {
        row.iter().enumerate().for_each(|(j, &col)| {
            if col == '#' {
                galaxies.push((i, j));
            }
        });
    });
    galaxies
}
#[timed::timed]
fn part1(data: &str) -> i32 {
    let galaxies = found_galaxies(get_universe(data));
    let mut result = 0;
    for (i, first_galaxy) in galaxies.iter().enumerate() {
        for (_j, second_galaxy) in galaxies.iter().skip(i + 1).enumerate() {
            let distance = (first_galaxy.0 as i32 - second_galaxy.0 as i32).abs()
                + (first_galaxy.1 as i32 - second_galaxy.1 as i32).abs();
            result += distance;
        }
    }
    result
}
fn get_universep2(data: &str) -> Vec<Vec<char>> {
    let mut grid = data
        .lines()
        .map(|x| x.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let mut universe = Vec::new();
    for row in grid {
        if row.iter().all(|&c| c == '.') {
            universe.push(vec!['x'; row.len()]);
        } else {
            universe.push(row.clone());
        }
    }
    grid = transpose(&universe);
    universe.clear();
    for col in grid {
        if col.iter().all(|&c| c == '.' || c == 'x') {
            universe.push(vec!['x'; col.len()]);
        } else {
            universe.push(col.clone());
        }
    }
    transpose(&universe)
}
#[timed::timed]
fn part2(data: &str) -> i64 {
    let universe = get_universep2(data);
    let galaxies = found_galaxies(get_universep2(data));
    let mut result = 0;
    let step_size = 1000000;
    for (i, first_galaxy) in galaxies.iter().enumerate() {
        for (_j, second_galaxy) in galaxies.iter().skip(i + 1).enumerate() {
            let mut steps = 0;
            let mut current_pos = (first_galaxy.0, first_galaxy.1);
            while current_pos.0 != second_galaxy.0 {
                if universe[current_pos.0][current_pos.1] == 'x' {
                    steps += step_size;
                } else {
                    steps += 1;
                }

                if current_pos.0 < second_galaxy.0 {
                    current_pos.0 += 1;
                } else {
                    current_pos.0 -= 1;
                }
            }
            while current_pos.1 != second_galaxy.1 {
                if universe[current_pos.0][current_pos.1] == 'x' {
                    steps += step_size;
                } else {
                    steps += 1;
                }

                if current_pos.1 < second_galaxy.1 {
                    current_pos.1 += 1;
                } else {
                    current_pos.1 -= 1;
                }
            }
            result += steps;
        }
    }
    result
}
fn main() {
    let data = read_data(11, InputType::NotTest);
    println!("Part 1: {}", part1(&data));
    println!("Part 2: {}", part2(&data));
}
