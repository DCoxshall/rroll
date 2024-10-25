use clap::Parser;
use rand::prelude::*;
use regex::Regex;

#[derive(Parser)]
struct Cli {
    #[clap(num_args = 1..)]
    pattern: Vec<String>,
}

/// Represents a roll of a collection of the same dice.
#[derive(Debug)]
struct Roll {
    /// The number of dice to roll. For "1d20+3", `times` is 1.
    times: i32,
    /// The number of sides that the rolling dice have. For "1d20+3", `sides` is 20.
    sides: i32,
    /// The added "bonus". For "1d20+3", `delta` is 3. `delta` need not be positive.
    delta: i32,
}

fn main() {
    let cli = Cli::parse();

    if cli.pattern.len() == 0 {
        println!("No dice rolled. Total: 0");
        std::process::exit(1);
    }

    let roll_regex = Regex::new(r"^([1-9][0-9]*)*[dD][1-9][0-9]*((\+|-)[0-9]+)?$").unwrap();

    for i in &cli.pattern {
        if !roll_regex.is_match(i) {
            println!("Invalid syntax for roll: {}. Syntax for rolls is xdy+z, where x, y and z are integers greater than zero.", i);
            std::process::exit(1);
        }
    }

    let rolls = parse_rolls(cli.pattern);
    let mut grand_total = 0;

    for roll in rolls {
        let res = simulate_roll(&roll);
        let roll_total = res.iter().sum::<i32>() + roll.delta;
        print_roll(&res, roll.delta, roll_total);

        grand_total += roll_total;
    }

    println!("Total: {}", grand_total);
}

fn print_roll(roll_vector: &Vec<i32>, delta: i32, total: i32) {
    print!("{:?} ", roll_vector);
    if delta > 0 {
        print!("+ {} ", delta);
    } else if delta < 0 {
        print!("- {} ", -delta);
    }
    println!(": {}", total);
}

fn simulate_roll(roll: &Roll) -> Vec<i32> {
    let mut res = vec![];
    let mut rng = rand::thread_rng();

    for _ in 0..roll.times {
        res.push(rng.gen_range(1..=roll.sides));
    }

    res
}

fn parse_rolls(rolls: Vec<String>) -> Vec<Roll> {
    let mut res = vec![];

    for roll in rolls {
        let parts: Vec<&str> = roll.split(['d', 'D', '+', '-']).collect();

        let times = match parts.get(0).unwrap().parse::<i32>() {
            Ok(t) => t,
            Err(_) => {
                println!("Number {} is too large.", parts.first().unwrap());
                std::process::exit(1);
            }
        };

        let sides = match parts.get(1).unwrap().parse::<i32>() {
            Ok(t) => t,
            Err(_) => {
                println!("Number {} is too large.", parts.get(1).unwrap());
                std::process::exit(1);
            }
        };

        let mut delta = if parts.len() == 2 {
            0
        } else {
            match parts.get(2).unwrap().parse() {
                Ok(t) => t,
                Err(_) => {
                    println!("Number {} is too large.", parts.first().unwrap());
                    std::process::exit(1);
                }
            }
        };

        if roll.contains('-') {
            delta *= -1;
        }

        res.push(Roll {
            times,
            sides,
            delta,
        });
    }

    res
}
