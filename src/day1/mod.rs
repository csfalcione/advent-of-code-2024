use std::path::Path;

use regex::Regex;

pub fn solve_part_1(mut left_list: Vec<i32>, mut right_list: Vec<i32>) -> i32 {
    left_list.sort();
    right_list.sort();
    left_list
        .iter()
        .zip(right_list.iter())
        .map(|(left, right)| (right - left).abs())
        .sum()
}

pub fn read_input(filename: impl AsRef<Path>) -> Result<(Vec<i32>, Vec<i32>), String> {
    let input = std::fs::read_to_string(filename).map_err(|e| e.to_string())?;

    let mut left_list = Vec::new();
    let mut right_list = Vec::new();

    let line_parse_regex = Regex::new(r"(\d+)\s+(\d+)").map_err(|e| e.to_string())?;

    input
        .lines()
        .filter_map(|line| line_parse_regex.captures(line))
        .map(|captures| {
            (
                captures.get(1).unwrap().as_str(),
                captures.get(2).unwrap().as_str(),
            )
        })
        .filter_map(|(left, right)| left.parse::<i32>().ok().zip(right.parse::<i32>().ok()))
        .for_each(|(left, right)| {
            left_list.push(left);
            right_list.push(right);
        });

    Ok((left_list, right_list))
}
