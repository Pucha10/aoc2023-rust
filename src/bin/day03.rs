use aoc2023::utils::*;
fn is_check(
    selected_lines: Vec<&str>,
    number_start: usize,
    number_len: usize,
    number_in_line: usize,
) -> bool {
    let number_end = number_len + number_start;
    if number_in_line == 0 {
        let line_with_number = selected_lines[0].chars().collect::<Vec<char>>();
        let line_without_number = selected_lines[1].chars().collect::<Vec<char>>();
        if number_start == 0 {
            for i in 0..=number_end {
                if (line_with_number[i] != '.'
                    && (line_with_number[i] < '0' || line_with_number[i] > '9'))
                    || (line_without_number[i] != '.'
                        && (line_without_number[i] < '0' || line_without_number[i] > '9'))
                {
                    return true;
                }
            }
        } else if number_end == line_with_number.len() {
            for i in number_start - 1..number_end {
                if (line_with_number[i] != '.'
                    && (line_with_number[i] < '0' || line_with_number[i] > '9'))
                    || (line_without_number[i] != '.'
                        && (line_without_number[i] < '0' || line_without_number[i] > '9'))
                {
                    return true;
                }
            }
        } else {
            for i in number_start - 1..=number_end {
                if (line_with_number[i] != '.'
                    && (line_with_number[i] < '0' || line_with_number[i] > '9'))
                    || (line_without_number[i] != '.'
                        && (line_without_number[i] < '0' || line_without_number[i] > '9'))
                {
                    return true;
                }
            }
        }
    } else if number_in_line == 2 {
        let line_with_number = selected_lines[1].chars().collect::<Vec<char>>();
        let line_without_number = selected_lines[0].chars().collect::<Vec<char>>();
        if number_start == 0 {
            for i in 0..=number_end {
                if (line_with_number[i] != '.'
                    && (line_with_number[i] < '0' || line_with_number[i] > '9'))
                    || (line_without_number[i] != '.'
                        && (line_without_number[i] < '0' || line_without_number[i] > '9'))
                {
                    return true;
                }
            }
        } else if number_end == line_with_number.len() {
            for i in number_start - 1..number_end {
                if (line_with_number[i] != '.'
                    && (line_with_number[i] < '0' || line_with_number[i] > '9'))
                    || (line_without_number[i] != '.'
                        && (line_without_number[i] < '0' || line_without_number[i] > '9'))
                {
                    return true;
                }
            }
        } else {
            for i in number_start - 1..=number_end {
                if (line_with_number[i] != '.'
                    && (line_with_number[i] < '0' || line_with_number[i] > '9'))
                    || (line_without_number[i] != '.'
                        && (line_without_number[i] < '0' || line_without_number[i] > '9'))
                {
                    return true;
                }
            }
        }
    } else {
        let line_with_number = selected_lines[1].chars().collect::<Vec<char>>();
        let first_line_without_number = selected_lines[0].chars().collect::<Vec<char>>();
        let second_line_without_number = selected_lines[2].chars().collect::<Vec<char>>();
        if number_start == 0 {
            for i in 0..=number_end {
                if (line_with_number[i] != '.'
                    && (line_with_number[i] < '0' || line_with_number[i] > '9'))
                    || (first_line_without_number[i] != '.'
                        && (first_line_without_number[i] < '0'
                            || first_line_without_number[i] > '9'))
                    || (second_line_without_number[i] != '.'
                        && (second_line_without_number[i] < '0'
                            || second_line_without_number[i] > '9'))
                {
                    return true;
                }
            }
        } else if number_end == line_with_number.len() {
            for i in number_start - 1..number_end {
                if (line_with_number[i] != '.'
                    && (line_with_number[i] < '0' || line_with_number[i] > '9'))
                    || (first_line_without_number[i] != '.'
                        && (first_line_without_number[i] < '0'
                            || first_line_without_number[i] > '9'))
                    || (second_line_without_number[i] != '.'
                        && (second_line_without_number[i] < '0'
                            || second_line_without_number[i] > '9'))
                {
                    return true;
                }
            }
        } else {
            for i in number_start - 1..=number_end {
                if (line_with_number[i] != '.'
                    && (line_with_number[i] < '0' || line_with_number[i] > '9'))
                    || (first_line_without_number[i] != '.'
                        && (first_line_without_number[i] < '0'
                            || first_line_without_number[i] > '9'))
                    || (second_line_without_number[i] != '.'
                        && (second_line_without_number[i] < '0'
                            || second_line_without_number[i] > '9'))
                {
                    return true;
                }
            }
        }
    }
    false
}
#[timed::timed]
fn part1(data: &str) -> i32 {
    let lines = data.lines().collect::<Vec<&str>>();
    let mut result = 0;
    let mut start_number = 0;
    let mut number = String::new();
    for (line_index, line) in lines.clone().into_iter().enumerate() {
        let element = line.chars().collect::<Vec<char>>();
        for i in 0..line.len() {
            if element[i] >= '0' && element[i] <= '9' {
                if number == "" {
                    start_number = i;
                }
                number.push(element[i]);
            } else if line_index == 0 && number != "" {
                if is_check(lines[0..=1].to_vec(), start_number, number.len(), 0) {
                    result += number.parse::<i32>().unwrap();
                    number = "".to_string();
                } else {
                    number = "".to_string();
                }
            } else if line_index == lines.len() - 1 && number != "" {
                if is_check(
                    lines[line_index - 1..=line_index].to_vec(),
                    start_number,
                    number.len(),
                    2,
                ) {
                    result += number.parse::<i32>().unwrap();
                    number = "".to_string();
                } else {
                    number = "".to_string();
                }
            } else if number != "" {
                if is_check(
                    lines[line_index - 1..=line_index + 1].to_vec(),
                    start_number,
                    number.len(),
                    1,
                ) {
                    result += number.parse::<i32>().unwrap();
                    number = "".to_string();
                } else {
                    number = "".to_string();
                }
            }
        }
        if line_index == 0 && number != "" {
            if is_check(lines[0..=1].to_vec(), start_number, number.len(), 0) {
                result += number.parse::<i32>().unwrap();
                number = "".to_string();
            } else {
                number = "".to_string();
            }
        } else if line_index == lines.len() - 1 && number != "" {
            if is_check(
                lines[line_index - 1..=line_index].to_vec(),
                start_number,
                number.len(),
                2,
            ) {
                result += number.parse::<i32>().unwrap();
                number = "".to_string();
            } else {
                number = "".to_string();
            }
        } else if number != "" {
            if is_check(
                lines[line_index - 1..=line_index + 1].to_vec(),
                start_number,
                number.len(),
                1,
            ) {
                result += number.parse::<i32>().unwrap();
                number = "".to_string();
            } else {
                number = "".to_string();
            }
        }
    }
    result
}
// fn multiply_2_numbers(selected_lines: Vec<&str>, position: usize, symbol_in_line: usize) -> i32 {
//     let mut how_many_numbers = 0;
//     let mut first_number = 0;
//     let mut second_number = 0;
//     let mut number = String::new();
//     if (symbol_in_line == 0) {
//         let line_with_symbol = selected_lines[0].chars().collect::<Vec<char>>();
//         let line_without_symbol = selected_lines[1].chars().collect::<Vec<char>>();
//         for i in (position - 1..=0).rev() {
//             if line_with_symbol[i]=>'0' && line_with_symbol[i]<='9'{

