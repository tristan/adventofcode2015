use std::fs::File;
use std::io::{BufRead, BufReader};

struct MixIter {
    current: [isize; 4],
    iteration: usize
}

impl<'a> MixIter {
    fn new() -> MixIter {
        MixIter { current: [100, 0, 0, 0], iteration: 0 }
    }
}

impl Iterator for MixIter {
    type Item = [isize; 4];
    fn next(&mut self) -> Option<Self::Item> {
        if self.iteration == 0 {
            self.iteration += 1;
            Some(self.current)
        } else if self.current[3] == 100 {
            None
        } else {
            if self.current[0] == 0 {
                if self.current[2] + self.current[3] == 100 {
                    self.current[3] += 1;
                    self.current[2] = 0;
                } else {
                    self.current[2] += 1;
                }
                self.current[1] = 0;
                self.current[0] = 100 - self.current[1] - self.current[2] - self.current[3];
            } else {
                self.current[0] -= 1;
                self.current[1] += 1;
            }
            Some(self.current)
        }
    }
}

fn main() {
    let f = File::open("day_15_input.txt").unwrap();
    let ingredients = BufReader::new(f)
        .lines()
        .filter_map(Result::ok)
        .map(|line| {
            let mut s1 = line.split(": ");
            let _name = s1.next().unwrap();
            let mut properties = s1.next().unwrap().split(", ")
                .map(|prop| {
                    let mut s1 = prop.split(" ");
                    let _prop_name = s1.next().unwrap();
                    s1.next().unwrap().parse::<isize>().unwrap()
                });
            (
                properties.next().unwrap(),
                properties.next().unwrap(),
                properties.next().unwrap(),
                properties.next().unwrap(),
                properties.next().unwrap()
            )
        })
        .collect::<Vec<(isize, isize, isize, isize, isize)>>();

    let part1 = MixIter::new().map(|amounts| {
        let cap =
            ingredients[0].0 * amounts[0] +
            ingredients[1].0 * amounts[1] +
            ingredients[2].0 * amounts[2] +
            ingredients[3].0 * amounts[3]
            ;
        let dur =
            ingredients[0].1 * amounts[0] +
            ingredients[1].1 * amounts[1] +
            ingredients[2].1 * amounts[2] +
            ingredients[3].1 * amounts[3]
            ;
        let flav =
            ingredients[0].2 * amounts[0] +
            ingredients[1].2 * amounts[1] +
            ingredients[2].2 * amounts[2] +
            ingredients[3].2 * amounts[3]
            ;
        let txt =
            ingredients[0].3 * amounts[0] +
            ingredients[1].3 * amounts[1] +
            ingredients[2].3 * amounts[2] +
            ingredients[3].3 * amounts[3]
            ;
        cap.max(0) * dur.max(0) * flav.max(0) * txt.max(0)
    }).max().unwrap();
    println!("part1: {}", part1);
    let part2 = MixIter::new().filter_map(|amounts| {
        let cals =
            ingredients[0].4 * amounts[0] +
            ingredients[1].4 * amounts[1] +
            ingredients[2].4 * amounts[2] +
            ingredients[3].4 * amounts[3]
            ;
        if cals == 500 {
            let cap =
                ingredients[0].0 * amounts[0] +
                ingredients[1].0 * amounts[1] +
                ingredients[2].0 * amounts[2] +
                ingredients[3].0 * amounts[3]
                ;
            let dur =
                ingredients[0].1 * amounts[0] +
                ingredients[1].1 * amounts[1] +
                ingredients[2].1 * amounts[2] +
                ingredients[3].1 * amounts[3]
                ;
            let flav =
                ingredients[0].2 * amounts[0] +
                ingredients[1].2 * amounts[1] +
                ingredients[2].2 * amounts[2] +
                ingredients[3].2 * amounts[3]
                ;
            let txt =
                ingredients[0].3 * amounts[0] +
                ingredients[1].3 * amounts[1] +
                ingredients[2].3 * amounts[2] +
                ingredients[3].3 * amounts[3]
                ;
            Some(cap.max(0) * dur.max(0) * flav.max(0) * txt.max(0))
        } else {
            None
        }
    }).max().unwrap();
    println!("part2: {}", part2);

}
