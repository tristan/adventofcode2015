use std::fs::File;
use std::io::Read;

fn main() {
    let mut stack = vec![];
    let (sum, _, _, stack_sum, _, _) =
        File::open("day_12_input.txt").unwrap()
        .bytes().filter_map(Result::ok)
        .fold(
            (0i32, 0i32, false, 0i32, false, 0u8),
            |(sum, cur, neg, stack_sum, has_red, red_chars), b| match b {
                b'{' | b'[' => {
                    stack.push((stack_sum, has_red));
                    (sum, 0, false, 0, has_red, 0)
                },
                48 | 49 | 50 | 51 | 52 | 53 | 54 | 55 | 56 | 57 => (
                    sum, (cur * 10) + (b as i32 - 48), neg,
                    stack_sum, has_red, 0
                ),
                b'-' => (sum, cur, true, stack_sum, has_red, 0),
                b':' => {
                    (sum, cur, neg, stack_sum, has_red, 1)
                },
                b'"' => {
                    if red_chars == 5 {
                        (sum, cur, neg, stack_sum, true, 0)
                    } else if red_chars == 1 {
                        (sum, cur, neg, stack_sum, has_red, 2)
                    } else {
                        (sum, cur, neg, stack_sum, has_red, 0)
                    }
                },
                b'r' => {
                    if red_chars == 2 {
                        (sum, cur, neg, stack_sum, has_red, 3)
                    } else {
                        (sum, cur, neg, stack_sum, has_red, 0)
                    }
                },
                b'e' => {
                    if red_chars == 3 {
                        (sum, cur, neg, stack_sum, has_red, 4)
                    } else {
                        (sum, cur, neg, stack_sum, has_red, 0)
                    }
                },
                b'd' => {
                    if red_chars == 4 {
                        (sum, cur, neg, stack_sum, has_red, 5)
                    } else {
                        (sum, cur, neg, stack_sum, has_red, 0)
                    }
                },
                b',' | b'}' | b']' => {
                    let cur = if neg { cur * -1 } else { cur };
                    let stack_sum = stack_sum + cur;
                    let (stack_sum, has_red) = if b == b'}' || b == b']' {
                        let (sum, red) = stack.pop().unwrap();
                        if has_red {
                            (sum, red)
                        } else {
                            (sum + stack_sum, red)
                        }
                    } else {
                        (stack_sum, has_red)
                    };
                    (
                        sum + cur,
                        0,
                        false,
                        stack_sum,
                        has_red,
                        0
                    )
                },
                _ => (sum, cur, neg, stack_sum, has_red, 0)
            });
    println!("part1: {}", sum);
    println!("part2: {}", stack_sum);
}
