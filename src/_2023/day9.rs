use std::fs;

pub fn part_one() -> i32 {
    fs::read_to_string("src/_2023/9.txt")
        .unwrap()
        .lines()
        .filter(|x| !x.is_empty())
        .map(|x| {
            x.split_whitespace()
                .map(|x| x.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .map(|x| {
            if x.is_empty() {
                return 0;
            }
            if x.len() == 1 {
                return x[0];
            }

            let mut tree: Vec<Vec<i32>> = vec![x.clone()];
            let mut line = x.clone();

            while line.iter().sum::<i32>() != 0 {
                let mut new_line = Vec::new();
                for i in 0..line.len() - 1 {
                    new_line.push(line[i + 1] - line[i]);
                }
                line = new_line.clone();
                tree.push(new_line);
            }

            tree.iter()
                .rev()
                .fold(0, |acc, row| row.last().unwrap_or(&0) + acc)
        })
        .sum::<i32>()
}

pub fn part_two() -> i32 {
    fs::read_to_string("src/_2023/9.txt")
        .unwrap()
        .lines()
        .filter(|x| !x.is_empty())
        .map(|x| {
            x.split_whitespace()
                .map(|x| x.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .map(|x| {
            let mut tree: Vec<Vec<i32>> = vec![];

            let mut line = x.clone();

            while line.iter().sum::<i32>() != 0 {
                let mut new_line = Vec::new();

                for i in 0..line.len() - 1 {
                    new_line.push(line[i + 1] - line[i]);
                }

                line = new_line.clone();
                tree.push(new_line);
            }

            // Push original input again to complete top row
            tree.push(x.clone());

            // Fold using sum of complete top row
            tree.iter()
                .rev()
                .fold(0, |acc, row| row.first().unwrap_or(&0) - acc)
        })
        .sum::<i32>()
}
