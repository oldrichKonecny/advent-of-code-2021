fn main() {
    let input = include_str!("../input.txt");

    println!("First solution: {}", first_solution(&input));
    println!("Second solution: {}", second_solution(&input));
}

fn second_solution(input: &str) -> i32 {
    let numbers = input
        .split(',')
        .map(|s| {
            s.parse::<i32>()
                .unwrap_or_else(|e| panic!("cannot parse input into numbers: {}", e))
        })
        .collect::<Vec<_>>();
    let mean = numbers.iter().map(|n| *n).sum::<i32>() as f64 / numbers.len() as f64;
    let mean = mean.floor() as i32;

    numbers
        .iter()
        .map(|n| {
            let x = (*n - mean).abs();
            ((x + 1) * x) / 2
        })
        .sum()
}

fn first_solution(input: &str) -> i32 {
    let mut numbers = input
        .split(',')
        .map(|s| {
            s.parse::<i32>()
                .unwrap_or_else(|e| panic!("cannot parse input into numbers: {}", e))
        })
        .collect::<Vec<_>>();
    numbers.sort();

    let median = if numbers.len() & 1 == 1 {
        numbers[(numbers.len() + 1) / 2]
    } else {
        (numbers[numbers.len() / 2] + numbers[(numbers.len() + 1) / 2]) / 2
    };

    numbers.iter().map(|n| (*n - median).abs()).sum()
}

mod tests {
    #[test]
    fn test1() {
        assert_eq!(2, 1 + 1);
    }
}
