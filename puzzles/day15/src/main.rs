use std::collections::VecDeque;

fn main() {
    let input = include_str!("../input.txt");

    println!("First solution: {}", first_solution(&input));
    println!("Second solution: {}", second_solution(&input));
}

fn second_solution(input: &str) -> usize {
    0
}

fn first_solution(input: &str) -> usize {
    let graph = parse_input(input);
    0
}

fn parse_input(input: &str) -> Vec<Vec<(u32, Option<usize>)>> {
    input.lines()
        .map(|line| line.chars().map(|c| (u32::from(c), None)).collect::<Vec<_>>())
        .collect::<Vec<_>>()
}

fn flood(graph: &mut Vec<Vec<(u32, Option<usize>)>>, point: (usize, usize)) {
    let mut queue: VecDeque<(Option<(usize, usize)>, Option<Option<&mut (u32, Option<usize>)>>)> = VecDeque::new();
    let mut x = (Some(point), graph.get_mut(point.0).map(|inner| inner.get_mut(point.1)));
    let mut prev_cost = 0;
    loop {
        if let (Some((px, py)),Some(Some((val, cost)))) = x {
            if cost.is_none() {

            }
        }


        if queue.is_empty() {
            break;
        } else {
            x = queue.pop_front().unwrap();
        }
    }
}



mod tests {
    #[test]
    fn test1() {
        assert_eq!(2, 1 + 1);
    }
}
