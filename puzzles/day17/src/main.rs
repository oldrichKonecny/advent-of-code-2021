use std::ops::RangeInclusive;
use regex::Regex;

fn main() {
    let input = include_str!("../input.txt");

    println!("First solution: {}", first_solution(&input));
    println!("Second solution: {}", second_solution(&input));
}

fn second_solution(input: &str) -> usize {
    let target_area = TargetArea::from(input);
    let mut counter = 0;

    for i in 0..185 {
        for j in -126..300 {
            let (_, is_in) = compute_trajectory((i, j), &target_area);
            if is_in {
                counter += 1;
            }
        }
    }

    counter
}

fn first_solution(input: &str) -> isize {
    let target_area = TargetArea::from(input);

    // for i in 0..isize::MAX {
    //     let point = (18, i);
    //     let (trajectory, is_in) = compute_trajectory(point, 250, &target_area);
    //     if is_in {
    //         println!("point: {} {}  , is_in: {}", point.0, point.1, is_in);
    //     }
    // }
    let (trajectory, is_in) = compute_trajectory((18, 124), &target_area);
    let max = trajectory.iter().map(|p| p.1).max();

    max.unwrap()
}

fn compute_trajectory(point: (isize, isize), area: &TargetArea) -> (Vec<(isize, isize)>, bool) {
    let mut result = Vec::new();
    result.push((0, 0));
    result.push(point);
    if area.is_in(point) {
        return (result, true);
    }

    let mut prev = point;
    let mut is_in = false;
    let mut i = 1;
    loop {
        let new = (prev.0 + (point.0 - i).max(0), prev.1 + (point.1 - i));
        result.push(new);
        prev = new;
        i += 1;

        if area.is_in(new) {
            is_in = true;
            break;
        } else if *area.y.start() > new.1 || *area.x.end() < new.0 {
            is_in = false;
            break;
        }
    };

    (result, is_in)
}

#[derive(Debug)]
struct TargetArea {
    x: RangeInclusive<isize>,
    y: RangeInclusive<isize>,
}

enum Position {
    InArea,
    OutOfReach,
    StillGoing,
    Missing,
}

impl TargetArea {
    fn from(input: &str) -> Self {
        let regex = Regex::new(r"^target area: x=([-]?\d+)\.\.([-]?\d+), y=([-]?\d+)\.\.([-]?\d+)$").unwrap();
        let mut cap = regex.captures(input).unwrap();
        let x1 = cap[1].parse::<isize>().unwrap();
        let x2 = cap[2].parse::<isize>().unwrap();
        let y1 = cap[3].parse::<isize>().unwrap();
        let y2 = cap[4].parse::<isize>().unwrap();
        Self {
            x: (x1..=x2),
            y: (y1..=y2),
        }
    }

    fn is_in(&self, point: (isize, isize)) -> bool {
        self.x.contains(&point.0) && self.y.contains(&point.1)
    }
}

mod tests {
    #[test]
    fn test1() {
        assert_eq!(2, 1 + 1);
    }
}


