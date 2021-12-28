use std::collections::HashMap;
use regex::Regex;

fn main() {
    let input = include_str!("../input_tst.txt");

    println!("First solution: {}", first_solution(&input));
    println!("Second solution: {}", second_solution(&input));
}

fn second_solution(input: &str) -> usize {
    let mut input = input.lines();
    let mut first_position = parse_starting_position(input.next().unwrap()) as u64;
    let mut sec_position = parse_starting_position(input.next().unwrap()) as u64;

    let mut p1_points = solve_rec(first_position, 1);
    let mut p2_points= solve_rec(sec_position, 1);

    println!("p1: {:?}", p1_points);
    println!("p2: {:?}", p2_points);

    0
}



fn solve_rec(point: u64, times: u64) -> HashMap<u64, u64> {
    let occurrence_map = occurrences_map();
    let mut res = HashMap::new();

    for (key, val) in occurrence_map {
        let new_position = new_position_for_move(key as u32, point as u32) as u64;
        res.entry(new_position).or_insert(val * times);
    }

    res
}

fn occurrences_map() -> HashMap<u64, u64> {
    let mut res = HashMap::new();

    res.entry(3).or_insert(1);
    res.entry(4).or_insert(3);
    res.entry(5).or_insert(6);
    res.entry(6).or_insert(7);
    res.entry(7).or_insert(6);
    res.entry(8).or_insert(3);
    res.entry(9).or_insert(1);

    res
}

// 3 -> 1
// 4 -> 3
// 5 -> 6
// 6 -> 7
// 7 -> 6
// 8 -> 3
// 9 -> 1

fn first_solution(input: &str) -> u32 {
    let mut input = input.lines();
    let mut first_position = parse_starting_position(input.next().unwrap());
    let mut sec_position = parse_starting_position(input.next().unwrap());

    let mut dice = DeterministicDice::new();

    let mut p1_points = 0;
    let mut p2_points = 0;
    loop {
        let p1_move = dice.next().unwrap() + dice.next().unwrap() + dice.next().unwrap();
        first_position = new_position_for_move(p1_move, first_position);
        p1_points += first_position;
        if p1_points >= 1000 {
            break p2_points * dice.counter;
        }

        let p2_move = dice.next().unwrap() + dice.next().unwrap() + dice.next().unwrap();
        sec_position = new_position_for_move(p2_move, sec_position);
        p2_points += sec_position;
        if p2_points >= 1000 {
            break p1_points * dice.counter;
        }
    }
}

fn new_position_for_move(dice_points: u32, position: u32) -> u32 {
    (((dice_points + position) - 1) % 10) + 1
}

#[derive(Debug)]
struct DeterministicDice {
    counter: u32,
    value: u32,
}

impl DeterministicDice {
    fn new() -> Self {
        DeterministicDice { counter: 0, value: 1 }
    }
}

impl Iterator for DeterministicDice {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        self.counter += 1;

        let res = self.value;
        if res + 1 > 100 {
            self.value = 1;
        } else {
            self.value += 1;
        }

        Some(res)
    }
}

fn parse_starting_position(line: &str) -> u32 {
    let regex = Regex::new(r"^Player \d starting position: (\d+)$").unwrap();
    let cap = regex.captures(line).unwrap();
    cap[1].parse().unwrap()
}

mod tests {
    #[test]
    fn test1() {
        assert_eq!(2, 1 + 1);
    }
}


