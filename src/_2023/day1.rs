pub fn part_one() -> i32 {
    let input = std::fs::read_to_string("src/_2023/1.txt").unwrap();

    let lines = input.split("\n");
    let re = regex::Regex::new(r"\d").unwrap();

    let matches: Vec<String> = lines
        .map(|line| {
            let matches: Vec<&str> = re
                .captures_iter(line)
                .filter_map(|cap| cap.get(0))
                .map(|m| m.as_str())
                .collect();

            let first = match matches.first() {
                Some(it) => it,
                None => "",
            };

            let last = match matches.last() {
                Some(it) => it,
                None => "",
            };

            format!("{}{}", first, last)
        })
        .collect();

    let mut count = 0;
    for line_match in matches {
        let num = match line_match.parse::<i32>() {
            Ok(it) => it,
            _ => 0,
        };

        count += num
    }

    return count;
}

pub fn part_two() -> u32 {
    let input = std::fs::read_to_string("src/_2023/1.txt").unwrap();

    let lines = input.split("\n");

    return lines
        .map(|line| {
            line.to_string()
                .replace("one", "one1one")
                .replace("two", "two2two")
                .replace("three", "three3three")
                .replace("four", "four4four")
                .replace("five", "five5five")
                .replace("six", "six6six")
                .replace("seven", "seven7seven")
                .replace("eight", "eight8eight")
                .replace("nine", "nine9nine")
        })
        .map(|line| {
            let re = regex::Regex::new(r"\d").unwrap();

            let matches: Vec<&str> = re
                .captures_iter(&line)
                .filter_map(|cap| cap.get(0))
                .map(|m| m.as_str())
                .collect();

            let first = match matches.first() {
                Some(it) => it,
                None => "",
            };

            let last = match matches.last() {
                Some(it) => it,
                None => "",
            };

            format!("{}{}", first, last)
        })
        .map(|line| {
            let num = match line.parse::<u32>() {
                Ok(it) => it,
                _ => 0,
            };

            num
        })
        .sum();
}
