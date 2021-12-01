#![feature(test)]
extern crate test;

fn main() {
    let input = include_str!("../input.txt").lines()
        .map(|l| l.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    println!("res1: {}", first_solution(&input));
    println!("res1 window: {}", first_solution_window(&input));
    println!("res2: {}", second_solution(&input));
}

fn second_solution(input: &[i32]) -> usize {
    input.windows(4)
        .filter(|&win| win[0] < win[3] )
        .count()
}

fn first_solution(input: &[i32]) -> u64 {
    let mut res = 0;
    let mut prev = input[0];

    for &i in input[1..].iter() {
        if i > prev {
            res += 1;
        }
        prev = i;
    }
    res
}

fn first_solution_window(input: &[i32]) -> usize {
    input.windows(2)
        .filter(|&win| win[0] < win[1])
        .count()
}


#[cfg(test)]
mod tests {
    use test::Bencher;

    use crate::{first_solution, first_solution_fast, first_solution_window, second_solution};

    #[bench]
    fn bench_sol1_loop(b: &mut Bencher) {
        let input = include_str!("../input.txt").lines()
            .map(|l| l.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();

        b.iter(|| {
            first_solution(&input)
        })
    }

    #[bench]
    fn bench_sol1_window(b: &mut Bencher) {
        let input = include_str!("../input.txt").lines()
            .map(|l| l.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();

        b.iter(|| {
            first_solution_window(&input)
        })
    }

    #[bench]
    fn bench_sol2(b: &mut Bencher) {
        let input = include_str!("../input.txt").lines()
            .map(|l| l.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();

        b.iter(|| {
            second_solution(&input)
        })
    }
}