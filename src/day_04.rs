mod md5;

fn main() {
    let input = "iwrupvqb";
    let part1 = (0..).filter(|i| {
        let input = format!("{}{}", input, i);
        let md5 = md5::md5(&input);
        md5[0] == 0 && md5[1] == 0 && (md5[2] & 0xf0) == 0
    }).next().unwrap();
    let part2 = (part1..).filter(|i| {
        let input = format!("{}{}", input, i);
        let md5 = md5::md5(&input);
        md5[0] == 0 && md5[1] == 0 && md5[2] == 0
    }).next().unwrap();
    println!("part1: {}", part1);
    println!("part2: {}", part2);
}
