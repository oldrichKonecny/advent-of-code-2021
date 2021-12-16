use std::collections::{HashSet, VecDeque};

fn main() {
    let input = include_str!("../input.txt");

    println!("First solution: {}", first_solution(&input));
    second_solution(&input);
}

fn second_solution(input: &str) {
    let mut graph = Graph::from(input).unwrap();
    graph.fold_all();
    graph.print();
}

fn first_solution(input: &str) -> usize {
    let mut graph = Graph::from(input).unwrap();
    graph.fold_once().unwrap();
    graph.count_points()
}

#[derive(Debug)]
enum Instruction {
    X(u32),
    Y(u32),
}

#[derive(Debug)]
struct Graph {
    points: Vec<(u32, u32)>,
    instructions: VecDeque<Instruction>,
}

impl Graph {
    fn print(&self) {
        let max_by_x = self.points.iter().max_by(|(x1, _), (x2, _)| x1.cmp(x2)).unwrap();
        let max_by_y = self.points.iter().max_by(|(_, y1), (_, y2)| y1.cmp(y2)).unwrap();

        let mut canvas: Vec<Vec<bool>> = vec![vec![false; 1 + max_by_x.0 as usize]; 1 + max_by_y.1 as usize];
        for (x, y) in self.points.iter() {
            canvas[*y as usize][*x as usize] = true;
        }

        println!();
        for i in 0..=max_by_y.1 as usize {
            for j in 0..=max_by_x.0 as usize {
                if canvas[i][j] {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            print!("\n");
        }
    }
    

    fn fold_all(&mut self) {
        (0..self.instructions.len()).for_each(|_| self.fold_once().unwrap())
    }

    fn fold_once(&mut self) -> Result<(), String> {
        let instruction = self.instructions.pop_front();

        match instruction {
            Some(Instruction::X(n)) => {
                self.points.iter_mut()
                    .filter(|(x, _)| *x > n)
                    .for_each(|(x, _)| *x -= 2 * (*x - n));
            }
            Some(Instruction::Y(n)) => {
                self.points.iter_mut()
                    .filter(|(_, y)| *y > n)
                    .for_each(|(_, y)| *y -= 2 * (*y - n));
            }
            None => return Err("No more instructions.".to_string()),
        }
        self.points.sort_unstable();
        self.points.dedup();

        Ok(())
    }

    fn count_points(&self) -> usize {
        self.points.len()
    }

    fn from(input: &str) -> Result<Self, String> {
        peg::parser! {
            grammar parser() for str {
                pub(crate) rule graph() -> Graph
                = points:parse_points() new_line() instructions:parse_instructions() { Graph { points, instructions, } }

                rule parse_points() -> Vec<(u32, u32)>
                = vec:point() ** delimeter() { vec }

                rule point() -> (u32, u32)
                = x:number() "," y:number() { (x, y) }

                rule number() -> u32
                = n:$(['0'..='9']+) { n.parse().unwrap() }

                rule parse_instructions() -> VecDeque<Instruction>
                = vec:instruction() ** delimeter() { vec.into_iter().collect::<VecDeque<_>>() }

                rule instruction() -> Instruction
                = "fold along " typ:instruction_type() "=" num:number() {
                    match typ.as_ref() {
                        "x" => Instruction::X(num),
                        "y" => Instruction::Y(num),
                        _ => panic!("wrong type {}", typ),
                } }

                rule instruction_type() -> String
                = n:$(['x' | 'y']) { n.to_string() }

                rule new_line()
                = "\n\n" / "\r\n\r\n"

                rule delimeter()
                = "\n" / "\r\n"
            }
        }
        parser::graph(input)
            .map_err(|err| format!("Error while parsing input: {}, err: {}", input, err))
    }
}

mod tests {
    #[test]
    fn test1() {
        assert_eq!(2, 1 + 1);
    }
}