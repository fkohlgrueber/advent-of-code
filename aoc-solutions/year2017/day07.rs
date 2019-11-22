#[allow(unused_imports)]
use aoc_tools::prelude::*;

pub struct Day();

impl Challenge for Day {
    type Input = Vec<ParseInput>;

    fn parse(input: String) -> Self::Input {
        ParseInput::from_str_multiple(&input)
    }

    fn part_1(input: Self::Input) -> String {
        calc_root(&input)
    }
    
    fn part_2(input: Self::Input) -> String {
        let root = calc_root(&input);
        let hm: HashMap<_, _> = input.into_iter().map(|x| (x.name.clone(), x)).collect();
        rek(&hm, &root).unwrap_err().to_string()
    }
}

fn calc_root(input: &Vec<ParseInput>) -> String {
    let mut nodes_from_deps = HashSet::new();
    input.iter().for_each(|e| {
        e.deps.iter().for_each(|x| {nodes_from_deps.insert(x.0.clone());})
    });
    for n in input {
        if !nodes_from_deps.contains(&n.name) {
            return n.name.to_string()
        }
    }
    panic!("no root!")
}

/// returns the total weight of node `current` and Err(correct_weight) if it contains the incorrect weight
fn rek(input: &HashMap<String, ParseInput>, current: &str) -> Result<usize, usize> {
    let own_weight = input[current].weight;
    let dep_weights = input[current].deps.iter().map(|e| rek(&input, &e.0)).collect::<Result<Vec<usize>, usize>>()?;
    let mut counter: HashMap<_, Vec<_>> = HashMap::new();
    input[current].deps.iter().zip(dep_weights.iter()).for_each(|(s, weight)| {
        counter.entry(weight).or_default().push(&s.0);
    });
    if counter.len() < 2 {
        return Ok(own_weight + dep_weights.iter().sum::<usize>());
    }
    let mut counter_list = counter.into_iter().collect::<Vec<_>>();
    counter_list.sort_by_key(|(_w, elmts)| elmts.len());
    let wrong_elmt = counter_list[0].1[0];
    let wrong_elmt_total_weight = counter_list[0].0;
    let correct_total_weight = counter_list[1].0;
    Err(input[wrong_elmt].weight + correct_total_weight - wrong_elmt_total_weight)
}

#[parse(r"{} \({}\){}")]
#[derive(Debug, Clone)]
pub struct ParseInput {
    #[parse = r"[a-z]+"]
    name: String,
    weight: usize,
    #[parse = r".*\n"]
    deps: Vec<Dep>
}

#[parse(r"{}")]
#[derive(Debug, Clone)]
pub struct Dep(
    #[parse = r"[a-z]+"]
    String
);





#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        Day::test_part_1("\
            pbga (66)\n\
            xhth (57)\n\
            ebii (61)\n\
            havc (66)\n\
            ktlj (57)\n\
            fwft (72) -> ktlj, cntj, xhth\n\
            qoyq (66)\n\
            padx (45) -> pbga, havc, qoyq\n\
            tknk (41) -> ugml, padx, fwft\n\
            jptl (61)\n\
            ugml (68) -> gyxo, ebii, jptl\n\
            gyxo (61)\n\
            cntj (57)\n", 
            "tknk");
    }

    #[test]
    fn test_part_2() {
        Day::test_part_2("\
            pbga (66)\n\
            xhth (57)\n\
            ebii (61)\n\
            havc (66)\n\
            ktlj (57)\n\
            fwft (72) -> ktlj, cntj, xhth\n\
            qoyq (66)\n\
            padx (45) -> pbga, havc, qoyq\n\
            tknk (41) -> ugml, padx, fwft\n\
            jptl (61)\n\
            ugml (68) -> gyxo, ebii, jptl\n\
            gyxo (61)\n\
            cntj (57)\n", 
            60);
    }
}
