use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let target_children = 3;
    let target_cats = 7;
    let target_samoyeds = 2;
    let target_pomeranians = 3;
    let target_akitas = 0;
    let target_vizslas = 0;
    let target_goldfish = 5;
    let target_trees = 3;
    let target_cars = 2;
    let target_perfumes = 1;

    let f = File::open("day_16_input.txt").unwrap();
    let (sue1, _, sue2, _) = BufReader::new(f)
        .lines().filter_map(Result::ok)
        .fold((0, 0, 0, 0), |acc, line| {
            let sue_num_idx = line.find(":").unwrap();
            let sue_num = line[4..sue_num_idx].parse::<usize>().unwrap();
            let matches = line[sue_num_idx + 2..].split(", ")
                .map(|compound| {
                    let mut s1 = compound.split(": ");
                    let compound_name = s1.next().unwrap();
                    let compound_val = s1.next().unwrap()
                        .parse::<isize>().unwrap();
                    match compound_name {
                        "children" => (
                            target_children == compound_val,
                            target_children == compound_val,
                        ),
                        "cats" => (
                            target_cats == compound_val,
                            target_cats < compound_val,
                        ),
                        "samoyeds" => (
                            target_samoyeds == compound_val,
                            target_samoyeds == compound_val,
                        ),
                        "pomeranians" => (
                            target_pomeranians == compound_val,
                            target_pomeranians > compound_val,
                        ),
                        "akitas" => (
                            target_akitas == compound_val,
                            target_akitas == compound_val,
                        ),
                        "vizslas" => (
                            target_vizslas == compound_val,
                            target_vizslas == compound_val,
                        ),
                        "goldfish" => (
                            target_goldfish == compound_val,
                            target_goldfish > compound_val,
                        ),
                        "trees" => (
                            target_trees == compound_val,
                            target_trees < compound_val,
                        ),
                        "cars" => (
                            target_cars == compound_val,
                            target_cars == compound_val,
                        ),
                        "perfumes" => (
                            target_perfumes == compound_val,
                            target_perfumes == compound_val,
                        ),
                        _ => panic!("unknown compound: {}", compound_name)
                    }
                })
                .collect::<Vec<(bool, bool)>>();
            let (sue1, val1) = {
                let count = matches.iter().filter(|(v, _)| *v).count();
                if count > acc.1 {
                    (sue_num, count)
                } else {
                    (acc.0, acc.1)
                }
            };
            let (sue2, val2) = {
                let count = matches.iter().filter(|(_, v)| *v).count();
                if count > acc.3 {
                    (sue_num, count)
                } else {
                    (acc.2, acc.3)
                }
            };
            (sue1, val1, sue2, val2)
        });
    println!("part1: {}", sue1);
    println!("part2: {}", sue2);
}
