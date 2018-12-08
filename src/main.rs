use std::fs::File;
use std::io::prelude::*;
use structopt::StructOpt;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;

fn get_func(day_number: i32) -> Result<CalcFunction, String> {
    match day_number {
        1 => Ok(day1::calc),
        2 => Ok(day2::calc),
        3 => Ok(day3::calc),
        4 => Ok(day4::calc),
        5 => Ok(day5::calc),
        6 => Ok(day6::calc),
        7 => Ok(day7::calc),
        8 => Ok(day8::calc),
        n @ 1..=24 => Err(format!("Day {} is not implemented yet.", n)),
        _ => Err("Day has to between 1 and 24.".to_owned()),
    }
}

pub fn read_input(day_number: i32) -> std::io::Result<String> {
    let mut file = File::open(format!("inputs/input{}.txt", day_number))?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents.trim().to_owned())
}

#[derive(StructOpt)]
struct Cli {
    day: i32,
}

type CalcFunction = for<'r> fn(&'r str) -> (String, String);

fn execute_func(day_number: i32) {
    match get_func(day_number) {
        Ok(func) => {
            let (part1, part2) = func(&read_input(day_number).unwrap());
            println!(
                "Results for day {}:\n  Part 1: {}\n  Part 2: {}",
                day_number,
                part1,
                part2,
            )
        },
        Err(s) => println!("{}", s),
    }
}

fn main() {
    let args = Cli::from_args();
    
    if args.day == 25 {
        for i in 1..=24 {
            execute_func(i);
        }
    } else {
        execute_func(args.day);
    }
}
