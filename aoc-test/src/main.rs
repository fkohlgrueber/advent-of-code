use aoc_tools::AocResults;

fn main() {
    
    let outputs = AocResults::from_file("../outputs.ron");
    
    let mut years = aoc_solutions::YEARS.iter().collect::<Vec<_>>();
    years.sort_by_key(|x| x.0);
    for (year, days) in years {
        let mut days = days.iter().collect::<Vec<_>>();
        days.sort_by_key(|x| x.0);
        for (day_idx, day) in days {
            
            let s = std::fs::read_to_string(format!("../inputs/year{}/input{:02}.txt", year, day_idx)).unwrap();
            let res = day.run(s);
            let (part_1, part_2) = outputs.get(*year, *day_idx).unwrap();
            let p1 = if part_1 == &res.0 { "OK" } else { "ERROR" };
            let p2 = if part_2 == &res.1 { "OK" } else { "ERROR" };
            println!("Year {}, day {}:\n  Part 1: {}  {}\n  Part 2: {}  {}", year, day_idx, res.0, p1, res.1, p2);
        }
    }
}
