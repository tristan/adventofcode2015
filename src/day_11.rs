
fn increment(value: &mut Vec<u8>) {
    let mut idx = 0;
    loop {
        if idx >= value.len() {
            value.push(0);
        }
        value[idx] = (value[idx] + 1) % 26;
        if value[idx] == 0 {
            idx += 1;
        } else {
            break;
        }
    }

}

const I_VAL: u8 = 'i' as u8 - 'a' as u8;
const O_VAL: u8 = 'o' as u8 - 'a' as u8;
const L_VAL: u8 = 'l' as u8 - 'a' as u8;

fn validate(value: &[u8]) -> bool {
    let mut has_increasing = false;
    let mut has_repeating = 0;
    let mut first_repeating = 0;
    for idx in 0..value.len() {
        let one = value[idx];
        if one == I_VAL || one == O_VAL || one == L_VAL {
            return false;
        }
        if !has_increasing && idx < value.len() - 2 {
            let two = value[idx + 1];
            let three = value[idx + 2];
            has_increasing = one == two + 1 && two == three + 1;
        }
        if (has_repeating == 0 || has_repeating == 1 && idx > first_repeating + 1) && idx < value.len() - 1 {
            let two = value[idx + 1];
            if one == two {
                has_repeating += 1;
                first_repeating = idx;
            }
        }
    }
    has_increasing && has_repeating == 2
}

fn main() {
    let mut input = "cqjxjnds"
        .chars().map(|c| c as u8 - 'a' as u8)
        .rev()
        .collect::<Vec<u8>>();
    loop {
        increment(&mut input);
        if validate(&input) {
            break;
        }
    }
    let part1 = input.iter()
        .map(|v| (v + 'a' as u8) as char)
        .rev()
        .collect::<String>();
    println!("part1: {}", part1);
    loop {
        increment(&mut input);
        if validate(&input) {
            break;
        }
    }
    let part2 = input.iter()
        .map(|v| (v + 'a' as u8) as char)
        .rev()
        .collect::<String>();
    println!("part2: {}", part2);
}
