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

    let res = walk_two(&mut grid.clone());

    res as i32
}

fn walk_two(grid: &mut Vec<Vec<char>>) -> usize {
    let mut start = (0, 0);
    let mut loop_set = HashSet::new();
    let mut q = VecDeque::new();

    for (r, row) in grid.iter().enumerate() {
        for (c, ch) in row.iter().enumerate() {
            if ch == &'S' {
                start = (r, c);
            }
        }
    }

    loop_set.insert(start);
    q.push_back(start);

    while let Some((r, c)) = q.pop_front() {
        let ch = grid[r][c];
        let grid_len = grid.len();
        let row_len = grid[0].len();

        // Up
        if r > 0
            && "S|JL".contains(ch)
            && "|7F".contains(grid[r - 1][c])
            && !loop_set.contains(&(r - 1, c))
        {
            loop_set.insert((r - 1, c));
            q.push_back((r - 1, c));
        }

        // Down
        if r < grid_len - 1
            && "S|F7".contains(ch)
            && "|LJ".contains(grid[r + 1][c])
            && !loop_set.contains(&(r + 1, c))
        {
            loop_set.insert((r + 1, c));
            q.push_back((r + 1, c));
        }

        // Left
        if c > 0
            && "S-J7".contains(ch)
            && "-LF".contains(grid[r][c - 1])
            && !loop_set.contains(&(r, c - 1))
        {
            loop_set.insert((r, c - 1));
            q.push_back((r, c - 1));
        }

        // Right
        if c < row_len - 1
            && "S-FL".contains(ch)
            && "-7J".contains(grid[r][c + 1])
            && !loop_set.contains(&(r, c + 1))
        {
            loop_set.insert((r, c + 1));
            q.push_back((r, c + 1));
        }
    }

    // replace non-loop chars with spaces
    for (r, row) in grid.iter_mut().enumerate() {
        for (c, ch) in row.iter_mut().enumerate() {
            if !loop_set.contains(&(r, c)) && *ch != 'S' {
                *ch = '.';
            }
        }
    }

    let outside_loop: HashSet<(usize, usize)> = HashSet::new();

    for (r, row) in grid.iter().enumerate() {
        let mut within = false;
        let mut up = false;
        for (c, ch) in row.iter().enumerate() {}
    }

    let mut outside = HashSet::new();

    // replace S with F (or 7 for main)
    for (r, row) in grid.iter_mut().enumerate() {
        for (c, ch) in row.iter_mut().enumerate() {
            if *ch == 'S' {
                *ch = '7'; // main input
            }
        }
    }

    for (r, row) in grid.iter().enumerate() {
        println!("{}", row.iter().collect::<String>());
        let mut within = false;
        let mut up: Option<bool> = None;
        for (c, &ch) in row.iter().enumerate() {
            match ch {
                '|' => {
                    assert!(up.is_none(), "up should be None");
                    within = !within;
                }
                '-' => {
                    assert!(up.is_some(), "up should be Some");
                }
                'L' | 'F' => {
                    assert!(up.is_none(), "up should be None");
                    up = Some(ch == 'L');
                }
                '7' | 'J' => {
                    assert!(up.is_some(), "up should be Some");
                    if ch != if up.unwrap() { 'J' } else { '7' } {
                        within = !within;
                    }
                    up = None;
                }
                '.' => {}
                _ => panic!("unexpected character (horizontal): {}", ch),
            }
            if !within {
                outside.insert((r, c));
            }
        }
    }

    let tiles = grid.len() * grid[0].len();

    tiles - outside.union(&loop_set).count()
}
