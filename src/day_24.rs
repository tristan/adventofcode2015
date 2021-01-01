use std::fs::File;
use std::io::{BufRead, BufReader};

fn build_groups(
    values: &[usize],
    target: usize,
    sum: usize,
    qe: usize,
    min_qe: &mut usize
) {
    (0..values.len()).for_each(|i| {
        let sum = sum + values[i];
        let qe = qe * values[i];
        if sum == target {
            if qe < *min_qe {
                *min_qe = qe;
            }
        } else if qe >= *min_qe {
            return;
        } else if sum < target {
            build_groups(&values[i+1..], target, sum, qe, min_qe);
        }
    })
}

fn main() {
    let weights = BufReader::new(File::open("day_24_input.txt").unwrap())
        .lines().filter_map(Result::ok)
        .map(|line| line.parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    let total_sum = weights.iter().sum::<usize>();
    let target = total_sum / 3;
    let mut min_qe = usize::MAX;
    build_groups(&weights, target, 0, 1, &mut min_qe);
    println!("part1: {}", min_qe);
    let target = total_sum / 4;
    let mut min_qe = usize::MAX;
    build_groups(&weights, target, 0, 1, &mut min_qe);
    println!("part2: {}", min_qe);
}
