use std::{collections::HashMap, fs};

struct Symbol {
    symbol: String,
    number: String,
}

pub fn part_one() -> i32 {
    let input = fs::read_to_string("src/_2023/3.txt").unwrap();
    let symbol_re = regex::Regex::new(r"[*\w.]").unwrap();
    let number_re = regex::Regex::new(r"\d+").unwrap();
    let mut number_map: HashMap<String, String> = HashMap::new();
    let mut symbol_map: HashMap<String, String> = HashMap::new();

    for (idx, line) in input.split("\n").enumerate() {
        let symbols = symbol_re
            .captures_iter(line)
            .map(|cap| cap[0].to_string())
            .collect::<Vec<String>>();

        let numbers = number_re
            .captures_iter(line)
            .map(|cap| cap[0].to_string())
            .collect::<Vec<String>>();

        for (i, symbol) in symbols.iter().enumerate() {}
    }

    0
}

pub fn part_two() -> i32 {
    0
}
