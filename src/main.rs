mod day1;

fn main() -> Result<(), String> {
    let filename = "src/day1/input";
    let (left_list, right_list) = day1::read_input(filename)?;
    println!("{}", day1::solve_part_1(left_list, right_list));
    Ok(())
}
