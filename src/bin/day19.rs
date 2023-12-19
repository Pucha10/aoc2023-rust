use aoc2023::utils::*;
use std::collections::HashMap;
#[derive(Debug)]
struct Part {
    x: i32,
    m: i32,
    a: i32,
    s: i32,
}
#[derive(Debug)]
struct WorkflowRule {
    variable: String,
    operator: char,
    value: i32,
    action: String,
}

#[derive(Debug)]
struct Workflow {
    name: String,
    rules: Vec<WorkflowRule>,
}
fn parse_part(line: &str) -> Part {
    let mut part = Part {
        x: 0,
        m: 0,
        a: 0,
        s: 0,
    };

    let values: Vec<&str> = line
        .trim_start_matches('{')
        .trim_end_matches('}')
        .split(',')
        .map(|s| s.trim())
        .collect();

    for value in values {
        let components: Vec<&str> = value.split('=').collect();
        let variable = components[0].trim();
        let value = components[1].trim().parse::<i32>().unwrap();

        match variable {
            "x" => part.x = value,
            "m" => part.m = value,
            "a" => part.a = value,
            "s" => part.s = value,
            _ => (),
        }
    }

    part
}
#[timed::timed]
fn part1(data: &str) -> i64 {
    let parsed_data = data.split("\r\n\r\n").collect::<Vec<_>>();
    let mut workflows: HashMap<String, Workflow> = HashMap::new();

    for line in parsed_data[0].lines() {
        let parts: Vec<&str> = line.splitn(2, '{').collect();
        let workflow_name = parts[0].trim();
        let rules_part = parts[1].trim_end_matches('}');
        let rules: Vec<&str> = rules_part.split(',').map(|rule| rule.trim()).collect();

        let mut workflow_rules = Vec::new();
        for rule in rules {
            let rule_parts: Vec<&str> = rule.split(':').collect();
            let mut condition = "";
            let mut action = "";
            if rule_parts.len() > 1 {
                condition = rule_parts[0].trim();
                action = rule_parts[1].trim();
            } else {
                action = rule_parts[0].trim();
            }
            let mut variable = String::new();
            let mut operator = ' ';
            let mut value = 0;
            for char in condition.chars() {
                if char.is_alphabetic() {
                    variable.push(char);
                } else if char.is_digit(10) {
                    value = value * 10 + char.to_digit(10).unwrap() as i32;
                } else {
                    operator = char;
                }
            }
            let workflow_rule = WorkflowRule {
                variable,
                operator,
                value,
                action: action.to_string(),
            };

            workflow_rules.push(workflow_rule);
        }

        let workflow = Workflow {
            name: workflow_name.to_string(),
            rules: workflow_rules,
        };

        workflows.insert(workflow_name.to_string(), workflow);
    }
    let mut parts: HashMap<String, Part> = HashMap::new();
    for (index, line) in parsed_data[1]
        .lines()
        .collect::<Vec<_>>()
        .iter()
        .enumerate()
    {
        let part_name = format!("part{}", index + 1);
        let part = parse_part(line);
        parts.insert(part_name, part);
    }

    1
}

// #[timed::timed]
// fn part2(data: &str) -> i64 {

// }
fn main() {
    let data = read_data(19, InputType::Test);
    println!("Part 1: {}", part1(&data));
    // println!("Part 2: {}", part2(&data));
}
