#[allow(unused_imports)]
use aoc_tools::prelude::*;

pub struct Day();

impl Challenge for Day {
    type Input = HashMap<char, Node>;

    fn parse(input: String) -> Self::Input {
        // init hash table for nodes
        let mut nodes: HashMap<char, Node> = HashMap::new();

        // parse input
        let dependencies = InputLine::from_str_multiple(&input);
        
        // create nodes and populate dependencies
        for InputLine(char1, char2) in dependencies {
            // char2 depends on char1

            // make sure that char1 exists
            nodes.entry(char1).or_insert_with(|| Node::new(char1));

            // make sure that char2 exists and
            // add char1 as dependency for char2
            nodes
                .entry(char2)
                .or_insert_with(|| Node::new(char2))
                .dependencies.push(char1);
        }

        nodes
    }

    fn part_1(mut input: Self::Input) -> String {
        // execute tasks
        let mut res = Vec::new();
        while input.values().any(|n| n.done == Status::Pending) {

            let mut ready_nodes = Vec::new();
            for n in input.values(){
                if n.done == Status::Pending {
                    let mut all_deps_done = true;
                    for c in &n.dependencies{
                        if input[c].done == Status::Pending{
                            all_deps_done = false;
                        }
                    }
                    if all_deps_done {
                        ready_nodes.push(n.id);
                    }
                }
            }
            ready_nodes.sort();
            res.push(ready_nodes[0]);
            input.get_mut(&ready_nodes[0]).unwrap().done = Status::Done;
        }

        res.iter().collect()
    }
    
    fn part_2(input: Self::Input) -> String {
        Self::_part_2(input, 5, 60)
    }
}

impl Day {
    fn _part_2(mut input: <Self as Challenge>::Input, num_workers: usize, min_duration: usize) -> String {
        let mut workers : Vec<Option<(usize, char)>> = vec![None; num_workers];
        let mut time = 0;
        while input.values().any(|n| n.done != Status::Done) {

            let mut ready_nodes = Vec::new();
            for n in input.values(){
                if n.done == Status::Pending {
                    let mut all_deps_done = true;
                    for c in &n.dependencies{
                        if input[c].done != Status::Done{
                            all_deps_done = false;
                        }
                    }
                    if all_deps_done {
                        ready_nodes.push(n.id);
                    }
                }
            }
            ready_nodes.sort();
            //println!("{:?}", ready_nodes);
            for w in &mut workers {
                if !ready_nodes.is_empty() && w.is_none(){
                    // idle worker and ready task -> worker starts working on task

                    let node_id = ready_nodes.remove(0);

                    // set worker to busy
                    *w = Some((time, node_id));

                    // set node to "in progress"
                    input.get_mut(&node_id).unwrap().done = Status::InProgress;
                }
            }

            time += 1;

            for w in &mut workers{
                if let Some((t_start, node_id)) = w {
                    let duration = min_duration as u8 + 1 + *node_id as u8 - b'A';
                    if time == *t_start + duration as usize{
                        input.get_mut(&node_id).unwrap().done = Status::Done;
                        *w = None;
                    }
                }
            }
        }

        time.to_string()
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Status {
    Pending,
    Done,
    InProgress,
}

#[derive(Debug, Clone)]
pub struct Node {
    id: char,
    dependencies: Vec<char>,
    done: Status,
}

impl Node {
    fn new(id: char) -> Node {
        Node {
            id,
            dependencies: vec![],
            done: Status::Pending,
        }
    }
}

#[parse("Step {} must be finished before step {} can begin.")]
struct InputLine(char, char);


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        Day::test_part_1("Step C must be finished before step A can begin.\n\
                          Step C must be finished before step F can begin.\n\
                          Step A must be finished before step B can begin.\n\
                          Step A must be finished before step D can begin.\n\
                          Step B must be finished before step E can begin.\n\
                          Step D must be finished before step E can begin.\n\
                          Step F must be finished before step E can begin.", 
                          "CABDFE"
        );
    }

    #[test]
    fn test_part_2() {
        assert_eq!(Day::_part_2(Day::parse(
            "Step C must be finished before step A can begin.\n\
             Step C must be finished before step F can begin.\n\
             Step A must be finished before step B can begin.\n\
             Step A must be finished before step D can begin.\n\
             Step B must be finished before step E can begin.\n\
             Step D must be finished before step E can begin.\n\
             Step F must be finished before step E can begin.".to_string()), 2, 0), 
            "15".to_string());
    }
}
