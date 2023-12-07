pub mod _2023 {
    pub mod day1;
    pub mod day2;
    pub mod day3;
}
fn main() {
    let d11 = crate::_2023::day1::part_one();
    let d12 = crate::_2023::day1::part_two();
    let d21 = crate::_2023::day2::part_one();
    let d22 = crate::_2023::day2::part_two();
    let d31 = crate::_2023::day3::part_one();
    let d32 = crate::_2023::day3::part_two();

    println!("Day 1 Part 1: {}", d11);
    println!("Day 1 Part 2: {}", d12);
    println!("Day 2 Part 1: {}", d21);
    println!("Day 2 Part 2: {}", d22);
    println!("Day 3 Part 1: {}", d31);
    println!("Day 3 Part 2: {}", d32);
}
