use std::cmp::min;
use std::fs;

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

pub fn part_two() -> i64 {
    0
}
