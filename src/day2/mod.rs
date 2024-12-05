use std::{cmp::Ordering, i32};

pub fn solve_part_1(reports: Vec<Vec<i32>>) -> usize {
    reports.iter().filter(|&report| is_safe(&report)).count()
}

fn is_safe(report: &[i32]) -> bool {
    is_monotonic(report) && abs_differences_are_between(report, 1, 3)
}

fn abs_differences_are_between(report: &[i32], lower_inclusive: i32, upper_inclusive: i32) -> bool {
    report
        .windows(2)
        .map(|window| (window[0], window[1]))
        .map(|(left, right)| (right - left).abs())
        .all(|diff| lower_inclusive <= diff && diff <= upper_inclusive)
}

#[derive(PartialEq)]
enum Directionality {
    Indeterminant,
    Increasing,
    Constant,
    Decreasing,
    NotMonotonic,
}
fn is_monotonic(report: &[i32]) -> bool {
    use Directionality::*;

    let directionality = report
        .windows(2)
        .map(|window| (window[0], window[1]))
        .map(|(left, right)| match right.cmp(&left) {
            Ordering::Equal => Constant,
            Ordering::Greater => Increasing,
            Ordering::Less => Decreasing,
        })
        .fold(Indeterminant, |aggregate, cursor| match aggregate {
            Indeterminant => cursor,
            NotMonotonic => NotMonotonic,
            Constant => NotMonotonic,
            other => {
                // other is either Increasing or Decreasing
                if other == cursor {
                    other
                } else {
                    NotMonotonic
                }
            }
        });
    directionality == Increasing || directionality == Decreasing
}

pub fn read_input(filename: impl AsRef<std::path::Path>) -> Result<Vec<Vec<i32>>, String> {
    let input = std::fs::read_to_string(filename).map_err(|e| e.to_string())?;

    let result = input
        .lines()
        .map(|line: &str| -> Vec<i32> {
            line.split(char::is_whitespace)
                .map(|input_number: &str| input_number.parse::<i32>().unwrap())
                .collect()
        })
        .collect();
    Ok(result)
}
