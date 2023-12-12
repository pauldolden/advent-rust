use rayon::prelude::*;
use regex::Regex;
use std::sync::Mutex;
use std::{collections::HashMap, fs};

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

fn walk_line_two(cfg: String, nums: Vec<i32>, cache: &mut HashMap<(String, Vec<i32>), i64>) -> i64 {
    if cfg.is_empty() {
        if nums.is_empty() {
            return 1;
        } else {
            return 0;
        }
    }

    if nums.is_empty() {
        if cfg.contains("#") {
            return 0;
        } else {
            return 1;
        }
    }

    let key = (cfg.clone(), nums.clone());

    if cache.contains_key(&key) {
        return *cache.get(&key).unwrap();
    }

    let mut result = 0;

    if ".?".contains(cfg.chars().nth(0).unwrap()) {
        let mut new_cfg = cfg.clone();
        new_cfg.remove(0);
        result += walk_line_two(new_cfg, nums.clone(), cache);
    }

    if "#?".contains(cfg.chars().nth(0).unwrap()) {
        let str_cfg = cfg.as_str();
        if nums[0] <= cfg.len() as i32
            && !str_cfg[..(nums[0] as usize)].contains(".")
            && ((nums[0] as usize) == cfg.len()
                || str_cfg.chars().nth(nums[0] as usize).unwrap() != '#')
        {
            let cfg_slice = if (nums[0] as usize) < cfg.len() {
                cfg[(nums[0] as usize + 1)..].to_string()
            } else {
                String::new()
            };

            let nums_slice = if nums.len() > 1 { &nums[1..] } else { &[] };
            result += walk_line_two(cfg_slice, nums_slice.to_vec(), cache);
        }
    }

    cache.insert(key, result);
    result
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
                .collect::<Vec<String>>();

            let numbers = l[1]
                .split(",")
                .map(|n| n.parse::<i32>().unwrap())
                .collect::<Vec<i32>>();

            let numbers = (0..5).flat_map(|_| numbers.clone()).collect::<Vec<i32>>();

            (springs, numbers)
        })
        .collect::<Vec<(Vec<String>, Vec<i32>)>>();

    let count = grid
        .par_iter()
        .map(|line| {
            let cache = Mutex::new(HashMap::new());
            let mut cache = cache.lock().unwrap_or_else(|e| e.into_inner());
            walk_line_two(line.0.join(""), line.1.clone(), &mut cache)
        })
        .sum::<i64>() as i64;

    count as i64
}
