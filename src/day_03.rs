use std::fs::File;
use std::io::Read;
use std::collections::HashMap;

fn do_move(
    visits: &mut HashMap<(isize, isize), usize>,
    c: u8,
    x: isize,
    y: isize
) -> (isize, isize) {
    let (x, y) = match c {
        b'^' => (x, y + 1),
        b'v' => (x, y - 1),
        b'<' => (x - 1, y),
        b'>' => (x + 1, y),
        _ => panic!("invalid input: {:?}", c as char)
    };
    let v = visits.get(&(x, y)).copied().unwrap_or(0) + 1;
    visits.insert((x, y), v);
    (x, y)
}

fn main() {
    let mut visits_p1 = HashMap::new();
    let mut visits_p2 = HashMap::new();
    visits_p1.insert((0, 0), 1);
    visits_p2.insert((0, 0), 2);
    File::open("day_03_input.txt").unwrap()
        .bytes()
        .filter_map(Result::ok)
        .filter(|&c| c != b'\n')
        .enumerate()
        .fold(
            (0isize, 0isize, 0isize, 0isize, 0isize, 0isize),
            |(x, y, sx, sy, rx, ry), (i, c)| {
                // part 1
                let (x, y) = do_move(&mut visits_p1, c, x, y);

                // part 2
                let (sx, sy, rx, ry) = if i % 2 == 0 {
                    let (sx, sy) = do_move(&mut visits_p2, c, sx, sy);
                    (sx, sy, rx, ry)
                } else {
                    let (rx, ry) = do_move(&mut visits_p2, c, rx, ry);
                    (sx, sy, rx, ry)
                };

                (x, y, sx, sy, rx, ry)
            }
        );
    let part1 = visits_p1.len();
    let part2 = visits_p2.len();
    println!("part1: {}", part1);
    println!("part2: {}", part2);
}
