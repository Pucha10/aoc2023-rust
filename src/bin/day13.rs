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
fn calc_horizontalp1(grid: &Vec<Vec<char>>) -> usize {
    for i in 1..=grid.len() / 2 {
        let mut first_array = grid[..i].to_vec();
        let mut second_array = grid[i..i * 2].to_vec();
        second_array.reverse();
        if first_array == second_array {
            return i;
        }
        first_array = grid[grid.len() - i..].to_vec();
        second_array = grid[grid.len() - 2 * i..grid.len() - i].to_vec();
        second_array.reverse();

        if first_array == second_array {
            return grid.len() - i;
        }
    }
    0
}
#[timed::timed]
fn part1(data: &str) -> usize {
    let mut result = 0;
    let parsed_data = data
        .split("\r\n\r\n")
        .map(|x| {
            x.lines()
                .map(|x| x.chars().collect::<Vec<_>>())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    for mut element in parsed_data {
        result += calc_horizontalp1(&element) * 100;
        element = transpose(&element);
        result += calc_horizontalp1(&element);
    }
    result
}
fn calc_horizontalp2(grid: &Vec<Vec<char>>) -> usize {
    for i in 1..=grid.len() / 2 {
        let mut how_many_mistakes = 0;
        let mut first_array = grid[..i].to_vec();
        let mut second_array = grid[i..i * 2].to_vec();
        second_array.reverse();
        for k in 0..first_array.len() {
            for j in 0..first_array[k].len() {
                if first_array[k][j] != second_array[k][j] {
                    how_many_mistakes += 1;
                }
            }
        }
        if how_many_mistakes == 1 {
            return i;
        }
        how_many_mistakes = 0;
        first_array = grid[grid.len() - i..].to_vec();
        second_array = grid[grid.len() - 2 * i..grid.len() - i].to_vec();
        second_array.reverse();
        for k in 0..first_array.len() {
            for j in 0..first_array[k].len() {
                if first_array[k][j] != second_array[k][j] {
                    how_many_mistakes += 1;
                }
            }
        }
        if how_many_mistakes == 1 {
            return grid.len() - i;
        }
    }
    0
}

#[timed::timed]
fn part2(data: &str) -> usize {
    let mut result = 0;
    let parsed_data = data
        .split("\r\n\r\n")
        .map(|x| {
            x.lines()
                .map(|x| x.chars().collect::<Vec<_>>())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    for mut element in parsed_data {
        result += calc_horizontalp2(&element) * 100;
        element = transpose(&element);
        result += calc_horizontalp2(&element);
    }
    result
}
fn main() {
    let data = read_data(13, InputType::NotTest);
    println!("Part 1: {}", part1(&data));
    println!("Part 2: {}", part2(&data));
}