//             }
//         }
//     }
//     first_number * second_number
// }
#[timed::timed]
fn part2(data: &String) -> i32 {
    let lines = data.lines().collect::<Vec<&str>>();
    // let mut result = 0;
    // for (line_index, line) in lines.clone().into_iter().enumerate() {
    //     let element = line.chars().collect::<Vec<char>>();
    //     for i in 0..line.len() {
    //         if element[i] == '*' {
    //             if line_index == 0 {
    //                 result += multiply_2_numbers(lines[0..=1].to_vec(), i, 0)
    //             } else if line_index == lines.len() - 1 {
    //                 result += multiply_2_numbers(lines[line_index - 1..=line_index].to_vec(), i, 2)
    //             } else {
    //                 result +=
    //                     multiply_2_numbers(lines[line_index - 1..=line_index + 1].to_vec(), i, 1)
    //             }
    //         }
    //     }
    // }
    // result
    let mut result = 0;
    for (line_index, line) in lines.clone().into_iter().enumerate() {
        for (char_index, character) in line.chars().enumerate() {
            if character == '*' {
                let mut numbers = vec![];
                for line_offset in -1..=1 {
                    let line_index_affter_offset = line_index as i32 + line_offset;
                    if line_index_affter_offset < 0
                        || lines.len() as i32 - 1 < line_index_affter_offset
                    {
                        continue;
                    }
                    let mut test_line = lines[line_index_affter_offset as usize]
                        .chars()
                        .collect::<Vec<char>>();
                    for char_offset in -1..=1 {
                        let char_index_after_offset = char_index as i32 + char_offset;
                        if char_index_after_offset < 0
                            || line.len() as i32 - 1 < char_index_after_offset
                        {
                            continue;
                        }
                        let possible_number = test_line[char_index_after_offset as usize];
                        if possible_number >= '0' && possible_number <= '9' {
                            let mut start_index = char_index_after_offset;
                            let mut end_index = char_index_after_offset;
                            while start_index > 0
                                && test_line[start_index as usize - 1] >= '0'
                                && test_line[start_index as usize - 1] <= '9'
                            {
                                start_index -= 1;
                            }
                            while end_index < test_line.len() as i32
                                && test_line[end_index as usize] >= '0'
                                && test_line[end_index as usize] <= '9'
                            {
                                end_index += 1;
                            }
                            let number = test_line[start_index as usize..end_index as usize]
                                .into_iter()
                                .collect::<String>();
                            numbers.push(number.parse::<i32>().unwrap());
                            test_line = test_line
                                .iter()
                                .collect::<String>()
                                .replace(&number, &".".repeat(number.len()))
                                .chars()
                                .collect::<Vec<char>>();
                        }
                    }
                }
                if numbers.len() == 2 {
                    result += numbers[0] * numbers[1];
                }
            }
        }
    }
    result
}

fn main() {
    let data = read_data(3, InputType::NotTest);
    println!("Part 1: {}", part1(&data));
    println!("Part 2: {}", part2(&data));
}
