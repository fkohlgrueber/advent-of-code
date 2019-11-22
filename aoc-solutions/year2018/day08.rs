#[allow(unused_imports)]
use aoc_tools::prelude::*;

pub struct Day();

impl Challenge for Day {
    type Input = Node;

    fn parse(input: String) -> Self::Input {
        parse_node(&mut input.split(' ').map(|x| x.parse().unwrap()))
    }

    fn part_1(input: Self::Input) -> String {
        input.sum_metadata().to_string()
    }
    
    fn part_2(input: Self::Input) -> String {
        input.calc_value().to_string()
    }
}


#[derive(Debug, Clone)]
pub struct Node {
    children: Vec<Node>,
    metadata: Vec<i32>,
}

impl Node {
    fn sum_metadata(&self) -> i32 {
        self.metadata.iter().sum::<i32>() + self.children.iter().map(|x| x.sum_metadata()).sum::<i32>()
    }

    fn calc_value(&self) -> i32 {
        if self.children.is_empty(){
            return self.sum_metadata();
        }
        
        let mut ret = 0;
        for m in &self.metadata{
            if  1 <= *m && *m <= self.children.len() as i32 {
                ret += self.children[(*m-1) as usize].calc_value();
            } 
        }
        
        ret
    }
}

fn parse_node<I>(num_iter: &mut I) -> Node
where 
    I : Iterator<Item = i32>,
{
    let num_children = num_iter.next().unwrap();
    let num_metadata = num_iter.next().unwrap();

    let mut children = Vec::new();
    children.reserve(num_children as usize);
    for _ in 0..num_children{
        children.push(parse_node(num_iter));
    }

    let mut metadata = Vec::new();
    children.reserve(num_metadata as usize);
    for _ in 0..num_metadata {
        metadata.push(num_iter.next().unwrap());
    }

    Node{
        children,
        metadata
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        Day::test_part_1("2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2", 138);
    }

    #[test]
    fn test_part_2() {
        Day::test_part_2("2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2", 66);
    }
}
