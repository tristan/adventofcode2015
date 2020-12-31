fn main() {
    let input = 29000000;
    let start = std::time::Instant::now();
    let part1: usize = (2..).map(|i| {
        let mut result = 10 + i * 10;
        let mut n = 2;
        let limit = (i as f64).sqrt().floor() as usize;
        while n <= limit {
            if i % n == 0 {
                result += n * 10;
                let x = i / n;
                if x != n {
                    result += (i / n) * 10;
                }
            }
            n += 1;
        }
        (result, i)
    }).skip_while(|&(r, _)| r < input)
        .map(|(_, i)| i)
        .next().unwrap();
    println!("part1: {} ({:?})", part1, start.elapsed());
    let start = std::time::Instant::now();
    let part2: usize = (2..).map(|i| {
        let mut result = 0;
        let mut n = 1;
        let limit = (i as f64).sqrt().floor() as usize;
        while n <= limit {
            if i % n == 0 {
                if n * 50 >= i {
                    result += 11 * n;
                }
                let x = i / n;
                if x != n && x * 50 >= i {
                    result += 11 * x
                }
            }
            n += 1;
        }
        (result, i)
    }).skip_while(|&(r, _)| r < input)
        .map(|(_, i)| i)
        .next().unwrap();
    println!("part2: {} ({:?})", part2, start.elapsed());
}
