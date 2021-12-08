fn main() {
    let input = include_str!("../input.txt");

    println!("First solution: {}", first_solution(&input)); //543
    println!("Second solution: {}", second_solution(&input)); //994266
}

fn second_solution(input: &str) -> u32 {
    let lines = input.lines()
        .map(|s| Line::from(s).unwrap())
        .collect::<Vec<_>>();

    lines.iter()
        .map(|l| l.solve_output())
        .sum()
}

fn first_solution(input: &str) -> usize {
    let lines = input.lines()
        .map(|s| Line::from(s).unwrap())
        .collect::<Vec<_>>();

    lines.iter()
        .flat_map(|l| l.output.iter()
            .filter(|o| o.len() == 2 || o.len() == 3 || o.len() == 4 || o.len() == 7))
        .count()
}

#[derive(Debug)]
struct Line {
    input: Vec<SevenString>,
    output: Vec<SevenString>,
}

#[derive(Debug)]
struct SevenString(String);
type SevenDisplay = (char, char, char, char, char, char, char);

impl SevenString {
    fn len(&self) -> usize {
        self.0.len()
    }

    fn interpret(&self, translation: &SevenDisplay) -> u32 {
        let chars = self.0.chars().collect::<Vec<_>>();

        match chars.len() {
            2 => 1,
            3 => 7,
            4 => 4,
            7 => 8,
            5 => {
                if !chars.contains(&translation.1) && !chars.contains(&translation.5) {
                    2
                } else if !chars.contains(&translation.1) && !chars.contains(&translation.4) {
                    3
                } else if !chars.contains(&translation.2) && !chars.contains(&translation.4) {
                    5
                } else {
                    panic!("Cannot interpret length 5 str into digit")
                }
            },
            6 => {
                if !chars.contains(&translation.3) {
                    0
                } else if !chars.contains(&translation.2) {
                    6
                } else if !chars.contains(&translation.4) {
                    9
                } else {
                    panic!("Cannot interpret length 6 str into digit")
                }
            }
            x => panic!("should not have {} digit count", x),
        }
    }

    fn sub(&self, rhs: &Self) -> Vec<char> {
        let rhs = rhs.0.chars().collect::<Vec<_>>();
        self.0.chars()
            .filter(|c| !rhs.contains(c))
            .collect()
    }
    fn same(&self, rhs: &Self) -> Vec<char> {
        let rhs = rhs.0.chars().collect::<Vec<_>>();
        self.0.chars()
            .filter(|c| rhs.contains(c))
            .collect()
    }
}

impl Line {
    fn from(str: &str) -> Result<Self, String> {
        peg::parser! {
            grammar parser() for str {
                pub(crate) rule line() -> Line
                = input:str_num()*<10> "|" output:str_num()*<4> { Line { input, output, } }

                rule str_num() -> SevenString
                = separator()? s:$(['a'..='g']*<2,7>) separator()? { SevenString(s.to_string()) }

                rule separator()
                = [' ' | '\n' | '\t']
            }
        }
        parser::line(str)
            .map_err(|err| format!("Error while parsing input: {}, err: {}", str, err))
    }

    fn determine_positions(&self) -> SevenDisplay {
        let one = self.input.iter().filter(|ss| ss.len() == 2).next().unwrap();
        let four = self.input.iter().filter(|ss| ss.len() == 4).next().unwrap();
        let seven = self.input.iter().filter(|ss| ss.len() == 3).next().unwrap();
        let eight = self.input.iter().filter(|ss| ss.len() == 7).next().unwrap();
        let x0 = seven.sub(&one)[0];

        let mut six_digits = self.input.iter().filter(|ss| ss.len() == 6).collect::<Vec<_>>();
        let x2 = six_digits.iter()
            .map(|ss| one.sub(ss))
            .filter(|v| v.len() == 1)
            .next().unwrap()[0];

        let x5 = one.0.chars().filter(|c| *c != x2).next().unwrap();

        let mut five_digits = self.input.iter().filter(|ss| ss.len() == 5).collect::<Vec<_>>();
        let three = five_digits.iter().filter(|ss| ss.0.contains(x0) && ss.0.contains(x2) && ss.0.contains(x5)).next().unwrap();
        let x3 = four.0.chars().filter(|c| *c != x2 && *c != x5 && three.0.contains(*c)).next().unwrap();
        let x1 = four.0.chars().filter(|c| *c != x2 && *c != x3 && *c != x5).next().unwrap();
        let x6 = three.0.chars().filter(|c| *c != x0 && *c != x2 && *c != x3 && *c != x5).next().unwrap();
        let x4 = eight.0.chars().filter(|c|*c != x0 && *c != x1 && *c != x2 && *c != x3 && *c != x5 && *c != x6).next().unwrap();
        (x0, x1, x2, x3, x4, x5, x6)
    }

    fn solve_output(&self) -> u32 {
        let translation = self.determine_positions();
        let res = self.output.iter()
            .map(|ss| ss.interpret(&translation))
            .collect::<Vec<_>>();

        res[0] * 1000 + res[1] * 100 + res[2] * 10 + res[3]
    }
}


mod tests {
    #[test]
    fn test1() {
        assert_eq!(2, 1 + 1);
    }
}