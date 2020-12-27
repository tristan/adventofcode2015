use std::fs::File;
use std::io::{BufRead, BufReader};

#[inline]
fn split_args(s: &str) -> [usize; 4] {
    let args = s.split(" through ")
        .map(|part| {
            part.split(",").map(|pos| {
                pos.parse::<usize>().unwrap()
            })
        })
        .flatten()
        .collect::<Vec<usize>>();
    [args[0], args[1], args[2], args[3]]
}

fn main() {
    let s = std::time::Instant::now();
    let f = File::open("day_06_input.txt").unwrap();
    let mut lights_p1 = vec![vec![false; 1000]; 1000];
    let mut lights_p2 = vec![vec![0usize; 1000]; 1000];
    BufReader::new(f)
        .lines()
        .filter_map(Result::ok)
        .for_each(|line| {
            match &line[..7] {
                "turn on" => {
                    let args = split_args(&line[8..]);
                    (args[0]..=args[2]).for_each(|x| {
                        (args[1]..=args[3]).for_each(|y| {
                            lights_p1[y][x] = true;
                            lights_p2[y][x] += 1;
                        })
                    })
                },
                "turn of" => {
                    let args = split_args(&line[9..]);
                    (args[0]..=args[2]).for_each(|x| {
                        (args[1]..=args[3]).for_each(|y| {
                            lights_p1[y][x] = false;
                            lights_p2[y][x] = lights_p2[y][x].saturating_sub(1);
                        })
                    })
                },
                "toggle " => {
                    let args = split_args(&line[7..]);
                    (args[0]..=args[2]).for_each(|x| {
                        (args[1]..=args[3]).for_each(|y| {
                            lights_p1[y][x] = !lights_p1[y][x];
                            lights_p2[y][x] += 2;
                        })
                    })

                },
                _ => panic!("invalid input: {}", line)
            }
        });

    let part1: usize = lights_p1.iter().map(|row| row.iter().filter(|&&light| light).count()).sum();
    println!("part1: {}", part1);
    let part2: usize = lights_p2.iter().map(|row| row.iter().sum::<usize>()).sum();
    println!("part2: {}", part2);
    println!("time: {:?}", s.elapsed());
}
