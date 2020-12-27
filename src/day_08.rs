use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let f = File::open("day_08_input.txt").unwrap();
    let input = BufReader::new(f)
        .lines().filter_map(Result::ok)
        .filter(|line| line.len() > 0)
        .collect::<Vec<String>>();
    let start = std::time::Instant::now();
    let (code_bytes, decoded_bytes, encoded_bytes) = input.iter()
        .fold((0, 0, 0), |(cur_code_bytes, cur_dec_bytes, cur_enc_bytes), line| {
            //dbg!(&line);
            let mut line_str = &line[1..];
            let mut last_idx = 1;
            let mut dec_bytes = 0;
            let mut enc_bytes = 0;
            while let Some(idx) = line_str.find("\\") {
                match &line_str[idx..idx + 2] {
                    "\\\\" | "\\\"" => {
                        dec_bytes += idx + 1;
                        enc_bytes += idx + 4;
                        last_idx += idx + 2;
                        line_str = &line[last_idx..];
                    },
                    "\\x" => {
                        dec_bytes += idx + 1;
                        enc_bytes += idx + 5;
                        last_idx += idx + 4;
                        line_str = &line[last_idx..];
                    }
                    _ => {
                        panic!(
                            "unexpected: {} ({}, {})",
                            &line_str[last_idx + idx..last_idx + idx + 1],
                            last_idx + idx,
                            last_idx + idx + 1
                        );
                    }
                }
            }
            dec_bytes += line_str.len() - 1;
            enc_bytes += line_str.len() + 5;
            (
                cur_code_bytes + line.len(),
                cur_dec_bytes + dec_bytes,
                cur_enc_bytes + enc_bytes
            )
        });
    println!("time: {:?}", start.elapsed());
    let part1 = code_bytes - decoded_bytes;
    println!("part1: {}", part1);
    let part2 = encoded_bytes - code_bytes;
    println!("part2: {}", part2);
}
