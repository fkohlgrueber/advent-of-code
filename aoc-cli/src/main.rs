use clap::{Arg, App, SubCommand};
use reqwest::header::COOKIE;
use anyhow::{Result, anyhow};
use colored::*;

fn main() -> Result<()> {
    let matches = App::new("Advent of Code CLI")
        .version("0.1")
        .author("Felix Kohlgrüber <felix.kohlgrueber@gmail.com")
        .subcommand(SubCommand::with_name("start")
            .about("downloads the puzzle input from adventofcode.com"))
        .subcommand(SubCommand::with_name("submit")
            .about("submits puzzle result to adventofcode.com")
            .arg(Arg::with_name("PART")
                .required(true)
                .possible_values(&["1", "2"]))
            .arg(Arg::with_name("RESULT"))
        )
        .subcommand(SubCommand::with_name("finish")
            .about("puts local solution into appropriate places"))
        .get_matches();
        
    match matches.subcommand() {
        ("start", Some(_sub_m)) => start(),
        ("submit", Some(sub_m)) => submit(
            sub_m.value_of("PART").unwrap() == "2",
            sub_m.value_of("RESULT")
        ),
        ("finish", Some(_sub_m)) => finish(),
        _ => {
            Err(anyhow!("Invalid subcommand..."))
        }, 
    }
}


fn start() -> Result<()> {
    let (year, day) = get_challenge_desc().unwrap_or_else(|_| {
        let today = chrono::offset::Local::today();
        let year = today.format("%Y").to_string().parse::<usize>().unwrap();
        let day = today.format("%d").to_string().parse::<usize>().unwrap();
        std::fs::write("challenge.txt", &format!("{},{}", year, day)).unwrap();
        (year, day)
    });
    println!("Requesting input for year {}, day {}...", year, day);
    let input = request_input(year, day)?;
    let lines_min_max = input.lines().fold((usize::max_value(), 0), |s, e| {
        (std::cmp::min(s.0, e.chars().count()), std::cmp::max(s.1, e.chars().count()))
    });
    std::fs::write("input.txt", &input)?;
    println!("{}", "Success!".green().bold());
    println!(
        "\nNum chars: {}\nNum lines: {} (min: {}, max: {})\n", 
        input.chars().count(), 
        input.lines().count(),
        lines_min_max.0,
        lines_min_max.1,
    );
    println!("Head of input: \n{}",
        input.lines().take(10).map(|s| s.chars().take(100).collect::<String>()).collect::<Vec<_>>().join("\n")
    );
    Ok(())
}

fn submit(part_2: bool, result: Option<&str>) -> Result<()> {
    let result = match result {
        Some(s) => s.to_string(),
        None => {
            // read from file...
            std::fs::read_to_string(&format!("result{}.txt", if part_2 { "2" } else { "1" }))?
        }
    };
    let (year, day) = get_challenge_desc()?;
    println!("Submitting {:?} as solution for part {}...", result, if part_2 { "2" } else { "1" });
    println!("{}", post_solution(year, day, part_2, result)?);
    Ok(())
}

fn get_challenge_desc() -> Result<(usize, usize)> {
    let s = std::fs::read_to_string("challenge.txt")?;
    let mut split = s.split(",");
    let year = split.next().unwrap().trim().parse().unwrap();
    let day = split.next().unwrap().trim().parse().unwrap();
    Ok((year, day))
}

fn get_auth_token() -> Result<String> {
    Ok(std::fs::read_to_string("/home/felix/.config/aoc/token")?)
}

fn request_input(year: usize, day: usize) -> Result<String> {
    let token = get_auth_token()?;
    let client = reqwest::Client::new();
    let mut res = client.get(&format!("https://adventofcode.com/{}/day/{}/input", year, day))
        .header(COOKIE, format!("session={}", token.trim()))
        .send()?.error_for_status()?;
    let text = res.text()?;
    Ok(text)
}

