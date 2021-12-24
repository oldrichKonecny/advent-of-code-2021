use std::collections::VecDeque;

fn main() {
    let input = include_str!("../input.txt");

    println!("First solution: {}", first_solution(&input));
    println!("Second solution: {}", second_solution(&input));
}

fn second_solution(input: &str) -> usize {
    let mut graph = parse_input(input);
    let len_x = graph.len();
    let len_y = graph[0].len();
    for t in 0..4 {
        for i in 0..len_x {
            let mut vec = Vec::new();
            for j in 0..len_y {
                let val = graph[i + (t * len_x)][j].0 + 1;
                let val = if val > 9 { val - 9 } else { val };
                vec.push((val, None));
            }
            graph.push(vec);
        }
    }

    for vec in graph.iter_mut() {
        let len = vec.len();
        while vec.len() < 5 * len {
            let val = vec[vec.len() - len].0 + 1;
            let val = if val > 9 { val - 9 } else { val };
            vec.push((val, None));
        }
    }

    flood(&mut graph);

    let last_line = &graph[graph.len()-1];
    last_line[last_line.len()-1].1.unwrap()
}

fn first_solution(input: &str) -> usize {
    let mut graph = parse_input(input);
    flood(&mut graph);

    let last_line = &graph[graph.len()-1];
    last_line[last_line.len()-1].1.unwrap()
}

fn parse_input(input: &str) -> Vec<Vec<(u32, Option<usize>)>> {
    input.lines()
        .map(|line| line.chars().map(|c| (u32::from(c) - 48, None)).collect::<Vec<_>>())
        .collect::<Vec<_>>()
}

fn flood(graph: &mut Vec<Vec<(u32, Option<usize>)>>) {
    {
        let start = &mut graph[0][0].1;
        start.replace(0);
    }

    let mut queue: VecDeque<((usize, usize), usize)> = VecDeque::new();
    let mut x = ((0, 0), graph[0][0].1.unwrap());

    loop {
        if x.0.0 > 0 { //TOP
            let node = &mut graph[x.0.0 -1][x.0.1];
            let new_cost = x.1 + node.0 as usize;
            if node.1.is_none() || node.1.unwrap() > new_cost {
                node.1.replace(new_cost);
                queue.push_back(((x.0.0 - 1, x.0.1), new_cost));
            }
        }
        if x.0.0 < graph.len() -1 { //BOTTOM
            let node = &mut graph[x.0.0 + 1][x.0.1];
            let new_cost = x.1 + node.0 as usize;
            if node.1.is_none() || node.1.unwrap() > new_cost {
                node.1.replace(new_cost);
                queue.push_back(((x.0.0 + 1, x.0.1), new_cost));
            }
        }
        if x.0.1 > 0 { //LEFT
            let node = &mut graph[x.0.0][x.0.1 - 1];
            let new_cost = x.1 + node.0 as usize;
            if node.1.is_none() || node.1.unwrap() > new_cost {
                node.1.replace(new_cost);
                queue.push_back(((x.0.0, x.0.1 - 1), new_cost));
            }
        }
        if x.0.1 < graph[x.0.0].len() -1 { //RIGHT
            let node = &mut graph[x.0.0][x.0.1 + 1];
            let new_cost = x.1 + node.0 as usize;
            if node.1.is_none() || node.1.unwrap() > new_cost {
                node.1.replace(new_cost);
                queue.push_back(((x.0.0, x.0.1 + 1), new_cost));
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
