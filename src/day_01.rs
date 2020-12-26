use std::fs::File;
use std::io::Read;

fn main() {
    let (final_floor, basement_step) = File::open("day_01_input.txt").unwrap()
        .bytes()
        .filter_map(Result::ok)
        .filter(|&c| c != b' ' && c != b'\n')
        .enumerate()
        .fold((0isize, 0usize), |(floor, step), (i, c)| {
            let floor = match c {
                b'(' => floor + 1,
                b')' => floor - 1,
                _ => panic!("invalid input")
            };
            (floor, if floor == -1 && step == 0 {
                i + 1
            } else {
                step
            })
        });
    println!("part1: {}", final_floor);
    println!("part2: {}", basement_step);
}
