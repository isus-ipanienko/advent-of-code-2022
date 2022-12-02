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

    println!("{}", calories_by_elf.iter().fold(std::i32::MIN, |a,b| a.max(*b)));

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
    };

    println!("{}", maxes.0 + maxes.1 + maxes.2);
}

fn run_puzzle(fname: &str, f: &dyn Fn(&str)) {
    println!("Input file {}:", fname);

    let input = fs::read_to_string(fname).expect("Can't read the file!");
    f(&input);
}

fn main() {
    run_puzzle("puzzle1_0.txt", &puzzle1);
    run_puzzle("puzzle1_1.txt", &puzzle1);
}
