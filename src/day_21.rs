
#[derive(Clone)]
struct Character {
    hit_points: i32,
    damage: i32,
    armor: i32
}

const WEAPONS: [(i32, i32, i32); 5] = [
    // cost, damage, armor
    (8, 4, 0),
    (10, 5, 0),
    (25, 6, 0),
    (40, 7, 0),
    (74, 8, 0),
];

const ARMOR: [(i32, i32, i32); 6] = [
    (0, 0, 0),
    (13, 0, 1),
    (31, 0, 2),
    (53, 0, 3),
    (75, 0, 4),
    (102, 0, 5),
];

const RINGS: [(i32, i32, i32); 8] = [
    (0, 0, 0),
    (0, 0, 0),
    (20, 0, 1),
    (25, 1, 0),
    (40, 0, 2),
    (50, 2, 0),
    (80, 0, 3),
    (100, 3, 0),
];

impl Character {
    fn new(hit_points: i32, damage: i32, armor: i32) -> Character {
        Character { hit_points, damage, armor }
    }

    fn fight(&mut self, other: &mut Character) -> bool {
        loop {
            other.hit_points -= (self.damage - other.armor).max(1);
            if other.hit_points <= 0 {
                return true;
            }
            self.hit_points -= (other.damage - self.armor).max(1);
            if self.hit_points <= 0 {
                return false;
            }
        }
    }
}

fn main() {
    let boss = Character::new(100, 8, 2);
    let mut winning_fight_cost = i32::MAX;
    let mut loosing_fight_cost = 0;
    WEAPONS.iter().for_each(|weapon| {
        ARMOR.iter().for_each(|armor| {
            RINGS.iter().enumerate().for_each(|(idx1, ring1)| {
                RINGS.iter().enumerate().for_each(|(idx2, ring2)| {
                    if idx2 <= idx1 {
                        return;
                    }
                    let mut player = Character::new(
                        100,
                        weapon.1 + armor.1 + ring1.1 + ring2.1,
                        weapon.2 + armor.2 + ring1.2 + ring2.2
                    );
                    let cost = weapon.0 + armor.0 + ring1.0 + ring2.0;
                    let mut boss = boss.clone();
                    if player.fight(&mut boss) {
                        if cost < winning_fight_cost {
                            winning_fight_cost = cost;
                        }
                    } else {
                        if cost > loosing_fight_cost {
                            loosing_fight_cost = cost;
                        }
                    }
                })
            })
        })
    });
    println!("part1: {}", winning_fight_cost);
    println!("part2: {}", loosing_fight_cost);
}
