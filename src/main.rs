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
}
