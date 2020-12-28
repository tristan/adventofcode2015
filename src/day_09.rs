use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::{HashSet, HashMap};

// enum Town {
//     AlphaCentauri,
//     Arbre,
//     Faerun,
//     Norrath,
//     Snowdin,
//     Straylight,
//     Tambi,
//     Tristram,
// }

//struct Graph(HashMap<String, HashMap<String, usize>>);

fn main() {
    let f = File::open("day_09_input.txt").unwrap();
    let mut graph: HashMap<String, HashMap<String, usize>> = HashMap::new();
    let mut towns: HashSet<String> = HashSet::new();
    BufReader::new(f)
        .lines().filter_map(Result::ok)
        .for_each(|line| {
            let mut split = line.split(" to ");
            let town1 = split.next().unwrap().to_string();
            let mut split = split.next().unwrap().split(" = ");
            let town2 = split.next().unwrap().to_string();
            let dist = split.next().unwrap().parse::<usize>().unwrap();
            towns.insert(town1.clone());
            towns.insert(town2.clone());
            graph.entry(town1.clone())
                .or_insert_with(|| HashMap::new())
                .insert(town2.clone(), dist);
            graph.entry(town2)
                .or_insert_with(|| HashMap::new())
                .insert(town1, dist);
        });

    let part1 = towns.iter().map(|town| {
        let mut towns = towns.clone();
        let mut town = town.clone();
        towns.remove(&town);
        let mut total_dist = 0;
        while towns.len() > 0 {
            let (next, dist) = graph.get(&town).unwrap()
                .iter().fold(None, |acc, (town2, dist)| -> Option<(&str, usize)> {
                    let dist = *dist;
                    if towns.contains(town2) {
                        match acc {
                            Some((_, prev_dist)) => if prev_dist > dist {
                                Some((town2, dist))
                            } else {
                                acc
                            },
                            None => {
                                Some((town2, dist))
                            }
                        }
                    } else {
                        acc
                    }
                }).unwrap();
            total_dist += dist;
            town = next.to_string();
            towns.remove(&town);
        }
        total_dist
    }).min().unwrap();

    println!("part1: {}", part1);

    let part2 = towns.iter().map(|town| {
        let mut towns = towns.clone();
        let mut town = town.clone();
        towns.remove(&town);
        let mut total_dist = 0;
        while towns.len() > 0 {
            let (next, dist) = graph.get(&town).unwrap()
                .iter().fold(None, |acc, (town2, dist)| -> Option<(&str, usize)> {
                    let dist = *dist;
                    if towns.contains(town2) {
                        match acc {
                            Some((_, prev_dist)) => if prev_dist < dist {
                                Some((town2, dist))
                            } else {
                                acc
                            },
                            None => {
                                Some((town2, dist))
                            }
                        }
                    } else {
                        acc
                    }
                }).unwrap();
            total_dist += dist;
            town = next.to_string();
            towns.remove(&town);
        }
        total_dist
    }).max().unwrap();

    println!("part2: {}", part2);
}
