use clap::{Arg, App, SubCommand};
use reqwest::header::COOKIE;
use anyhow::{Context, Result, anyhow};
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
        .get_matches();
        
    match matches.subcommand() {
        ("start", Some(sub_m)) => start(),
        ("submit", Some(sub_m)) => submit(
            sub_m.value_of("PART").unwrap() == "2",
            sub_m.value_of("RESULT")
        ),
        _ => {
            Err(anyhow!("Invalid subcommand..."))
        }, 
    }
}


fn start() -> Result<()> {
    let (year, day) = get_challenge_desc()?;
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