
mod aoc_workbench;
//mod test;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let s = std::fs::read_to_string("input.txt")?;
    let data = aoc_workbench::Day::parse(s);
    /*
    println!("Part 1 tests");
    for (i, o) in aoc_workbench::Day::tests_part_1() {
        let data = aoc_workbench::Day::parse(i.to_string());
        assert_eq!(aoc_workbench::Day::part_1(data), o);
    }*/
    println!("Part 1:");
    let res_1 = aoc_workbench::Day::part_1(data.clone());
    println!("{}", res_1);
    if !res_1.is_empty() {
        std::fs::write("result1.txt", res_1)?;
    }
    println!("Part 2:");
    let res_2 = aoc_workbench::Day::part_2(data);
    println!("{}", res_2);
    if !res_2.is_empty() {
        std::fs::write("result2.txt", res_2)?;
    }
    Ok(())
}


/*fn main() -> Result<(), Box<dyn std::error::Error>> {
    for elmt in test::description(2018, 3)? {
        if let test::Elmt::Block(s) = elmt {
            /*if is_valid_input(&s) {
                println!("Valid input:\n{}\n\n", s)
            }*/
        }
    }
    
    //test::get_input(2018, 3)?;
    Ok(())
}
*/