
#[allow(unused_imports)]
use aoc_tools::prelude::*;

use regex::Regex;

pub fn calc(input: &str) -> (String, String) {
    (part_1(input), part_2(input, 5, 60).to_string())
}

fn part_1(input: &str) -> String {
    let mut nodes = gen_nodes(input);

    // execute tasks
    let mut res = Vec::new();
    while nodes.values().any(|n| n.done == Status::Pending) {

        let mut ready_nodes = Vec::new();
        for n in nodes.values(){
            if n.done == Status::Pending {
                let mut all_deps_done = true;
                for c in &n.dependencies{
                    if nodes[c].done == Status::Pending{
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
        nodes.get_mut(&ready_nodes[0]).unwrap().done = Status::Done;
    }

    res.iter().collect()
}

fn part_2(input: &str, num_workers: usize, min_duration: usize) -> i32 {
    let mut nodes = gen_nodes(input);

    let mut workers : Vec<Option<(usize, char)>> = vec![None; num_workers];
    let mut time = 0;
    while nodes.values().any(|n| n.done != Status::Done) {

        let mut ready_nodes = Vec::new();
        for n in nodes.values(){
            if n.done == Status::Pending {
                let mut all_deps_done = true;
                for c in &n.dependencies{
                    if nodes[c].done != Status::Done{
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
        for w in &mut workers{
            if !ready_nodes.is_empty() && w.is_none(){
                // idle worker and ready task -> worker starts working on task

                let node_id = ready_nodes.remove(0);

                // set worker to busy
                *w = Some((time, node_id));

                // set node to "in progress"
                nodes.get_mut(&node_id).unwrap().done = Status::InProgress;
            }
        }

        time += 1;

        for w in &mut workers{
            if let Some((t_start, node_id)) = w {
                let duration = min_duration as u8 + 1 + *node_id as u8 - b'A';
                if time == *t_start + duration as usize{
                    nodes.get_mut(&node_id).unwrap().done = Status::Done;
                    *w = None;
                }
            }
        }
    }

    time as i32
}

#[derive(Debug, PartialEq)]
enum Status {
    Pending,
    Done,
    InProgress,
}

#[derive(Debug)]
struct Node {
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

fn gen_nodes(input: &str) -> HashMap<char, Node> {
    // init hash table for nodes
    let mut nodes: HashMap<char, Node> = HashMap::new();

    // parse input
    let re = Regex::new(r"Step (.) must be finished before step (.) can begin.").unwrap();
    let mut dependencies = Vec::new();
    for cap in re.captures_iter(input) {
        let char1 = cap[1].chars().next().unwrap();
        let char2 = cap[2].chars().next().unwrap();
        dependencies.push((char1, char2));
    }

    // create nodes and populate dependencies
    for (char1, char2) in dependencies {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(
            part_1("Step C must be finished before step A can begin.\n\
                    Step C must be finished before step F can begin.\n\
                    Step A must be finished before step B can begin.\n\
                    Step A must be finished before step D can begin.\n\
                    Step B must be finished before step E can begin.\n\
                    Step D must be finished before step E can begin.\n\
                    Step F must be finished before step E can begin."), 
            "CABDFE"
        );
    }

    #[test]
    fn test_part_2() {
        assert_eq!(
            part_2("Step C must be finished before step A can begin.\n\
                    Step C must be finished before step F can begin.\n\
                    Step A must be finished before step B can begin.\n\
                    Step A must be finished before step D can begin.\n\
                    Step B must be finished before step E can begin.\n\
                    Step D must be finished before step E can begin.\n\
                    Step F must be finished before step E can begin.", 2, 0), 
            15
        );
    }
}
