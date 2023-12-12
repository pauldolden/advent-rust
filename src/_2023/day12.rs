use regex::Regex;
use std::fs;

#[derive(Debug)]
pub struct Line {
    pub springs: Vec<String>,
    pub numbers: Vec<i32>,
}

pub fn part_one() -> i64 {
    let grid = fs::read_to_string("src/_2023/12.txt").unwrap();
    let grid = grid
        .split("\n")
        .filter(|x| !x.is_empty())
        .map(|l| l.split(" ").collect::<Vec<&str>>())
        .map(|l| {
            let springs = l[0]
                .split("")
                .filter(|c| !c.is_empty())
                .map(|c| c.to_string())
                .collect::<Vec<String>>();

            let numbers = l[1]
                .split(",")
                .map(|n| n.parse::<i32>().unwrap())
                .collect::<Vec<i32>>();

            Line { springs, numbers }
        })
        .collect::<Vec<Line>>();

    let mut count = 0;
    for line in grid {
        count += walk_line(line.springs.clone(), 0, &line.numbers, 0, 0);
    }

    count as i64
}

fn walk_line(
    line: Vec<String>,
    str_idx: usize,
    numbers: &Vec<i32>,
    num_idx: usize,
    group_size: i32,
) -> i32 {
    if str_idx == line.len() {
        return validate_hash_groups(&line.join(""), numbers);
    }

    if line[str_idx] != "?" {
        if line[str_idx] == "#" {
            return walk_line(line, str_idx + 1, numbers, num_idx, group_size + 1);
        }

        if line[str_idx] == "." {
            if group_size > 0 {
                if group_size == numbers[num_idx] {
                    return walk_line(line, str_idx + 1, numbers, num_idx + 1, 0);
                } else {
                    return 0;
                }
            } else {
                return walk_line(line, str_idx + 1, numbers, num_idx, 0);
            }
        }
    } else {
        let mut count = 0;
        // Replace '?' with a '#'
        let mut new_line = line.clone();
        new_line[str_idx] = "#".to_string();
        count += walk_line(new_line, str_idx + 1, numbers, num_idx, group_size + 1);

        // Replace '?' with a '.'
        let mut new_line = line.clone();
        new_line[str_idx] = ".".to_string();
        count += walk_line(new_line, str_idx + 1, numbers, num_idx, 0);

        return count;
    }
    // should never get here
    0
}

fn validate_hash_groups(line: &str, numbers: &[i32]) -> i32 {
    let re = Regex::new(r"#+").unwrap();
    let hash_groups: Vec<&str> = re.find_iter(line).map(|mat| mat.as_str()).collect();

    if hash_groups.len() != numbers.len() {
        return 0;
    }

    for (i, group) in hash_groups.iter().enumerate() {
        if group.len() as i32 != numbers[i] {
            return 0;
        }
    }

    1
}

pub fn part_two() -> i64 {
    let grid = fs::read_to_string("src/_2023/12.txt")
        .unwrap()
        .lines()
        .filter(|x| !x.is_empty())
        .map(|l| l.to_string())
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    0
}
