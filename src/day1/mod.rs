use std::{collections::BTreeMap, path::Path};

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

pub fn solve_part_2(left_list: Vec<i32>, right_list: Vec<i32>) -> i32 {
    let mut right_freq_map: BTreeMap<i32, usize> = BTreeMap::new();
    right_list.iter().for_each(|&location_id| {
        right_freq_map
            .entry(location_id)
            .and_modify(|count| *count += 1)
            .or_insert(1);
    });

    left_list
        .iter()
        .map(|&location_id| location_id * *(right_freq_map.get(&location_id).unwrap_or(&0)) as i32)
        .sum()
}

pub fn read_input(filename: impl AsRef<Path>) -> Result<(Vec<i32>, Vec<i32>), String> {
    let input = std::fs::read_to_string(filename).map_err(|e| e.to_string())?;

    let line_parse_regex = Regex::new(r"(\d+)\s+(\d+)").map_err(|e| e.to_string())?;

    let lists = input
        .lines()
        .filter_map(|line| line_parse_regex.captures(line))
        .map(|captures| {
            (
                captures.get(1).unwrap().as_str(),
                captures.get(2).unwrap().as_str(),
            )
        })
        .filter_map(|(left, right)| left.parse::<i32>().ok().zip(right.parse::<i32>().ok()))
        .unzip();
    Ok(lists)
}
