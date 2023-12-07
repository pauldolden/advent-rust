use std::collections::HashMap;

use regex::Regex;

const RED_MAX: i32 = 12;
const GREEN_MAX: i32 = 13;
const BLUE_MAX: i32 = 14;

pub fn part_one() -> i32 {
    let input = std::fs::read_to_string("src/_2023/2.txt").unwrap();
    let lines = input.split("\n");
    let re = Regex::new(r"((?:\d+\s\w+(?:,\s)?)+)").unwrap();
    let id_re = Regex::new(r"\d+").unwrap();

    return lines
        .filter(|line| !line.is_empty())
        .map(|line| {
            if line.is_empty() {
                return (0, vec![]);
            }
            let matches: Vec<&str> = re
                .captures_iter(line)
                .filter_map(|cap| cap.get(0))
                .map(|m| m.as_str())
                .collect();

            let id = id_re
                .find(line)
                .expect("Failed to find ID")
                .as_str()
                .trim()
                .parse::<i32>()
                .expect("Failed to parse ID");
            (id, matches)
        })
        .map(|(id, matches)| {
            let matches = matches
                .iter()
                .map(|m| m.split(", ").collect::<Vec<&str>>())
                .flatten()
                .map(|m| {
                    let mut split = m.split(" ");
                    let value = split.next().unwrap().parse::<i32>().unwrap();
                    let color = split.next().unwrap();

                    (color, value)
                })
                .collect::<Vec<(&str, i32)>>();
            (id, matches)
        })
        .map(|(id, matches)| {
            let matches = matches
                .iter()
                .map(|(color, value)| match color {
                    &"red" => *value > RED_MAX,
                    &"green" => *value > GREEN_MAX,
                    &"blue" => *value > BLUE_MAX,
                    _ => true,
                })
                .collect::<Vec<bool>>();
            (id, matches)
        })
        .collect::<Vec<(i32, Vec<bool>)>>()
        .iter()
        .filter(|(_, matches)| matches.iter().all(|m| *m == false))
        .map(|(id, _)| *id)
        .collect::<Vec<i32>>()
        .iter()
        .sum();
}

pub fn part_two() -> i32 {
    let input = std::fs::read_to_string("src/_2023/2.txt").unwrap();
    let lines = input.split("\n");
    let re = Regex::new(r"((?:\d+\s\w+(?:,\s)?)+)").unwrap();

    return lines
        .filter(|line| !line.is_empty())
        .map(|line| {
            let caps = re
                .captures_iter(line)
                .filter_map(|cap| cap.get(0))
                .map(|m| m.as_str())
                .collect::<Vec<&str>>();

            caps
        })
        .map(|matches| {
            matches
                .iter()
                .map(|m| m.split(", ").collect::<Vec<&str>>())
                .flatten()
                .map(|m| {
                    let mut split = m.split(" ");
                    let value = split.next().unwrap().parse::<i32>().unwrap();
                    let color = split.next().unwrap();

                    (color, value)
                })
                .collect::<Vec<(&str, i32)>>()
        })
        .map(|matches| {
            let mut max_values: HashMap<&str, i32> = HashMap::new();

            for (color, value) in matches {
                let max_value = max_values.entry(color).or_insert(value);
                if value > *max_value {
                    *max_value = value;
                }
            }

            max_values.iter().map(|(_, v)| v).product::<i32>()
        })
        .sum::<i32>();
}
