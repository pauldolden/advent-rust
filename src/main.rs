pub mod _2023 {
    pub mod day1;
    pub mod day2;
}
fn main() {
    let d11 = crate::_2023::day1::part_one();
    let d12 = crate::_2023::day1::part_two();
    let d21 = crate::_2023::day2::part_one();

    println!("Day 1 Part 1: {}", d11);
    println!("Day 1 Part 2: {}", d12);
    println!("Day 2 Part 1: {}", d21);
}
