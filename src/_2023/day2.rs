use regex::Regex;

const RED_MAX: i32 = 12;
const GREEN_MAX: i32 = 13;
const BLUE_MAX: i32 = 14;

pub fn part_one() -> i32 {
    let input = std::fs::read_to_string("src/_2023/2.txt").unwrap();

    let lines = input.split("\n");

    let re = Regex::new(r"((?:\d+\s\w+(?:,\s)?)+)").unwrap();
    let id_re = Regex::new(r"\d+").unwrap();

    let matches = lines
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
        .collect::<Vec<(i32, Vec<&str>)>>()
        .iter()
        .map(|(id, m)| {
            let mat = m
                .iter()

            (id, mat)
        });

    0
}
