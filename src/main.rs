#![allow(dead_code)]

mod day1;
mod day2;

fn main() -> Result<(), String> {
    let filename = "src/day2/input";
    let reports = day2::read_input(filename)?;
    println!("{}", day2::solve_part_1(reports));
    Ok(())
}
