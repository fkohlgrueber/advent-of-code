use serde_json::Value;

fn main() {
    
    let outputs: Value = serde_json::from_str(&std::fs::read_to_string("../outputs.json").unwrap()).unwrap();

    
    let mut years = aoc_solutions::YEARS.iter().collect::<Vec<_>>();
    years.sort_by_key(|x| x.0);
    for (year, days) in years {
        let mut days = days.iter().collect::<Vec<_>>();
        days.sort_by_key(|x| x.0);
        for (day_idx, day) in days {
            
            let s = std::fs::read_to_string(format!("../inputs/year{}/input{:02}.txt", year, day_idx)).unwrap();
            let res = day.run(s);
            let (part_1, part_2) = get_exp_output(&outputs, *year, *day_idx);
            let p1 = if part_1 == res.0 { "OK" } else { "ERROR" };
            let p2 = if part_2 == res.1 { "OK" } else { "ERROR" };
            println!("Year {}, day {}:\n  Part 1: {}  {}\n  Part 2: {}  {}", year, day_idx, res.0, p1, res.1, p2);
        }
    }
}

fn get_exp_output(val: &Value, year: usize, day: usize) -> (String, String) {
    if let Value::Object(map) = val {
        if let Value::Object(map2) = map.get(&year.to_string()).unwrap() {
            if let Value::Array(vals) = map2.get(&day.to_string()).unwrap() {
                (to_str(&vals[0]), to_str(&vals[1]))
            } else {
                panic!()
            }
        } else {
            panic!()
        }
    } else {
        panic!()
    }
}

fn to_str(val: &Value) -> String {
    match val {
        Value::Number(n) => n.to_string(),
        Value::String(s) => s.to_string(),
        _ => panic!()
    }
}