use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::{HashSet, HashMap};

struct Permutations<'a, T> {
    inner: Vec<&'a T>,
    i: usize,
    c: Vec<usize>
}

impl<'a, T> Permutations<'a, T> {
    fn new(inner: &'a [T]) -> Permutations<'a, T> {
        Permutations {
            inner: inner.iter().collect(),
            i: 0,
            c: vec![0; inner.len()]
        }
    }
}

impl<'a, T> Iterator for Permutations<'a, T> {
    type Item = Vec<&'a T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.i == 0 {
            self.i = 1;
            return Some(self.inner.clone());
        }
        while self.i <= self.inner.len() {
            let i = self.i - 1;
            if self.c[i] < i {
                if i % 2 == 0 {
                    self.inner.swap(0, i);
                } else {
                    self.inner.swap(self.c[i], i);
                }
                self.c[i] += 1;
                self.i = 1;
                return Some(self.inner.clone())
            } else {
                self.c[self.i - 1] = 0;
                self.i += 1;
            }
        }
        return None;
    }
}

fn main() {
    let f = File::open("day_13_input.txt").unwrap();
    let mut graph: HashMap<String, HashMap<String, isize>> = HashMap::new();
    let mut people: HashSet<String> = HashSet::new();
    BufReader::new(f)
        .lines().filter_map(Result::ok)
        .for_each(|line| {
            let mut split = line.split(" would ");
            let person1 = split.next().unwrap().to_string();
            let mut split = split.next().unwrap()
                .split(" happiness units by sitting next to ");
            let mut units_text = split.next().unwrap()
                .split(" ");
            let units = match units_text.next().unwrap() {
                "lose" => -1,
                "gain" => 1,
                _ => panic!("invalid input")
            } * units_text.next().unwrap()
                .parse::<isize>().unwrap();
            let person2 = split.next().unwrap();
            let person2 = person2[..person2.len() - 1].to_string();
            people.insert(person1.clone());
            people.insert(person2.clone());
            let happiness = graph.entry(person1.clone())
                .or_insert_with(|| HashMap::new())
                .entry(person2.clone())
                .or_insert(0);
            *happiness += units;
        });
    let mut people = people.into_iter().collect::<Vec<String>>();
    let part1 = Permutations::new(&people).map(|people| -> isize {
        people.iter().zip(people.iter().cycle().skip(1))
            .map(|(&p1, &p2)| {
                let p1units = graph.get(p1).unwrap()
                    .get(p2).unwrap();
                let p2units = graph.get(p2).unwrap()
                    .get(p1).unwrap();
                p1units + p2units
            }).sum()
    }).max().unwrap();
    println!("part1: {}", part1);
    people.push("Tristan".to_string());
    let part2 = Permutations::new(&people).map(|people| -> isize {
        people.iter().zip(people.iter().cycle().skip(1))
            .map(|(&p1, &p2)| {
                let p1units = if let Some(units) = graph.get(p1) {
                    units.get(p2).unwrap_or(&0)
                } else {
                    &0
                };
                let p2units = if let Some(units) = graph.get(p2) {
                    units.get(p1).unwrap_or(&0)
                } else {
                    &0
                };
                p1units + p2units
            }).sum()
    }).max().unwrap();
    println!("part2: {}", part2);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_permutate() {
        let data = vec![1, 2, 3];
        let mut p = Permutations::new(&data);
        assert_eq!(p.next(), Some(vec![&1, &2, &3]));
        assert_eq!(p.next(), Some(vec![&2, &1, &3]));
        assert_eq!(p.next(), Some(vec![&3, &1, &2]));
        assert_eq!(p.next(), Some(vec![&1, &3, &2]));
        assert_eq!(p.next(), Some(vec![&2, &3, &1]));
        assert_eq!(p.next(), Some(vec![&3, &2, &1]));
        assert_eq!(p.next(), None);
    }
}
