use aoc2023::utils::*;

fn change_matrix(
    matrix: Vec<Vec<char>>,
    position: (isize, isize),
    laser_dir: char,
) -> Vec<Vec<char>> {
    if position.0 < 0
        || position.0 >= matrix.len() as isize
        || position.1 < 0
        || position.1 >= matrix.len() as isize
    {
        return matrix;
    }

    if matrix[position.0 as usize][position.1 as usize] == '#' {
        return matrix;
    }
    let mut position = position;
    let mut laser_dir = laser_dir;
    let mut matrix = matrix.clone();
    let sign = matrix[position.0 as usize][position.1 as usize];
    match sign {
        '.' => {
            matrix[position.0 as usize][position.1 as usize] = laser_dir;
            let (dx, dy) = get_direction(laser_dir);
            position.0 += dy;
            position.1 += dx;
            matrix = change_matrix(matrix, position, laser_dir);
        }
        '/' | '\\' | '!' | '$' => {
            if sign == '/' {
                matrix[position.0 as usize][position.1 as usize] = '!';
            } else {
                matrix[position.0 as usize][position.1 as usize] = '$';
            }
            laser_dir = reflect_direction(laser_dir, sign);
            let (dx, dy) = get_direction(laser_dir);
            position.0 += dy;
            position.1 += dx;

            matrix = change_matrix(matrix, position, laser_dir);
        }
        '|' => {
            matrix[position.0 as usize][position.1 as usize] = '#';
            if laser_dir == 'v' {
                laser_dir = 'v';
                let (dx, dy) = get_direction(laser_dir);
                matrix = change_matrix(matrix, (position.0 + dy, position.1 + dx), laser_dir);
            } else if laser_dir == '^' {
                laser_dir = '^';
                let (dx, dy) = get_direction(laser_dir);
                matrix = change_matrix(matrix, (position.0 + dy, position.1 + dx), laser_dir);
            } else {
                laser_dir = 'v';
                let (dx, dy) = get_direction(laser_dir);
                matrix = change_matrix(matrix, (position.0 + dy, position.1 + dx), laser_dir);
                laser_dir = '^';
                let (dx, dy) = get_direction(laser_dir);
                matrix = change_matrix(matrix, (position.0 + dy, position.1 + dx), laser_dir);
            }
        }
        '-' => {
            matrix[position.0 as usize][position.1 as usize] = '#';
            if laser_dir == '>' {
                laser_dir = '>';
                let (dx, dy) = get_direction(laser_dir);
                matrix = change_matrix(matrix, (position.0 + dy, position.1 + dx), laser_dir);
            } else if laser_dir == '<' {
                laser_dir = '<';
                let (dx, dy) = get_direction(laser_dir);
                matrix = change_matrix(matrix, (position.0 + dy, position.1 + dx), laser_dir);
            } else {
                laser_dir = '>';
                let (dx, dy) = get_direction(laser_dir);
                matrix = change_matrix(matrix, (position.0 + dy, position.1 + dx), laser_dir);
                laser_dir = '<';
                let (dx, dy) = get_direction(laser_dir);
                matrix = change_matrix(matrix, (position.0 + dy, position.1 + dx), laser_dir);
            }
        }
        '>' | '<' | '^' | 'v' => {
            matrix[position.0 as usize][position.1 as usize] = '2';
            let (dx, dy) = get_direction(laser_dir);
            position.0 += dy;
            position.1 += dx;
            matrix = change_matrix(matrix, position, laser_dir);
        }

        _ => {
            let mut parsed_num = sign.to_digit(10).unwrap();
            parsed_num += 1;
            matrix[position.0 as usize][position.1 as usize] =
                std::char::from_digit(parsed_num as u32, 10).unwrap();
            let (dx, dy) = get_direction(laser_dir);
            position.0 += dy;
            position.1 += dx;
            matrix = change_matrix(matrix, position, laser_dir);
        }
    }
    let matrix = matrix.clone();
    matrix
}

fn get_direction(dir: char) -> (isize, isize) {
    match dir {
        '>' => (1, 0),
        '<' => (-1, 0),
        '^' => (0, -1),
        'v' => (0, 1),
        _ => unreachable!(),
    }
}

