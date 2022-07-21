const MAX_DEPTH: usize = 4;

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

    roots.iter_mut().for_each(|r| r.reduce());

    0
}



enum Node {
    Leaf(u8),
    Inner {
        left: u8,
        right: u8,
        depth: usize,

    }
}

impl Node {
    fn reduce(&mut self) {
        loop {
            if self.try_explode() {
                continue
            } else {
                if self.try_split() {
                    continue
                } else {
                    break
                }
            }
        }
    }

    fn try_explode(&mut self) -> bool {
        println!("\nTrying to explode: {:?}", self);
        let mut node = self;
        for depth in 1..=4 {
            if let Node::Inner(l, r) = node {
                println!("LEFT: {:?} | RIGHT: {:?}", l, r);

                if depth == MAX_DEPTH {
                    println!("Must explode these two: LEFT: {:?} | RIGHT: {:?}", l, r);
                }
                match **l {
                    Node::Inner(_, _) => node = l,
                    Node::Leaf(_) => node = r,
                }
            } else {
                break;
            }
        }

        false
    }

    fn try_split(&mut self) -> bool {
        // println!("trying to split: {:?}", self);
        false
    }
}

#[derive(Debug)]
enum UnmarkedNode {
    Leaf(u8),
    Inner(Box<UnmarkedNode>, Box<UnmarkedNode>),
}

impl UnmarkedNode {
    fn parse(line: &str) -> Result<Self, String> {
        peg::parser! {
            grammar parser() for str {
                rule leaf() -> UnmarkedNode
                = n:$(['0'..='9']+) { Node::Leaf(n.parse::<u8>().unwrap()) }

                rule node() -> UnmarkedNode
                = x:leaf() / x:inner() { x }

                rule inner() -> UnmarkedNode
                = "[" l:node() "," r:node() "]" { UnmarkedNode::Inner(Box::new(l), Box::new(r)) }

                pub(crate) rule root() -> UnmarkedNode
                = "[" l:node() "," r:node() "]" { UnmarkedNode::Inner(Box::new(l), Box::new(r)) }
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


