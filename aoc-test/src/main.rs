
fn main() {
    println!("{:?}", aoc_solutions::year2017::DAYS.keys());
    println!("{:?}", aoc_solutions::YEARS.keys());

    
    let mut years = aoc_solutions::YEARS.iter().collect::<Vec<_>>();
    years.sort_by_key(|x| x.0);
    for (year, days) in years {
        let mut days = days.iter().collect::<Vec<_>>();
        days.sort_by_key(|x| x.0);
        for (day_idx, day) in days {
            
            let s = std::fs::read_to_string(format!("../inputs/year{}/input{:02}.txt", year, day_idx)).unwrap();
            let res = day.run(s);
            println!("Year {}, day {}:\n  Part 1: {}\n  Part 2: {}", year, day_idx, res.0, res.1);
        }
    }
}