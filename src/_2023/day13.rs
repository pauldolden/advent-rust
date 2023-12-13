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

    let values = find_symmetry(&grid);

    println!("{:?}", values);

    0
}

fn find_symmetry(grid: &Vec<Vec<Vec<char>>>) -> i64 {
    let mut horiz_values = Vec::new();
    let mut vert_values = Vec::new();
    for block in grid.iter() {
        for (i, line) in block.iter().enumerate() {
            if i < block.len() - 1 && line.clone() == block[i + 1] {
                horiz_values.push((i) as i64);
            }
        }
    }

    for block in grid.iter() {
        for col in 0..block[0].len() {
            let mut column_values = Vec::new();
            for row in 0..block.len() {
                column_values.push(block[row][col]);
            }
            for (i, line) in column_values.iter().enumerate() {
                if i < column_values.len() - 1 && line.clone() == column_values[i + 1] {
                    vert_values.push((i) as i64);
                }
            }
        }
    }

    println!("{:?}", horiz_values);
    println!("{:?}", vert_values);

    0
}

pub fn part_two() -> i64 {
    0
}
