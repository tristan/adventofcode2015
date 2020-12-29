use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let f = File::open("day_14_input.txt").unwrap();
    let race_end = 2503;
    let deer = BufReader::new(f)
        .lines()
        .filter_map(Result::ok)
        .map(|line| {
            let mut i1 = line.split(" can fly ");
            let _name = i1.next().unwrap();
            let mut i2 = i1.next().unwrap().split(" km/s for ");
            let speed = i2.next().unwrap().parse::<usize>().unwrap();
            let mut i3 = i2.next().unwrap()
                .split(" seconds, but then must rest for ");
            let time = i3.next().unwrap().parse::<usize>().unwrap();
            let mut i4 = i3.next().unwrap().split(" ");
            let rest = i4.next().unwrap().parse::<usize>().unwrap();
            let cycle_time = time + rest;
            let cycle_distance = speed * time;
            let full_cycle_count = race_end / cycle_time;
            let remaining_time = race_end - (full_cycle_count * cycle_time);
            let remaining_dist = remaining_time.min(time) * speed;
            (
                speed, time, rest,
                full_cycle_count * cycle_distance + remaining_dist
            )
        })
        .collect::<Vec<_>>();
    let part1 = deer.iter().map(|x| x.3)
        .max().unwrap();
    println!("part1: {}", part1);

    let mut scores = vec![0; deer.len()];
    let mut states = vec![(0, 0, 0); deer.len()];
    (0..race_end).for_each(|_| {
        let mut max_val = 0;
        deer.iter().zip(states.iter_mut())
            .for_each(|(deer, mut state)| {
                if state.2 > 0 {
                    state.2 -= 1;
                } else {
                    state.0 += deer.0;
                    state.1 += 1;
                    if state.1 >= deer.1 {
                        state.2 = deer.2;
                        state.1 = 0;
                    }
                }
                if max_val < state.0 {
                    max_val = state.0;
                }
            });
        states.iter().zip(scores.iter_mut()).for_each(|(state, score)| {
            if state.0 == max_val {
                *score += 1;
            }
        });
    });
    let part2 = scores.iter().max().unwrap();
    println!("part2: {}", part2);
}
