use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let f = File::open("day_05_input.txt").unwrap();
    let part1 = BufReader::new(f)
        .lines()
        .filter_map(Result::ok)
        .filter(|line| {
            let (vowels, _, has_repeat, has_following) = line.chars().fold(
                (0, '-', false, false),
                |(vowels, prev, has_repeat, has_following), c| {
                    (
                        vowels + match c {
                            'a' | 'e' | 'i' | 'o' | 'u' => 1,
                            _ => 0
                        },
                        c,
                        has_repeat || c == prev,
                        has_following || match (prev, c) {
                            ('a', 'b') | ('c', 'd') | ('p', 'q') | ('x', 'y') =>
                                true,
                            _ => false
                        }
                    )
                });
            vowels >= 3 && has_repeat && !has_following
        })
        .count();
    println!("part1: {}", part1);
    let f = File::open("day_05_input.txt").unwrap();
    let part2 = BufReader::new(f)
        .lines()
        .filter_map(Result::ok)
        .filter(|line| {
            let has_pair = line.chars().zip(line.chars().skip(1)).enumerate().any(|(i, (a, b))| {
                line.chars().skip(i + 2).zip(line.chars().skip(i + 3)).any(|(c, d)| {
                    a == c && b == d
                })
            });
            let has_repeat = line.chars().zip(line.chars().skip(2)).any(|(a, b)| {
                a == b
            });
            has_pair && has_repeat
        })
        .count();
    println!("part2: {}", part2);
}
