use std::fs::File;
use std::io::{BufRead, BufReader};

fn count_neighbors(lights: &Vec<Vec<bool>>, x: usize, y: usize) -> usize {
    (y.saturating_sub(1)..=(y + 1).min(99)).map(|dy| {
        (x.saturating_sub(1)..=(x + 1).min(99)).map(|dx| {
            if dx == x && dy == y { 0 } else if lights[dy][dx] { 1 } else { 0 }
        }).sum::<usize>()
    }).sum()
}

fn mutate(lights: &Vec<Vec<bool>>) -> Vec<Vec<bool>> {
    let mut next = Vec::with_capacity(100);
    for y in 0..100 {
        let mut row = Vec::with_capacity(100);
        for x in 0..100 {
            let neighbors = count_neighbors(lights, x, y);
            if lights[y][x] {
                if neighbors == 2 || neighbors == 3 {
                    row.push(true);
                } else {
                    row.push(false);
                }
            } else {
                if neighbors == 3 {
                    row.push(true);
                } else {
                    row.push(false);
                }
            }
        }
        next.push(row);
    }
    next
}

fn main() {
    let lights = BufReader::new(File::open("day_18_input.txt").unwrap())
        .lines().filter_map(Result::ok)
        .map(|line| line.chars().map(|c| c == '#').collect())
        .collect::<Vec<Vec<bool>>>();

    let lights_part1 = (0..100).fold(lights.clone(), |lights, _| {
        mutate(&lights)
    });

    let part1 = lights_part1.into_iter().map(|row| {
        row.into_iter().filter(|&v| v).count()
    }).sum::<usize>();
    println!("part1: {}", part1);

    let lights_part2 = (0..100).fold(lights, |lights, _| {
        let mut lights = mutate(&lights);
        lights[0][0] = true;
        lights[0][99] = true;
        lights[99][0] = true;
        lights[99][99] = true;
        lights
    });

    let part2 = lights_part2.into_iter().map(|row| {
        row.into_iter().filter(|&v| v).count()
    }).sum::<usize>();
    println!("part2: {}", part2);

}
