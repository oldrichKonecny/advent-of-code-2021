fn main() {
    let input = include_str!("../input.txt")
        .lines()
        .map(Direction::parse)
        .collect::<Vec<_>>();

    println!("First solution: {}", first_solution(&input));
    println!("Second solution: {}", second_solution(&input)); //1997106066
}

fn second_solution(directions: &[Direction]) -> i64 {
    let mut horizontal_pos = 0;
    let mut depth = 0;
    let mut aim = 0;
    for dir in directions {
        match dir {
            Direction::Forward(x) => {
                horizontal_pos += *x as i64;
                depth += aim * *x as i64;
            }
            Direction::Up(x) => aim -= *x as i64,
            Direction::Down(x) => aim += *x as i64,
        }
    }

    horizontal_pos * depth
}

fn first_solution(directions: &[Direction]) -> i32 {
    let mut horizontal_pos = 0;
    let mut depth = 0;
    for dir in directions {
        match dir {
            Direction::Forward(x) => horizontal_pos += x,
            Direction::Up(x) => depth -= x,
            Direction::Down(x) => depth += x,
        }
    }

    horizontal_pos * depth
}

enum Direction {
    Forward(i32),
    Up(i32),
    Down(i32),
}

impl Direction {
    fn parse(line: &str) -> Direction {
        let (enm, value) = line.split_once(' ').unwrap();
        let value = value.parse::<i32>().unwrap();
        match enm {
            "Forward" => Direction::Forward(value),
            "Up" => Direction::Up(value),
            "Down" => Direction::Down(value),
            x => panic!("Unexpected input token: {}", x),
        }
    }
}

mod tests {
    #[test]
    fn test1() {
        assert_eq!(2, 1 + 1);
    }

}