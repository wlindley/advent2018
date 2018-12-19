use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() {
    let contents = read_input();
    let node = parse(&contents);
    let total = sum_node(&node);
    println!("Metadata sum: {}", total);
}

fn read_input() -> String {
    let f = File::open("input.txt").expect("could not find file");
    let mut r = BufReader::new(&f);
    let mut contents = String::new();
    r.read_to_string(&mut contents);
    return contents;
}

fn numeric_stream(input: &String) -> Vec<u32> {
    return input.split_whitespace().map(|t| t.parse().unwrap()).collect();
}

fn parse(input: &String) -> Node {
    let numbers = numeric_stream(input);
    let mut iter = numbers.iter();
    return Node::read(&mut iter);
}

fn sum_node(node: &Node) -> u32 {
    return node.metadata.iter().fold(0, |total, c| total + c)
        + node.children.iter().map(sum_node).fold(0, |total, c| total + c);
}

#[derive(Debug, PartialEq, Eq)]
struct Node {
    children: Vec<Node>,
    metadata: Vec<u32>,
}

impl Node {
    fn new(children: Vec<Node>, metadata: Vec<u32>) -> Node {
        return Node{children, metadata};
    }

    fn read(mut stream: &mut std::slice::Iter<u32>) -> Node {
        let (num_children, num_metadata) = Node::read_header(&mut stream);
        return Node{
            children: Node::read_children(&mut stream, num_children),
            metadata: Node::read_metadata(&mut stream, num_metadata),
        };
    }

    fn read_header(stream: &mut std::slice::Iter<u32>) -> (usize, usize) {
        return (*stream.next().unwrap() as usize, *stream.next().unwrap() as usize);
    }

    fn read_children(mut stream: &mut std::slice::Iter<u32>, count: usize) -> Vec<Node> {
        return (0..count).map(|_| Node::read(&mut stream)).collect();
    }

    fn read_metadata(stream: &mut std::slice::Iter<u32>, count: usize) -> Vec<u32> {
        return stream.take(count).cloned().collect();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_numeric_stream() {
        assert_eq!(vec![0, 2, 5, 5], numeric_stream(&String::from("0 2 5 5")));
    }

    #[test]
    fn test_parse_simple() {
        let node = parse(&String::from("0 2 5 5"));
        let expected = Node::new(Vec::new(), vec![5, 5]);
        assert_eq!(expected, node);
    }

    #[test]
    fn test_parse_single_child() {
        let node = parse(&String::from("1 2 0 1 3 5 5"));
        let expected = Node::new(vec![Node::new(Vec::new(), vec![3])], vec![5, 5]);
        assert_eq!(expected, node);
    }

    #[test]
    fn test_sum_node() {
        let node = Node::new(Vec::new(), vec![1, 2, 3]);
        assert_eq!(6, sum_node(&node));

        let node = Node::new(vec![
            Node::new(Vec::new(), vec![4, 5, 6]),
            Node::new(Vec::new(), vec![7, 8, 9]),
        ], vec![1, 2, 3]);
        assert_eq!(45, sum_node(&node));
    }
}
