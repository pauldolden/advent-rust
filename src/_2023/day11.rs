use std::fs;

pub fn part_one() -> i64 {
    let mut grid = fs::read_to_string("src/_2023/11.txt")
        .unwrap()
        .lines()
        .filter(|x| !x.is_empty())
        .map(|l| l.to_string())
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    // duplicate all rows that are only .
    let mut empty_rows = Vec::new();
    let mut empty_cols = Vec::new();

    for (i, row) in grid.clone().iter().rev().enumerate() {
        if row.iter().all(|c| *c == '.') {
            empty_rows.push(grid.len() - i - 1);
        }
    }

    for col in 0..grid[0].len() {
        let mut column_values = Vec::new();
        for row in 0..grid.len() {
            column_values.push(grid[row][col]);
        }
        if column_values.iter().all(|c| *c == '.') {
            empty_cols.push(col);
        }
    }

    // duplicate all columns that are only .
    for row in grid.iter_mut() {
        for (c, col) in empty_cols.iter().enumerate() {
            row.insert(*col + (c + 1), '.');
        }
    }

    // duplicate all rows that are only .
    for row in empty_rows.iter() {
        grid.insert(*row + 1, vec!['.'; grid[0].len()]);
    }

    let nodes = find_nodes(&grid);

    let mut total = 0;

    for i in 0..nodes.len() {
        for j in (i + 1)..nodes.len() {
            total += manhattan_distance(nodes[i], nodes[j]);
        }
    }

    total
}

fn manhattan_distance(p1: (i64, i64), p2: (i64, i64)) -> i64 {
    let (x1, y1) = p1;
    let (x2, y2) = p2;
    (x2 - x1).abs() + (y2 - y1).abs()
}

fn find_nodes(grid: &Vec<Vec<char>>) -> Vec<(i64, i64)> {
    let mut nodes = Vec::new();

    for (r, row) in grid.iter().enumerate() {
        for (c, ch) in row.iter().enumerate() {
            if ch == &'#' {
                nodes.push((r as i64, c as i64));
            }
        }
    }

    nodes
}

pub fn part_two() -> i64 {
    let grid = fs::read_to_string("src/_2023/11.txt")
        .unwrap()
        .lines()
        .filter(|x| !x.is_empty())
        .map(|l| l.to_string())
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    // duplicate all rows that are only .
    let mut empty_rows = Vec::new();
    let mut empty_cols = Vec::new();

    for (i, row) in grid.clone().iter().enumerate() {
        if row.iter().all(|c| *c == '.') {
            empty_rows.push(i);
        }
    }

    for col in 0..grid[0].len() {
        let mut column_values = Vec::new();
        for row in 0..grid.len() {
            column_values.push(grid[row][col]);
        }
        if column_values.iter().all(|c| *c == '.') {
            empty_cols.push(col);
        }
    }

    let mut nodes = find_nodes(&grid);

    let mut total = 0;

    let empty_rows_i64 = empty_rows.iter().map(|&r| r as i64).collect::<Vec<i64>>();
    let empty_cols_i64 = empty_cols.iter().map(|&c| c as i64).collect::<Vec<i64>>();

    // shift nodes by empty rows and cols
    for node in nodes.iter_mut() {
        shift_node(node, empty_rows_i64.clone(), empty_cols_i64.clone(), 1);
    }

    println!("Nodes: {:?}", nodes);
    println!("Shifted Nodes: {:?}", nodes);

    for i in 0..nodes.len() {
        for j in (i + 1)..nodes.len() {
            total += manhattan_distance(nodes[i], nodes[j]);
        }
    }

    total
}

fn shift_node(node: &mut (i64, i64), empty_rows: &[i64], empty_cols: &[i64], scale: i64) {
    let (r, c) = *node;

    let r_shift = empty_rows.iter().filter(|&&row| row < r).count() as i64 * scale;
    let c_shift = empty_cols.iter().filter(|&&col| col < c).count() as i64 * scale;

    *node = (r + r_shift, c + c_shift);
}
