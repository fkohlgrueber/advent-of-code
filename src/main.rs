use std::fs::File;
use std::io::prelude::*;
use structopt::StructOpt;

mod day1;

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

fn main() {
    let args = Cli::from_args();
    match &args.day {
        1 => println!("{:?}", day1::calc(&read_input(1).unwrap())),
        1..=24 => println!("This day is not implemented yet."),
        _ => println!("Day has to between 1 and 24."),
    }
}
