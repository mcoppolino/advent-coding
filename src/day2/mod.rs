use std::fs;
use std::io::{self, BufRead};

pub fn run() {
    let file = fs::File::open("day2/input.txt").expect("Failed to open input file");
    
    // Avoid Vec allocs by reading directly.
    let reader = io::BufReader::new(file);

    let mut safe_reports = 0;
    let mut dampened_safe_reports = 0;

    for line in reader.lines() {
        let line = line.expect("Failed to read a line");

        let level: Vec<i32> = line
            .split_whitespace()
            .map(|num| num.parse::<i32>().expect("Invalid number"))
            .collect();

        if is_safe(&level) {
            safe_reports += 1;
            dampened_safe_reports += 1;
            continue;
        }

        let mut is_dampened_safe = false;
        for i in 0..level.len() {
            let mut modified_level = level.clone();
            modified_level.remove(i);
            if is_safe(&modified_level) {
                is_dampened_safe = true;
                break;
            }
        }

        if is_dampened_safe {
            dampened_safe_reports += 1;
        }
    }

    println!("Red-Nosed Reports\nPart 1: {safe_reports}\nPart 2: {dampened_safe_reports}");
}

fn is_safe(level: &[i32]) -> bool {
    if level.len() < 2 {
        return true;
    }

    let mut increasing = None;

    for pair in level.windows(2) {
        let diff = pair[1] - pair[0];

        if diff.abs() < 1 || diff.abs() > 3 {
            return false; 
        }

        if increasing.is_none() {
            increasing = Some(diff > 0);
        } else if let Some(is_increasing) = increasing {
            if (diff > 0) != is_increasing {
                return false;
            }
        }
    }

    true
}
