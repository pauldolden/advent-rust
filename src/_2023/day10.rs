use std::{collections::HashMap, collections::HashSet, collections::VecDeque, fs};

pub fn part_one() -> i32 {
    let mut grid: HashMap<i32, Vec<&str>> = HashMap::new();
    let mut start = (0 as usize, 0 as usize); // Y, X

    let lines = fs::read_to_string("src/_2023/10.txt")
        .unwrap()
        .lines()
        .filter(|x| !x.is_empty())
        .map(|l| l.to_string())
        .collect::<Vec<String>>();

    lines.iter().enumerate().for_each(|(i, l)| {
        grid.insert(
            i as i32,
            l.split("")
                .filter(|x| {
                    if *x == "S" {
                        start = (i, l.find("S").unwrap());
                    }
                    !x.is_empty()
                })
                .collect::<Vec<&str>>(),
        );
    });

    walk(&grid, start)
}

pub fn walk(grid: &HashMap<i32, Vec<&str>>, start: (usize, usize)) -> i32 {
    let mut max_steps = 0;
    let dirs = vec![
        ((0, 1), vec!["-", "J", "7"]),  // Right
        ((0, -1), vec!["-", "F", "L"]), // Left
        ((1, 0), vec!["|", "L", "J"]),  // Down
        ((-1, 0), vec!["|", "7", "F"]), // Up
    ];

    for (dir, valid) in dirs.iter() {
        let mut visited: HashSet<(usize, usize)> = HashSet::new();
        visited.insert(start);
        let steps = explore(start, dir, valid, grid, &dirs, &mut visited);
        if steps > max_steps {
            max_steps = steps;
        }
    }
    max_steps / 2 + 1
}

fn explore(
    current: (usize, usize),
    dir: &(i32, i32),
    valid: &Vec<&str>,
    grid: &HashMap<i32, Vec<&str>>,
    dirs: &Vec<((i32, i32), Vec<&str>)>,
    visited: &mut HashSet<(usize, usize)>,
) -> i32 {
    let next = (
        (current.0 as i32 + dir.0) as usize,
        (current.1 as i32 + dir.1) as usize,
    );

    if next.0 >= grid.len()
        || next.1 >= grid.get(&(next.0 as i32)).unwrap().len()
        || visited.contains(&next)
    {
        return 0;
    }

    let next_val = grid[&(next.0 as i32)][next.1];

    if valid.contains(&next_val) {
        visited.insert(next);
        let mut max_steps = 1; // One step for moving to 'next'

        let mut max_steps_in_dir = 0;
        // Explore further in the same direction
        max_steps_in_dir = max_steps_in_dir.max(explore(next, dir, valid, grid, dirs, visited));

        for (d, v) in dirs.iter() {
            // Explore other directions from the new position
            max_steps_in_dir = max_steps_in_dir.max(explore(next, d, v, grid, dirs, visited));
        }

        max_steps += max_steps_in_dir;

        max_steps
    } else {
        0 // No valid move, so no steps
    }
}

pub fn part_two() -> i32 {
    let grid = fs::read_to_string("src/_2023/10.txt")
        .unwrap()
        .lines()
        .filter(|x| !x.is_empty())
        .map(|l| l.to_string().chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    walk_two(&mut grid.clone());

    0
}

fn walk_two(grid: &mut Vec<Vec<char>>) {
    let mut start = (0, 0);
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    let mut queue: VecDeque<(usize, usize)> = VecDeque::new();

    for (r, row) in grid.iter().enumerate() {
        for (c, ch) in row.iter().enumerate() {
            if ch == &'S' {
                start = (r, c);
            }
        }
    }

    visited.insert(start);
    queue.push_back(start);

    let dirs = vec![
        ((0, 1), vec!['-', 'J', '7']),  // Right
        ((0, -1), vec!['-', 'F', 'L']), // Left
        ((1, 0), vec!['|', 'L', 'J']),  // Down
        ((-1, 0), vec!['|', '7', 'F']), // Up
    ];

    while !queue.is_empty() {
        let current = queue.pop_front().unwrap();

        for (dir, valid) in dirs.iter() {
            let next = (
                (current.0 as i32 + dir.0) as usize,
                (current.1 as i32 + dir.1) as usize,
            );

            if next.0 >= grid.len()
                || next.1 >= grid[0].len()
                || visited.contains(&next)
                || !valid.contains(&grid[next.0][next.1])
            {
                continue;
            }

            visited.insert(next);
            queue.push_back(next);
        }
    }

    println!("Visited: {:?}", visited.len() / 2);
}

fn print_grid(grid: &Vec<Vec<char>>) {
    for row in grid {
        for c in row {
            print!("{}", c);
        }
        println!();
    }
}

fn flood_fill(grid: &mut Vec<Vec<char>>, x: usize, y: usize, target: char, replacement: char) {
    if x >= grid.len() || y >= grid[0].len() || grid[x][y] != target {
        return;
    }

    grid[x][y] = replacement;

    if x > 0 {
        flood_fill(grid, x - 1, y, target, replacement);
    }
    if x < grid.len() - 1 {
        flood_fill(grid, x + 1, y, target, replacement);
    }
    if y > 0 {
        flood_fill(grid, x, y - 1, target, replacement);
    }
    if y < grid[0].len() - 1 {
        flood_fill(grid, x, y + 1, target, replacement);
    }
}