fn reflect_direction(dir: char, mirror: char) -> char {
    match (dir, mirror) {
        ('>', '/') => '^',
        ('>', '!') => '^',
        ('>', '\\') => 'v',
        ('>', '$') => 'v',
        ('<', '/') => 'v',
        ('<', '!') => 'v',
        ('<', '\\') => '^',
        ('<', '$') => '^',
        ('^', '/') => '>',
        ('^', '!') => '>',
        ('^', '\\') => '<',
        ('^', '$') => '<',
        ('v', '/') => '<',
        ('v', '!') => '<',
        ('v', '\\') => '>',
        ('v', '$') => '>',
        _ => dir,
    }
}
#[timed::timed]
fn part1(data: &str) -> usize {
    let mut parsed_data = data
        .lines()
        .map(|x| x.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    parsed_data = change_matrix(parsed_data, (0, 0), '>');
    let mut result = 0;
    for i in 0..parsed_data.len() {
        for j in 0..parsed_data[i].len() {
            if parsed_data[i][j] == '>'
                || parsed_data[i][j] == '<'
                || parsed_data[i][j] == 'v'
                || parsed_data[i][j] == '^'
                || parsed_data[i][j] == '!'
                || parsed_data[i][j] == '$'
                || parsed_data[i][j] == '#'
                || parsed_data[i][j].is_numeric()
            {
                result += 1;
            }
        }
    }
    result
}
#[timed::timed]
fn part2(data: &str) -> u32 {
    let mut result = 0;
    let parsed_data = data
        .lines()
        .map(|x| x.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    for i in 0..parsed_data[0].len() {
        let mut tmp_matrix = parsed_data.clone();
        tmp_matrix = change_matrix(tmp_matrix, (0, i as isize), 'v');
        let mut tmp = 0;
        for k in 0..tmp_matrix.len() {
            for j in 0..tmp_matrix[i].len() {
                if tmp_matrix[k][j] == '>'
                    || tmp_matrix[k][j] == '<'
                    || tmp_matrix[k][j] == 'v'
                    || tmp_matrix[k][j] == '^'
                    || tmp_matrix[k][j] == '!'
                    || tmp_matrix[k][j] == '$'
                    || tmp_matrix[k][j] == '#'
                    || tmp_matrix[k][j].is_numeric()
                {
                    tmp += 1;
                }
            }
        }
        if tmp > result {
            result = tmp
        }
    }
    for i in 0..parsed_data[0].len() {
        let mut tmp_matrix = parsed_data.clone();
        tmp_matrix = change_matrix(
            tmp_matrix,
            ((parsed_data.len() - 1) as isize, i as isize),
            '^',
        );
        let mut tmp = 0;
        for k in 0..tmp_matrix.len() {
            for j in 0..tmp_matrix[i].len() {
                if tmp_matrix[k][j] == '>'
                    || tmp_matrix[k][j] == '<'
                    || tmp_matrix[k][j] == 'v'
                    || tmp_matrix[k][j] == '^'
                    || tmp_matrix[k][j] == '!'
                    || tmp_matrix[k][j] == '$'
                    || tmp_matrix[k][j] == '#'
                    || tmp_matrix[k][j].is_numeric()
                {
                    tmp += 1;
                }
            }
        }
        if tmp > result {
            result = tmp
        }
    }
    for i in 0..parsed_data.len() {
        let mut tmp_matrix = parsed_data.clone();
        tmp_matrix = change_matrix(tmp_matrix, (i as isize, 0), '>');
        let mut tmp = 0;
        for k in 0..tmp_matrix.len() {
            for j in 0..tmp_matrix[i].len() {
                if tmp_matrix[k][j] == '>'
                    || tmp_matrix[k][j] == '<'
                    || tmp_matrix[k][j] == 'v'
                    || tmp_matrix[k][j] == '^'
                    || tmp_matrix[k][j] == '!'
                    || tmp_matrix[k][j] == '$'
                    || tmp_matrix[k][j] == '#'
                    || tmp_matrix[k][j].is_numeric()
                {
                    tmp += 1;
                }
            }
        }
        if tmp > result {
            result = tmp
        }
    }
    for i in 0..parsed_data.len() {
        let mut tmp_matrix = parsed_data.clone();
        tmp_matrix = change_matrix(
            tmp_matrix,
            (i as isize, (parsed_data[0].len() - 1) as isize),
            'v',
        );
        let mut tmp = 0;
        for k in 0..tmp_matrix.len() {
            for j in 0..tmp_matrix[i].len() {
                if tmp_matrix[k][j] == '>'
                    || tmp_matrix[k][j] == '<'
                    || tmp_matrix[k][j] == 'v'
                    || tmp_matrix[k][j] == '^'
                    || tmp_matrix[k][j] == '!'
                    || tmp_matrix[k][j] == '$'
                    || tmp_matrix[k][j] == '#'
                    || tmp_matrix[k][j].is_numeric()
                {
                    tmp += 1;
                }
            }
        }
        if tmp > result {
            result = tmp
        }
    }
    result
}
fn main() {
    let data = read_data(16, InputType::NotTest);
    println!("Part 1: {}", part1(&data));
    println!("Part 2: {}", part2(&data));
}
