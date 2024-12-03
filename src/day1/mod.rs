use std::collections::HashMap;
use std::fs;

pub fn run() {
    let input = fs::read_to_string("day1/input.txt").expect("Failed to read input file");


    let mut left_list: Vec<i32> = Vec::new();
    let mut right_list: Vec<i32> = Vec::new();

    for line in input.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if let (Some(left), Some(right)) = (parts.get(0), parts.get(1)) {
            left_list.push(left.parse().expect("Invalid number in left list"));
            right_list.push(right.parse().expect("Invalid number in right list"));
        }
    }

    left_list.sort();
    right_list.sort();

    let total_distance: i32 = left_list
        .iter()
        .zip(right_list.iter())
        .map(|(l, r)| (l - r).abs())
        .sum();

    let mut frequency_table: HashMap<i32, usize> = HashMap::new();
    for &num in &right_list {
        *frequency_table.entry(num).or_insert(0) += 1;
    }

    let similarity: i32 = left_list
        .iter()
        .map(|&num| num * (*frequency_table.get(&num).unwrap_or(&0) as i32))
        .sum();

    println!("Historian Hysteria\nPart 1: {total_distance}\n Part 2: {similarity}");
}
