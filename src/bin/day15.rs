use aoc2023::utils::*;
fn hash_algorithm(text: &str) -> u32 {
    let mut value = 0;
    for c in text.chars() {
        value += c as u32;
        value *= 17;
        value %= 256;
    }
    value
}

#[timed::timed]
fn part1(data: &str) -> u32 {
    let elements = data.split(",").collect::<Vec<_>>();
    let mut result = 0;
    for element in elements {
        result += hash_algorithm(element);
    }
    result
}

#[timed::timed]
fn part2(data: &str) -> u32 {
    let elements = data.split(",").collect::<Vec<_>>();
    let mut boxes: Vec<Vec<String>> = vec![vec![]; 256];
    for element in elements {
        let (lens_label, rest) = element.split_at(
            element
                .find(|c: char| !c.is_alphabetic())
                .unwrap_or(element.len()),
        );
        let operation = rest.chars().next().unwrap();
        let focal_length: u32 = rest
            .get(1..)
            .and_then(|s| s.parse().ok())
            .unwrap_or_default();
        let box_index = hash_algorithm(lens_label) as usize;
        match operation {
            '=' => {
                let lens = format!("{} {}", lens_label, focal_length);
                if let Some(existing_index) = boxes[box_index]
                    .iter()
                    .position(|l| l.starts_with(lens_label))
                {
                    boxes[box_index][existing_index] = lens;
                } else {
                    boxes[box_index].push(lens);
                }
            }
            '-' => {
                let lens_to_remove = format!("{} ", lens_label);
                if let Some(index) = boxes[box_index]
                    .iter()
                    .position(|l| l.starts_with(&lens_to_remove))
                {
                    boxes[box_index].remove(index);
                }
            }
            _ => unreachable!(),
        }
    }
    let mut result = 0;
    for (box_index, box_content) in boxes.iter().enumerate() {
        for (slot_index, lens) in box_content.iter().enumerate() {
            let parts: Vec<&str> = lens.split_whitespace().collect();
            let focal_length: u32 = parts[1].parse().unwrap();
            result += (box_index + 1) as u32 * (slot_index as u32 + 1) * focal_length;
        }
    }
    result
}
fn main() {
    let data = read_data(15, InputType::NotTest);
    println!("Part 1: {}", part1(&data));
    println!("Part 2: {}", part2(&data));
}
