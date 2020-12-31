use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::{HashMap, HashSet};

fn main() {
    let mut transforms: HashMap<String, Vec<String>> = HashMap::new();
    let source = BufReader::new(File::open("day_19_input.txt").unwrap())
        .lines().filter_map(Result::ok).filter(|line| line.len() > 0)
        .filter_map(|line| {
            if let Some(idx) = line.find(" => ") {
                let mol = line[..idx].to_string();
                let transform = line[idx + 4..].to_string();
                transforms.entry(mol)
                    .or_insert_with(|| vec![])
                    .push(transform);
                None
            } else {
                Some(line)
            }
        })
        .next().unwrap();

    let mut molecules: HashSet<String> = HashSet::new();
    transforms.iter().for_each(|(start, options)| {
        options.iter().for_each(|replacement| {
            let mut prefix = 0;
            while let Some(idx) = source[prefix..].find(start) {
                prefix += idx;
                let mol = format!(
                    "{}{}{}",
                    &source[..prefix],
                    replacement,
                    &source[prefix + start.len()..]
                );
                molecules.insert(mol);
                prefix += start.len();
            }
        })
    });

    println!("part1: {}", molecules.len());

    // I had to cheat, solution taken from
    // https://www.reddit.com/r/adventofcode/comments/3xflz8/day_19_solutions/
    let source = source.replace("Rn", "(").replace("Ar", ")").replace("Y", ",");
    let (tokens, brackets, commas) = source
        .chars()
        .fold((0, 0, 0), |(tokens, brackets, commas), w| {
            if w == '(' || w == ')' {
                (tokens, brackets + 1, commas)
            } else if w == ',' {
                (tokens, brackets, commas + 1)
            } else if w.is_uppercase() {
                (tokens + 1, brackets, commas)
            } else {
                (tokens, brackets, commas)
            }
        });
    let part2 = (tokens + brackets + commas) - brackets - 2 * commas - 1;
    println!("part2: {}", part2);
}
