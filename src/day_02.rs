use std::fs::File;
use std::io::{BufReader, BufRead};

fn main() {
    let f = File::open("day_02_input.txt").unwrap();
    let (part1, part2): (usize, usize) = BufReader::new(f)
        .lines()
        .filter_map(Result::ok)
        .map(|line| {
            let dims = line.splitn(3, "x")
                .map(|c| c.parse().unwrap())
                .collect::<Vec<usize>>();

            match dims.as_slice() {
                &[length, width, height] => {
                    let paper = {
                        let lw = length * width;
                        let wh = width * height;
                        let hl = height * length;
                        lw.min(wh).min(hl) +
                            2 * lw +
                            2 * wh +
                            2 * hl
                    };
                    let ribbon = {
                        let a = if width > height {
                            if width > length {
                                (height + length) * 2
                            } else {
                                (width + height) * 2
                            }
                        } else {
                            if height > length {
                                (width + length) * 2
                            } else {
                                (height + width) * 2
                            }
                        };
                        a + (length * width * height)
                    };
                    (paper, ribbon)
                },
                _ => panic!("invalid size")
            }
        })
        .fold((0, 0), |(paper_sum, ribbon_sum), (paper, ribbon)| {
            (paper_sum + paper, ribbon_sum + ribbon)
        });
    println!("part1: {}", part1);
    println!("part1: {}", part2);
}
