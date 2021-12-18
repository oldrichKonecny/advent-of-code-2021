use std::collections::HashMap;

fn main() {
    let input = include_str!("../input.txt");

    println!("First solution: {}", first_solution(&input));
    println!("Second solution: {}", second_solution(&input));
}

fn second_solution(input: &str) -> usize {
    let lines = input
        .lines()
        .map(|l| Line::from(l).unwrap_or_else(|e| panic!("Cannot create line from input: {:?}", e)))
        .collect::<Vec<_>>();

    let mut graph: HashMap<(u32, u32), u32> = HashMap::new();
    for line in lines.iter() {
        for point in line.get_all_points().into_iter() {
            let v = graph.entry(point).or_insert(0);
            *v += 1;
        }
    }

    graph.values().filter(|v| **v > 1).count()
}

fn first_solution(input: &str) -> usize {
    let lines = input
        .lines()
        .map(|l| Line::from(l).unwrap_or_else(|e| panic!("Cannot create line from input: {:?}", e)))
        .filter(Line::is_horizontal_or_vertical)
        .collect::<Vec<_>>();

    let mut graph: HashMap<(u32, u32), u32> = HashMap::new();
    for line in lines.iter() {
        for point in line.get_all_points().into_iter() {
            let v = graph.entry(point).or_insert(0);
            *v += 1;
        }
    }

    graph.values().filter(|v| **v > 1).count()
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Line {
    x1: u32,
    y1: u32,
    x2: u32,
    y2: u32,
}

#[derive(Debug, thiserror::Error)]
enum Error {
    #[error("could not parse {0}: {1}")]
    ParseError(String, String),
}

impl Line {
    fn from(str: &str) -> Result<Self, Error> {
        peg::parser! {
            grammar parser() for str {
                pub(crate) rule line() -> Line
                = x1:num() "," y1:num() " -> " x2:num() "," y2:num() { Line {x1, y1, x2, y2} }

                rule num() -> u32
                = s:$(['0'..='9']+) { s.parse().unwrap() }
            }
        }

        parser::line(str).map_err(|e| Error::ParseError(str.into(), String::from(e.to_string())))
    }

    fn is_horizontal_or_vertical(&self) -> bool {
        self.x1 == self.x2 || self.y1 == self.y2
    }

    fn get_all_points(&self) -> Vec<(u32, u32)> {
        if self.x1 == self.x2 {
            (self.y1.min(self.y2)..=self.y2.max(self.y1))
                .map(|y| (self.x1, y))
                .collect()
        } else if self.y1 == self.y2 {
            (self.x1.min(self.x2)..=self.x2.max(self.x1))
                .map(|x| (x, self.y1))
                .collect()
        } else {
            let (l, h) = if self.x1 < self.x2 {
                ((self.x1, self.y1), (self.x2, self.y2))
            } else {
                ((self.x2, self.y2), (self.x1, self.y1))
            };
            let y_multiplier: i32 = if l.1 < h.1 { 1 } else { -1 };
            (l.0..=h.0)
                .enumerate()
                .map(|(i, x)| (x, (l.1 as i32 + i as i32 * y_multiplier) as u32))
                .collect()
        }
    }
}

mod tests {
    #[test]
    fn test1() {
        assert_eq!(2, 1 + 1);
    }
}
