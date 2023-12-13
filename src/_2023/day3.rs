// use regex::Regex;
// use std::fs;
//
// pub fn part_one() -> i32 {
//     let grid = fs::read_to_string("src/_2023/3.txt")
//         .unwrap()
//         .lines()
//         .filter(|x| !x.is_empty())
//         .map(|l| l.to_string())
//         .collect::<Vec<String>>();
//
//     let symbol_re = Regex::new(r"[^\w\s\.]").unwrap();
//     let number_re = Regex::new(r"\d+").unwrap();
//     let mut symbols = Vec::new();
//     let mut numbers = Vec::new();
//
//     for (l, line) in grid.iter().enumerate() {
//         symbols.push(
//             symbol_re
//                 .captures_iter(&line)
//                 .map(|cap| (l, (cap.get(0).unwrap().start(), cap.get(0).unwrap().end())))
//                 .collect::<Vec<(usize, (usize, usize))>>(),
//         );
//
//         numbers.push(
//             number_re
//                 .captures_iter(&line)
//                 .map(|cap| (l, (cap.get(0).unwrap().start(), cap.get(0).unwrap().end())))
//                 .collect::<Vec<(usize, (usize, usize))>>(),
//         )
//     }
//
//     let symbols = symbols
//         .iter()
//         .flatten()
//         .collect::<Vec<&(usize, (usize, usize))>>();
//
//     let numbers = numbers
//         .iter()
//         .flatten()
//         .collect::<Vec<&(usize, (usize, usize))>>();
//
//     // for each number look around its indeces for a symbol
//
//     for number in numbers {
//         let (y, (start_x, end_x)) = number;
//         let mut has_adjacent_symbol = false;
//
//     }
//
//     print_grid(&grid);
//
//     0
// }
//
// fn print_grid(grid: &Vec<String>) {
//     for line in grid {
//         println!("{}", line);
//     }
// }
//
// pub fn part_two() -> i32 {
//     0
// }
