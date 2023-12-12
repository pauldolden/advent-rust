use regex::Regex;
use std::fs;

#[derive(Debug)]
pub struct Line {
    pub springs: Vec<char>,
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
                .map(|s| s.chars().next().unwrap())
                .collect::<Vec<char>>();

            let numbers = l[1]
                .split(",")
                .map(|n| n.parse::<i32>().unwrap())
                .collect::<Vec<i32>>();

            Line { springs, numbers }
        })
        .collect::<Vec<Line>>();

    let mut count = 0;
    for (l, line) in grid.iter().enumerate() {
        count += walk_line(&mut line.springs.clone(), 0, &line.numbers, 0, 0);
        println!("{}: {}", l, count);
    }

    count as i64
}

fn walk_line(
    line: &mut Vec<char>,
    str_idx: usize,
    numbers: &[i32],
    num_idx: usize,
    group_size: i32,
) -> i32 {
    if str_idx == line.len() {
        return validate_hash_groups(&line.iter().collect::<String>(), numbers);
    }

    match line[str_idx] {
        '#' => walk_line(line, str_idx + 1, numbers, num_idx, group_size + 1),
        '.' => {
            let next_num_idx = if group_size > 0 { num_idx + 1 } else { num_idx };
            walk_line(line, str_idx + 1, numbers, next_num_idx, 0)
        }
        '?' => {
            line[str_idx] = '#';
            let count_hash = walk_line(line, str_idx + 1, numbers, num_idx, group_size + 1);

            line[str_idx] = '.';
            let count_dot = walk_line(line, str_idx + 1, numbers, num_idx, 0);

            line[str_idx] = '?'; // undo the change
            count_hash + count_dot
        }
        _ => unreachable!(), // Or handle other cases if needed
    }
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
                .collect::<Vec<String>>()
                .join("");

            let mut new_springs = Vec::new();

            for _ in 0..5 {
                new_springs.push(springs.clone());
            }

            let springs = new_springs
                .join("?")
                .split("")
                .filter(|c| !c.is_empty())
                .map(|c| c.to_string())
                .map(|s| s.chars().next().unwrap())
                .collect::<Vec<char>>();

            let numbers = l[1]
                .split(",")
                .map(|n| n.parse::<i32>().unwrap())
                .collect::<Vec<i32>>();

            let numbers = (0..5).flat_map(|_| numbers.clone()).collect::<Vec<i32>>();

            Line { springs, numbers }
        })
        .collect::<Vec<Line>>();

    let mut count = 0;
    for line in grid.iter() {
        count += walk_line(&mut line.springs.clone(), 0, &line.numbers, 0, 0);
    }

    count as i64
}
