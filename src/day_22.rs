
#[derive(Clone, Debug)]
enum Effect {
    Shield(i32),
    Poison(i32),
    Recharge(i32),
    HardMode,
}

impl PartialEq for Effect {
    fn eq(&self, rhs: &Effect) -> bool {
        match (self, rhs) {
            (Effect::Shield(_), Effect::Shield(_)) => true,
            (Effect::Poison(_), Effect::Poison(_)) => true,
            (Effect::Recharge(_), Effect::Recharge(_)) => true,
            (Effect::HardMode, Effect::HardMode) => true,
            (_, _) => false
        }
    }
}

const SPELLS: [(i32, i32, i32, Option<Effect>); 5] = [
    (53, 4, 0, None), // Magic Missile
    (73, 2, 2, None), // Drain
    (113, 0, 0, Some(Effect::Shield(6))), // Shield
    (173, 0, 0, Some(Effect::Poison(6))), // Poison
    (229, 0, 0, Some(Effect::Recharge(5))), // Recharge
];

#[derive(Clone)]
struct Character {
    hit_points: i32,
    mana: i32,
    damage: i32,
    armor: i32,
    effects: Vec<Effect>
}

impl Character {
    fn new(hit_points: i32, mana: i32, damage: i32, armor: i32) -> Character {
        Character { hit_points, mana, damage, armor, effects: vec![] }
    }
}

fn fight(
    mut player: Character,
    mut boss: Character,
    turn: i32,
    mana_used: i32,
    min_mana: &mut i32
) {
    player.effects = player.effects.clone().into_iter()
        .filter_map(|effect| {
            match effect {
                Effect::Shield(time) => {
                    let time = time - 1;
                    if time == 0 {
                        player.armor -= 7;
                        None
                    } else {
                        Some(Effect::Shield(time))
                    }
                },
                Effect::Poison(time) => {
                    boss.hit_points -= 3;
                    let time = time - 1;
                    if time == 0 {
                        None
                    } else {
                        Some(Effect::Poison(time))
                    }
                },
                Effect::Recharge(time) => {
                    player.mana += 101;
                    let time = time - 1;
                    if time == 0 {
                        None
                    } else {
                        Some(Effect::Recharge(time))
                    }
                },
                Effect::HardMode => {
                    if turn % 2 == 0 {
                        player.hit_points -= 1;
                    }
                    Some(Effect::HardMode)
                },
            }
        }).collect();
    if player.hit_points <= 0 {
        // LOST!
        return;
    }
    if boss.hit_points <= 0 {
        // WON!
        if mana_used < *min_mana {
            *min_mana = mana_used;
        }
        return;
    }
    if turn % 2 == 0 {
        // player turn
        for (cost, damage, heal, effect) in &SPELLS {
            if let Some(effect) = effect {
                if player.effects.contains(effect) {
                    continue;
                }
            }
            if *cost > player.mana {
                continue;
            }
            let mut player = player.clone();
            let mut boss = boss.clone();
            player.mana -= cost;
            let mana_used = mana_used + cost;
            if mana_used > *min_mana {
                // short circuit
                continue;
            }
            if *damage > 0 {
                boss.hit_points -= damage;
                if boss.hit_points <= 0 {
                    // WON!
                    if mana_used < *min_mana {
                        *min_mana = mana_used;
                    }
                    continue;
                }
            }
            player.hit_points += heal;
            if let Some(effect) = effect {
                if *effect == Effect::Shield(0) {
                    player.armor += 7;
                }
                player.effects.push(effect.clone());
            }
            fight(player, boss, turn + 1, mana_used, min_mana);
        }
    } else {
        // boss turn
        player.hit_points -= (boss.damage - player.armor).max(1);
        if player.hit_points <= 0 {
            // LOST!
            return;
        } else {
            fight(player, boss, turn + 1, mana_used, min_mana);
        }
    }
}

fn main() {
    let boss = Character::new(58, 0, 9, 0);
    let mut player = Character::new(50, 500, 0, 0);
    let mut min_mana = i32::MAX;
    fight(player.clone(), boss.clone(), 0, 0, &mut min_mana);
    println!("part1: {}", min_mana);
    let mut min_mana = i32::MAX;
    player.effects.push(Effect::HardMode);
    fight(player, boss, 0, 0, &mut min_mana);
    println!("part2: {}", min_mana);
}
