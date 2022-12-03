use std::fs;

fn puzzle1(input: &str) {
    let rations = input.split('\n');

    let mut calories_by_elf: Vec<i32> = Vec::new();
    for ration in rations {
        if ration == "" {
            calories_by_elf.push(0);
        } else {
            if let Some(last) = calories_by_elf.last_mut() {
                *last += ration.parse::<i32>().expect("This should be an int!");
            }
        }
    }

    println!(
        "{}",
        calories_by_elf.iter().fold(std::i32::MIN, |a, b| a.max(*b))
    );

    let mut maxes = (std::i32::MIN, std::i32::MIN, std::i32::MIN);

    for cals in calories_by_elf {
        if cals > maxes.0 {
            maxes.2 = maxes.1;
            maxes.1 = maxes.0;
            maxes.0 = cals;
        } else if cals > maxes.1 {
            maxes.2 = maxes.1;
            maxes.1 = cals;
        } else if cals > maxes.2 {
            maxes.2 = cals;
        }
    }

    println!("{}", maxes.0 + maxes.1 + maxes.2);
}

fn puzzle2(input: &str) {
    let rounds = input.split('\n');
    let mut score = 0;
    let mut real_score = 0;

    for round in rounds {
        match round.split_once(' ') {
            Some((opponent, me)) => {
                let my_move = me.chars().next().unwrap();
                let opponents_move = opponent.chars().next().unwrap();

                match my_move {
                    'X' => {
                        score += 1;
                        real_score += 0;
                        match opponents_move {
                            'A' => {
                                score += 3;
                                real_score += 3;
                            }
                            'B' => {
                                score += 0;
                                real_score += 1;
                            }
                            'C' => {
                                score += 6;
                                real_score += 2;
                            }
                            _ => {
                                println!(
                                    "Invalid round! opponent: {} me: {}",
                                    opponents_move, my_move
                                );
                            }
                        };
                    }
                    'Y' => {
                        score += 2;
                        real_score += 3;
                        match opponents_move {
                            'A' => {
                                score += 6;
                                real_score += 1;
                            }
                            'B' => {
                                score += 3;
                                real_score += 2;
                            }
                            'C' => {
                                score += 0;
                                real_score += 3;
                            }
                            _ => {
                                println!(
                                    "Invalid round! opponent: {} me: {}",
                                    opponents_move, my_move
                                );
                            }
                        };
                    }
                    'Z' => {
                        score += 3;
                        real_score += 6;
                        match opponents_move {
                            'A' => {
                                score += 0;
                                real_score += 2;
                            }
                            'B' => {
                                score += 6;
                                real_score += 3;
                            }
                            'C' => {
                                score += 3;
                                real_score += 1;
                            }
                            _ => {
                                println!(
                                    "Invalid round! opponent: {} me: {}",
                                    opponents_move, my_move
                                );
                            }
                        };
                    }
                    _ => {
                        println!(
                            "Invalid round! opponent: {} me: {}",
                            opponents_move, my_move
                        );
                    }
                }
            }
            None => {
                continue;
            }
        }
    }

    println!("My score: {} My real score: {}", score, real_score);
}

fn to_priority(c: char) -> usize {
    match c {
        'a'..='z' => c as usize - 'a' as usize + 1,
        'A'..='Z' => c as usize - 'A' as usize + 27,
        _ => {
            panic!()
        }
    }
}

fn puzzle3(input: &str) {
    let rucksacks = input.split('\n');
    let mut priority_accumulator = 0;

    for rucksack in rucksacks {
        let mut has_type: [(bool, bool); 52] = [(false, false); 52];
        let (first_half, second_half) = rucksack.split_at(rucksack.len() / 2);

        for item in first_half.chars() {
            has_type[to_priority(item) - 1].0 = true;
        }

        for item in second_half.chars() {
            let priority = to_priority(item);
            if has_type[priority - 1].0 && !has_type[priority - 1].1 {
                priority_accumulator += to_priority(item);
            }
            has_type[priority - 1].1 = true;
        }
    }

    println!("Accumulated priority: {}", priority_accumulator);
}

fn puzzle3_part2(input: &str) {
    // I could have done it functionally, but I wanted to save on the accumulation pass.
    let mut rucksacks = input.lines().into_iter();
    let mut priority_accumulator = 0;

    loop {
        let group: (&str, &str, &str) = (
            match rucksacks.next() {
                Some(x) => x,
                None => break,
            },
            match rucksacks.next() {
                Some(x) => x,
                None => break,
            },
            match rucksacks.next() {
                Some(x) => x,
                None => break,
            },
        );

        let mut has_type: [(bool, bool, bool); 52] = [(false, false, false); 52];

        for item in group.0.chars() {
            has_type[to_priority(item) - 1].0 = true;
        }

        for item in group.1.chars() {
            has_type[to_priority(item) - 1].1 = true;
        }

        for item in group.2.chars() {
            let priority = to_priority(item);
            if has_type[priority - 1].0 && has_type[priority - 1].1 && !has_type[priority - 1].2 {
                priority_accumulator += priority;
            }
            has_type[priority - 1].2 = true;
        }
    }

    println!("Accumulated priority for badges: {}", priority_accumulator);
}

fn run_puzzle(fname: &str, f: &dyn Fn(&str)) {
    println!("Input file {}:", fname);

    let input = fs::read_to_string(fname).expect("Can't read the file!");
    f(&input);
}

fn main() {
    run_puzzle("puzzle1_0.txt", &puzzle1);
    run_puzzle("puzzle1_1.txt", &puzzle1);
    run_puzzle("puzzle2_0.txt", &puzzle2);
    run_puzzle("puzzle2_1.txt", &puzzle2);
    run_puzzle("puzzle3_0.txt", &puzzle3);
    run_puzzle("puzzle3_1.txt", &puzzle3);
    run_puzzle("puzzle3_0.txt", &puzzle3_part2);
    run_puzzle("puzzle3_1.txt", &puzzle3_part2);
}
