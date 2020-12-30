use std::fs::File;
use std::io::{BufRead, BufReader};

fn check(
    current: u8,
    count: u8,
    rest: &[u8],
    target: u8,
    matches: &mut usize,
    min: &mut u8,
    min_count: &mut usize
) {
    let current = current + rest[0];
    let count = count + 1;
    if current == target {
        if count < *min {
            *min = count;
            *min_count = 1;
        } else if count == *min {
            *min_count += 1;
        }
        *matches += 1;
    } else if current > target {
        return;
    } else if rest.len() == 1 {
        return;
    } else {
        for i in 1..rest.len() {
            check(current, count, &rest[i..], target, matches, min, min_count);
        }
    }
}

fn main() {
    let containers = BufReader::new(File::open("day_17_input.txt").unwrap())
        .lines().filter_map(Result::ok)
        .map(|line| line.parse::<u8>().unwrap())
        .collect::<Vec<_>>();

    let mut matches = 0;
    let mut min = u8::MAX;
    let mut min_count = 0;
    for i in 0..containers.len() {
        check(0, 0, &containers[i..], 150, &mut matches, &mut min, &mut min_count);
    }
    println!("part1: {}", matches);
    println!("part2: {}", min_count);
}
