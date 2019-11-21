use std::fs::File;
use std::io::prelude::*;
use std::time::Instant;
use structopt::StructOpt;

mod day7;
mod day8;
mod day9;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day20;
mod day21;
mod day22;

fn get_func(day_number: i32) -> Result<CalcFunction, String> {
    match day_number {
        7 => Ok(day7::calc),
        8 => Ok(day8::calc),
        9 => Ok(day9::calc),
        10 => Ok(day10::calc),
        11 => Ok(day11::calc),
        12 => Ok(day12::calc),
        13 => Ok(day13::calc),
        14 => Ok(day14::calc),
        15 => Ok(day15::calc),
        16 => Ok(day16::calc),
        17 => Ok(day17::calc),
        18 => Ok(day18::calc),
        19 => Ok(day19::calc),
        20 => Ok(day20::calc),
        21 => Ok(day21::calc),
        22 => Ok(day22::calc),
        n @ 23..=24 => Err(format!("Day {} is not implemented yet.", n)),
        _ => Err("Day has to between 1 and 24.".to_owned()),
    }
}

pub fn read_input(day_number: i32) -> std::io::Result<String> {
    let mut file = File::open(format!("inputs/year2018/input{:02}.txt", day_number))?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents.trim().to_owned())
}

#[derive(StructOpt)]
struct Cli {
    day: i32,
    bench: bool,
}

type CalcFunction = for<'r> fn(&'r str) -> (String, String);

fn execute_func(day_number: i32) {
    match get_func(day_number) {
        Ok(func) => {
            let input = read_input(day_number).unwrap();
            let now = Instant::now();
            let (part1, part2) = func(&input);
            let elapsed = now.elapsed();
            println!(
                "Results for day {}:\n  Part 1: {}\n  Part 2: {}\n  (calculated in {} seconds)",
                day_number,
                part1,
                part2,
                elapsed.as_secs() as f32 + elapsed.subsec_millis() as f32 / 1000.,
            )
        },
        Err(s) => println!("{}", s),
    }
}

fn bench_func(day_number: i32){
    if let Ok(func) = get_func(day_number) {
        let num_iterations = 10;
        let input = read_input(day_number).unwrap();
        let mut res : Vec<(String, String)> = Vec::new();
        res.reserve(num_iterations);
        let now = Instant::now();
        for _ in 0..num_iterations {
            res.push(func(&input));
        }
        let elapsed = now.elapsed();
        for i in 1..res.len() {
            if res[i] != res[0] {panic!("error!")};
        }
        println!(
            "Day {:2}: {:.3} s per iteration",
            day_number,
            (elapsed.as_secs() as f32 + elapsed.subsec_millis() as f32 / 1000.) / num_iterations as f32,
        )
    }
}

fn main() {
    let args = Cli::from_args();
    
    let func = if args.bench { bench_func} else {execute_func};

    if args.day == 25 {
        for i in 1..=24 {
            func(i);
        }
    } else {
        func(args.day);
    }
}
