fn main() {
    let input = include_str!("../input_tst.txt");

    println!("First solution: {}", first_solution(&input));
    println!("Second solution: {}", second_solution(&input));
}

fn second_solution(input: &str) -> usize {
    0
}

fn first_solution(input: &str) -> usize {
    let mut roots = input.lines()
        .map(|line| Node::parse(line).unwrap())
        .collect::<Vec<_>>();

    let node1 = &mut roots[0];
    node1.reduce();

    0
}

#[derive(Debug)]
enum Node {
    Leaf(u8),
    Inner(Box<Node>, Box<Node>),
}

impl Node {
    fn reduce(&mut self) {
        while self.try_explode() {}
        while self.try_split() {}
    }

    fn try_explode(&mut self) -> bool {
        println!("trying to explode: {:?}", self);
        let mut node = self;
        while let Node::Inner(l, r) = node {
            println!("left: {:?}, right: {:?}", l, r);
            node = l;
        }

        false
    }

    fn try_split(&mut self) -> bool {
        println!("trying to split: {:?}", self);
        false
    }

    fn parse(line: &str) -> Result<Self, String> {
        peg::parser! {
            grammar parser() for str {
                rule leaf() -> Node
                = n:$(['0'..='9']+) { Node::Leaf(n.parse::<u8>().unwrap()) }

                rule node() -> Node
                = x:leaf() / x:inner() { x }

                rule inner() -> Node
                = "[" l:node() "," r:node() "]" { Node::Inner(Box::new(l), Box::new(r)) }

                pub(crate) rule root() -> Node
                = "[" l:node() "," r:node() "]" { Node::Inner(Box::new(l), Box::new(r)) }
            }
        }
        parser::root(line)
            .map_err(|e| format!("Cannot parse line ({}), because of error: {}", line, e.to_string()))
    }
}

mod tests {
    #[test]
    fn test1() {
        assert_eq!(2, 1 + 1);
    }
}


