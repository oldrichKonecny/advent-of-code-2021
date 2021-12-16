fn main() {
    let input = include_str!("../input.txt");

    println!("First solution: {}", first_solution(&input));
    println!("Second solution: {}", second_solution(&input));
}

fn second_solution(input: &str) -> u64 {
    let mut split = input.split("\r\n\r\n");
    let board_inputs = split.next().unwrap().split(',')
        .map(|s| s.parse::<u32>().unwrap_or_else(|e| panic!("Cannot parse: {}, with error: {}", s, e)))
        .collect::<Vec<_>>();
    let boards = split.into_iter()
        .map(Board::from)
        .collect::<Vec<_>>();

    let (sum, last_input) = play_bingo_lose(&board_inputs, boards);
    sum * last_input
}

fn play_bingo_lose(values: &[u32], mut boards: Vec<Board>) -> (u64, u64) {
    for (i, x) in values.iter().enumerate() {
        for b in boards.iter_mut() {
            b.try_mark(*x);
        }
        if i >= 4 {
            boards.retain(|b| !b.check_for_win());
        }
        if boards.len() == 1 {
            // finish last bingo and get result
            let mut final_board = boards.remove(0);
            for j in values[i..].iter() {
                if final_board.try_mark(*j) && final_board.check_for_win() {
                    return (final_board.sum_non_marked(), *j as u64)
                }
            }
        }
    }
    panic!("no board was last..");
}

fn first_solution(input: &str) -> u64 {
    let mut split = input.split("\r\n\r\n");
    let board_inputs = split.next().unwrap().split(',')
        .map(|s| s.parse::<u32>().unwrap_or_else(|e| panic!("Cannot parse: {}, with error: {}", s, e)))
        .collect::<Vec<_>>();
    let mut boards = split.into_iter()
        .map(Board::from)
        .collect::<Vec<_>>();

    let (sum, last_input) = play_bingo_win(&board_inputs, &mut boards);
    sum * last_input
}

fn play_bingo_win(values: &[u32], boards: &mut [Board]) -> (u64, u64) {
    for (i, x) in values.iter().enumerate() {
        for b in boards.iter_mut() {
            if b.try_mark(*x) && i >= 4 && b.check_for_win() {
                return (b.sum_non_marked(), *x as u64)
            }
        }
    }
    panic!("No board was winning..");
}

#[derive(Debug)]
struct Board {
    vec: Vec<Vec<(bool, u32)>>,
    counter: Vec<usize>,
    square: usize,
}

impl Board {
    fn from(input: &str) -> Self {
        let mut square = None;
        let vec = input.lines()
            .map(|s| {
                let inner_vec = s.split_whitespace()
                    .filter(|s| !s.is_empty())
                    .map(|s| (false, s.parse().unwrap()))
                    .collect::<Vec<(bool, u32)>>();
                if square.is_some() && square.unwrap() != inner_vec.len()  {
                    panic!("Different row length.");
                } else {
                    square = Some(inner_vec.len());
                }
                inner_vec
            })
            .collect::<Vec<_>>();
        let square = square.expect("Something went wrong, seems like wrong input..");
        assert_eq!(square, vec.len());

        Self {
            vec,
            counter: vec![0; square * 2],
            square
        }
    }

    fn try_mark(&mut self, value: u32) -> bool {
        for (i, vec) in self.vec.iter_mut().enumerate() {
            for (j, val) in vec.iter_mut().enumerate() {
                if val.1 == value && !val.0 {
                    val.0 = true;
                    self.counter[i] += 1;
                    self.counter[self.square + j] += 1;
                    return true
                }
            }
        }
        false
    }

    fn check_for_win(&self) -> bool {
        self.counter.iter().any(|i| *i == self.square)
    }

    fn sum_non_marked(&self) -> u64 {
        let mut sum = 0;
        for i in self.vec.iter() {
            for j in i.iter() {
                if j.0 == false {
                    sum += j.1 as u64;
                }
            }
        }
        sum
    }
}

mod tests {
    #[test]
    fn test1() {
        assert_eq!(2, 1 + 1);
    }
}