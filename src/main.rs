use clap::Parser;
use regex::Regex;
use rand::prelude::*;

#[derive(Parser)]
struct Cli {
    #[clap(num_args = 1..)]
    pattern: Vec<String>,
}

#[derive(Debug)]
struct Roll {
    fst: i32,
    snd: i32,
    delta: i32,
}

fn main() {
    let cli = Cli::parse();
    let roll_regex = Regex::new(r"^([1-9][0-9]*)*[dD][1-9][0-9]*(\+[0-9]+)?$").unwrap();

    for i in &cli.pattern {
        if !roll_regex.is_match(i) {
            println!("Invalid syntax for roll: {}. Syntax for rolls is NdN+N, where N is an integer greater than zero.", i);
            std::process::exit(-1);
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
        if delta != 0 {
            print!("+ {} ", delta);
        }
        println!(": {}", total);
}

fn simulate_roll(roll: &Roll) -> Vec<i32> {
    let mut res = vec![];
    let mut rng = rand::thread_rng();
    let mut y: i32;

    for _ in 0..roll.fst {
        y = rng.gen_range(1..=roll.snd);
        res.push(y);
    }

    res
}

fn parse_rolls(rolls: Vec<String>) -> Vec<Roll> {
    let mut res = vec![];
    
    for roll in rolls {
        let parts: Vec<&str> = roll.split(['d', 'D', '+']).collect();
        let fst: i32 = parts.get(0).unwrap().parse().unwrap();
        let snd: i32 = parts.get(1).unwrap().parse().unwrap();
        let delta: i32;
    
        if parts.len() == 2 {
            delta = 0;
        } else {
            delta = parts.get(2).unwrap().parse().unwrap();
        }
    
        res.push(Roll { fst, snd, delta });
    }

    res
}
