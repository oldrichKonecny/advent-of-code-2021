fn main() {
    let input = include_str!("../input.txt");

    println!("First solution: {}", first_solution(&input));
    println!("Second solution: {}", second_solution(&input));
}

fn second_solution(input: &str) -> u64 {
    let mut scores = input
        .lines()
        .map(|l| Line::from(l))
        .filter_map(|l| l.incomplete_score())
        .collect::<Vec<_>>();

    scores.sort();
    scores[scores.len() / 2]
}

fn first_solution(input: &str) -> u32 {
    input
        .lines()
        .map(|l| Line::from(l))
        .filter_map(|l| l.corrupted_score())
        .sum()
}

#[derive(Debug)]
struct Line<'a> {
    str: &'a str,
    state: LineState,
}

#[derive(Debug)]
enum LineState {
    Ok,
    Incomplete(Vec<char>),
    Corrupted((char, usize)),
}

impl<'a> Line<'a> {
    fn from(str: &'a str) -> Self {
        let mut stack = Vec::new();
        let mut state = None;
        for (i, c) in str.chars().enumerate() {
            match c {
                '(' | '[' | '{' | '<' => stack.push(c),
                ')' | ']' | '}' | '>' => {
                    if let Some(x) = stack.pop() {
                        match (x, c) {
                            ('(', ')') | ('[', ']') | ('{', '}') | ('<', '>') => continue,
                            _ => (),
                        }
                    }
                    state = Some(LineState::Corrupted((c, i)));
                    break;
                }
                _ => panic!("Unexpected token: {} at position: {}.", c, i),
            }
        }

        if state.is_none() && stack.len() > 0 {
            state = Some(LineState::Incomplete(stack));
        } else if state.is_none() {
            state = Some(LineState::Ok);
        }
        {}

        Self {
            str,
            state: state.unwrap(),
        }
    }

    fn corrupted_score(&self) -> Option<u32> {
        if let LineState::Corrupted((c, _)) = self.state {
            match c {
                ')' => Some(3),
                ']' => Some(57),
                '}' => Some(1197),
                '>' => Some(25137),
                _ => panic!("Wrong character: {}, not sure how it got here tho..", c),
            }
        } else {
            None
        }
    }

    fn incomplete_score(&self) -> Option<u64> {
        if let LineState::Incomplete(stack) = &self.state {
            let sum = stack
                .iter()
                .rev()
                .map(|c| match c {
                    '(' => 1,
                    '[' => 2,
                    '{' => 3,
                    '<' => 4,
                    _ => panic!("Unknown character: {}, should not be here.", c),
                })
                .fold(0u64, |acc, x| acc * 5 + x);
            Some(sum)
        } else {
            None
        }
    }
}

mod tests {
    #[test]
    fn test1() {
        assert_eq!(2, 1 + 1);
    }
}
