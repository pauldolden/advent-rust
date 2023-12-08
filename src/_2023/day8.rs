use std::collections::HashMap;
use std::fs;

fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}

fn lcm(a: u64, b: u64) -> u64 {
    a * b / gcd(a, b)
}

pub fn part_one() -> i32 {
    let dir_map: HashMap<char, usize> = [('L', 0), ('R', 1)].into_iter().collect();
    let mut map: HashMap<String, Vec<String>> = HashMap::new();
    let input = fs::read_to_string("src/_2023/8.txt").unwrap();

    let parts = input.split("\n\n").collect::<Vec<&str>>();
    let instructions = parts[0]
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect::<Vec<char>>();

    for line in parts[1].lines() {
        let line_parts = line.split(" = ").collect::<Vec<&str>>();
        let source = line_parts[0];
        let dests = line_parts[1]
            .split(", ")
            .map(|d| d.trim_matches(|c: char| c == '(' || c == ')'))
            .map(String::from)
            .collect::<Vec<String>>();
        map.insert(source.to_string(), dests);
    }

    return map
        .keys()
        .filter(|&key| key == "AAA")
        .cloned()
        .collect::<Vec<String>>()
        .into_iter()
        .map(|node| {
            let mut steps = 0;
            let mut current_node = node;
            let mut i = 0;
            while current_node != "ZZZ" {
                steps += 1;
                let instruction = instructions[i];
                let dir = dir_map[&instruction];
                let curr_neighbors = &map[&current_node];
                let next_node = &curr_neighbors[dir];

                current_node = next_node.clone();
                i = (i + 1) % instructions.len();
            }
            steps
        })
        .sum();
}

pub fn part_two() -> u64 {
    let dir_map: HashMap<char, usize> = [('L', 0), ('R', 1)].into_iter().collect();
    let mut map: HashMap<String, Vec<String>> = HashMap::new();
    let input = fs::read_to_string("src/_2023/8.txt").unwrap();

    let parts = input.split("\n\n").collect::<Vec<&str>>();
    let instructions = parts[0]
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect::<Vec<char>>();

    for line in parts[1].lines() {
        let line_parts = line.split(" = ").collect::<Vec<&str>>();
        let source = line_parts[0];
        let dests = line_parts[1]
            .split(", ")
            .map(|d| d.trim_matches(|c: char| c == '(' || c == ')'))
            .map(String::from)
            .collect::<Vec<String>>();
        map.insert(source.to_string(), dests);
    }

    return map
        .keys()
        .filter(|&key| key.ends_with('A'))
        .cloned()
        .collect::<Vec<String>>()
        .into_iter()
        .map(|node| {
            let mut steps = 0;
            let mut current_node = node;
            let mut i = 0;
            while current_node.chars().last().unwrap() != 'Z' {
                steps += 1;
                let instruction = instructions[i];
                let dir = dir_map[&instruction];
                let curr_neighbors = &map[&current_node];
                let next_node = &curr_neighbors[dir];

                current_node = next_node.clone();
                i = (i + 1) % instructions.len();
            }
            steps
        })
        .collect::<Vec<i32>>()
        .into_iter()
        .fold(1, |n, x| lcm(x as u64, n));
}
