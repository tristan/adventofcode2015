fn u8_to_single_digit_vec(input: u8, result: &mut Vec<u8>) {
    if input >= 10 {
        u8_to_single_digit_vec(input / 10, result);
    }
    result.push(input % 10);
}

fn step(start: &[u8]) -> Vec<u8> {
    let mut result = Vec::with_capacity(start.len());
    let (chr, count) = start.iter().fold(None, |acc, &c| {
        if let Some((chr, count)) = acc {
            if chr == c {
                Some((chr, count + 1))
            } else {
                u8_to_single_digit_vec(count, &mut result);
                result.push(chr);
                Some((
                    c,
                    1
                ))
            }
        } else {
            Some((c, 1))
        }
    }).unwrap();
    u8_to_single_digit_vec(count, &mut result);
    result.push(chr);
    result
}

fn main() {
    let input = "1113222113".chars()
        .map(|c| c as u8 - '0' as u8)
        .collect::<Vec<_>>();
    let s = std::time::Instant::now();
    let result1 = (0..40).fold(input, |start, _| {
        step(&start)
    });
    println!("time: {:?}", s.elapsed());
    println!("part1: {}", result1.len());
    let s = std::time::Instant::now();
    let result2 = (0..10).fold(result1, |start, _| {
        step(&start)
    });
    println!("time: {:?}", s.elapsed());
    println!("part2: {}", result2.len());
}
