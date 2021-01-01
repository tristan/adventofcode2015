fn main() {
    // (row, col)
    let input = (3010, 3019);

    let mut current: u128 = 20151125;
    let start = std::time::Instant::now();
    'outer: for s in 2.. {
        for (col, row) in (1..=s).zip((1..=s).rev()) {
            current = (current * 252533) % 33554393;
            if row == input.0 && col == input.1 {
                break 'outer;
            }
        }
    }
    println!("part1: {} ({:?})", current, start.elapsed());
}
