use std::cmp::min;
use std::{fs, i64};

pub fn part_one() -> i64 {
    let grid = fs::read_to_string("src/_2023/13.txt").unwrap();
    let grid = grid
        .split("\n\n")
        .filter(|l| !l.is_empty())
        .map(|b| {
            b.lines()
                .collect::<Vec<&str>>()
                .iter()
                .map(|l| l.chars().collect::<Vec<char>>())
                .collect::<Vec<Vec<char>>>()
        })
        .collect::<Vec<Vec<Vec<char>>>>();

    let res = find_symmetry(&grid);

    res
}

fn find_symmetry(grid: &Vec<Vec<Vec<char>>>) -> i64 {
    let mut horizontal_symmetry = 0;
    let mut vertical_symmetry = 0;

    for block in grid.iter() {
        let mut found_symmetry = false;
        for (i, line) in block.iter().enumerate() {
            if i < block.len() - 1 && line.clone() == block[i + 1] {
                let min_distance_to_edge = min(i, block.len() - 1 - i);
                let left = i;
                let right = i + 1;
                let mut symmetrical = true;

                for j in 0..=min_distance_to_edge {
                    if left < j || right + j >= block.len() {
                        break;
                    }

                    if block[left - j] != block[right + j] {
                        symmetrical = false;
                        break;
                    }
                }

                if symmetrical {
                    horizontal_symmetry += 100 * (i + 1);
                    found_symmetry = true;
                    break;
                }
            }
        }

        if found_symmetry {
            continue;
        }

        let mut columns_as_rows = Vec::new();

        for col in 0..block[0].len() {
            let mut column_values = Vec::new();
            for row in 0..block.len() {
                column_values.push(block[row][col]);
            }
            columns_as_rows.push(column_values);
        }

        for (i, line) in columns_as_rows.iter().enumerate() {
            if i < columns_as_rows.len() - 1 && line.clone() == columns_as_rows[i + 1] {
                let min_distance_to_edge = min(i, columns_as_rows.len() - 1 - i);
                let left = i;
                let right = i + 1;
                let mut symmetrical = true;

                for j in 0..=min_distance_to_edge {
                    if left < j || right + j >= columns_as_rows.len() {
                        break;
                    }

                    if columns_as_rows[left - j] != columns_as_rows[right + j] {
                        symmetrical = false;
                        break;
                    }
                }

                if symmetrical {
                    vertical_symmetry += i + 1;
                }
            }
        }
    }

    (horizontal_symmetry + vertical_symmetry) as i64
}

fn find_mirror(grid: &[Vec<char>]) -> usize {
    for r in 1..grid.len() {
        let above = &grid[..r];
        let below = &grid[r..];

        let diff = above.iter().rev().zip(below.iter()).fold(0, |acc, (a, b)| {
            acc + a.iter().zip(b.iter()).filter(|&(x, y)| x != y).count()
        });

        if diff == 1 {
            return r;
        }
    }
    0
}

pub fn part_two() -> i64 {
    let content = fs::read_to_string("./src/_2023/13.txt").expect("Failed to read file");

    let mut total = 0;

    for block in content.split("\n\n") {
        let grid: Vec<Vec<char>> = block.lines().map(|line| line.chars().collect()).collect();

        let row = find_mirror(&grid);
        total += row * 100;

        let transposed_grid: Vec<Vec<char>> = (0..grid[0].len())
            .map(|i| grid.iter().map(|row| row[i]).collect())
            .collect();

        let col = find_mirror(&transposed_grid);
        total += col;
    }

    total as i64
}