fn post_solution(year: usize, day: usize, part_2: bool, answer: String) -> Result<String> {
    let token = get_auth_token()?;
    let client = reqwest::Client::new();
    let mut res = client.post(&format!("https://adventofcode.com/{}/day/{}/answer", year, day))
        .header(COOKIE, format!("session={}", token.trim()))
        .form(&[("level", if part_2 { "2" } else { "1" }), ("answer", &answer)])
        .send()?.error_for_status()?;
    let text = res.text()?;

    let doc = select::document::Document::from_read(text.as_bytes())?;
    use select::predicate::*;
    let pre = Child(Name("article"), Name("p"));
    for elmt in doc.find(pre).into_selection().iter() {
        let s = elmt.text();
        let res = SubmitResult::from_str(&s);
        return Ok(format!("{}", res));
    }
    Ok(text)
}

enum SubmitResult {
    Success,
    WrongAnswer(WrongAnswer),
    TooRecently(usize, usize), // mins, seconds to wait
    WrongLevel,
    Unknown(String)
}

impl SubmitResult {
    fn from_str(s: &str) -> Self {
        if s.contains("Did you already complete it?") {
            SubmitResult::WrongLevel
        } else if s.contains("That's not the right answer") {
            let wa = if s.contains("your answer is too low") {
                WrongAnswer::TooLow
            } else if s.contains("your answer is too high") {
                WrongAnswer::TooHigh
            } else {
                WrongAnswer::Else
            };
            SubmitResult::WrongAnswer(wa)
        } else if s.contains("You gave an answer too recently") {
            let re = regex::Regex::new(r"You have(?: (?P<mins>\d+)m)? (?P<secs>\d+)s left to wait").unwrap();
            let cap = re.captures(s).unwrap();
            let mins = cap.name("mins").map(|x| x.as_str().parse::<usize>().unwrap()).unwrap_or(0);
            let secs = cap.name("secs").unwrap().as_str().parse::<usize>().unwrap();
            SubmitResult::TooRecently(mins, secs)
        } else if s.contains("That's the right answer!") {
            SubmitResult::Success
        } else {
            SubmitResult::Unknown(s.to_string())
        }
    }
}

impl std::fmt::Display for SubmitResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SubmitResult::Success => write!(f, "{}", "Success!".green().bold()),
            SubmitResult::WrongAnswer(wa) => {
                match wa {
                    WrongAnswer::TooLow => write!(f, "{}", "Answer too low!".red().bold()),
                    WrongAnswer::TooHigh => write!(f, "{}", "Answer too high!".red().bold()),
                    WrongAnswer::Else => write!(f, "{}", "Wrong answer!".red().bold()),
                }
            },
            SubmitResult::TooRecently(mins, secs) => write!(f, "{}", 
                format!("Need to wait{} {}s...", if *mins == 0 {"".to_string()} else {format!(" {}m", mins)}, secs).yellow().bold()
            ),
            SubmitResult::WrongLevel => write!(f, "{}", "Wrong puzzle?".yellow().bold()),
            SubmitResult::Unknown(s) => write!(f, "Unknown output:\n{}", s)
        }
    }
}

enum WrongAnswer {
    TooLow,
    TooHigh,
    Else
}

fn finish() -> Result<()> {
    let (year, day) = get_challenge_desc()?;

    // get results
    let res_part_1 = std::fs::read_to_string("result1.txt")?;
    let res_part_2 = std::fs::read_to_string("result2.txt")?;

    // append results to result db
    let mut res_db = aoc_tools::AocResults::from_file("../outputs.ron");
    res_db.insert(year, day, (res_part_1, res_part_2));
    res_db.write_to_file("../outputs.ron");

    // copy source code
    let solution_txt = std::fs::read_to_string("src/aoc_workbench.rs")?;
    std::fs::write(&format!("../aoc-solutions/year{}/day{:02}.rs", year, day), &solution_txt)?;

    // copy input file
    let input_txt = std::fs::read_to_string("input.txt")?;
    std::fs::write(&format!("../inputs/year{}/input{:02}.txt", year, day), &input_txt)?;

    // delete input, result and challenge files
    std::fs::remove_file("input.txt")?;
    std::fs::remove_file("result1.txt")?;
    std::fs::remove_file("result2.txt")?;
    std::fs::remove_file("challenge.txt")?;

    // reset aoc_workbench to initial state
    std::process::Command::new("git").args(&["checkout", "--", "src/aoc_workbench.rs"]).output()?;

    Ok(())
}
