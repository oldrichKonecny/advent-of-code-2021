use std::borrow::BorrowMut;
use std::cell::{Cell, RefCell};
use std::ops::Add;
use std::rc::Rc;

fn main() {
    let input = include_str!("../input.txt");

    println!("First solution: {}", first_solution(&input)); // 526
    // println!("Second solution: {}", second_solution(&input)); //

    println!("second solution test: {}", second_solution("2199943210
3987894921
9856789892
8767896789
9899965678"));
}

fn second_solution(input: &str) -> usize {
    let ranges = input.lines()
        .map(|l| {
            let mut res = Vec::new();
            let mut tmp = None;
            for (i, c) in l.chars().enumerate() {
                let c = c as u32 - 48;
                if tmp.is_none() && c != 9 && i == l.len() - 1 {
                    let t = (i, i);
                    res.push(t);
                } else if tmp.is_none() && c != 9 {
                    tmp = Some(i);
                } else if tmp.is_some() && c == 9 {
                    let t = (tmp.unwrap(), i - 1);
                    res.push(t);
                    tmp = None;
                } else if tmp.is_some() && i == l.len() - 1 {
                    let t = (tmp.unwrap(), i);
                    res.push(t);
                }
            }
            res
        })
        .collect::<Vec<_>>();
    let mut all_bounds: Vec<Vec<Bound>> = Vec::new();
    for (i, line) in ranges.iter().enumerate() {
        let mut bounds = Vec::new();
        for r in line.iter() {
            if i == 0 {
                bounds.push(Bound::from(r.0, r.1, true));
                continue;
            }

            let prev_bounds = all_bounds.get_mut(i - 1).unwrap();
            let mut affected_bounds = prev_bounds.iter_mut().filter(|b| b.is_connected(r.0, r.1)).collect::<Vec<_>>();
            match affected_bounds.len() {
                0 => bounds.push(Bound::from(r.0, r.1, true)),
                1 => {
                    let new_bound = Bound::from_bound(r.0, r.1, &mut affected_bounds[0]);
                    bounds.push(new_bound);
                },
                _ => {
                    let first_bound = affected_bounds[0];
                    for b in &affected_bounds[1..] {
                        b.is_top = false
                    }
                }
            }
        }
        all_bounds.push(bounds);
    }

    for (i, bounds) in all_bounds.iter().enumerate() {
        for b in bounds.iter() {
            println!("{}: {:?}", i, b);
        }
        println!();
    }

    let mut sums = all_bounds.iter()
        .flat_map(|bounds| bounds.iter().filter(|b| b.is_top))
        .map(|b| b.sum.take())
        .collect::<Vec<_>>();
    sums.sort();
    sums.into_iter().rev().take(3).fold(1, |acc, s| s * acc)
}

#[derive(Debug)]
struct Bound {
    start: usize,
    end: usize,
    sum: Rc<RefCell<usize>>,
    is_top: bool,
}

impl Bound {
    fn from(start: usize, end: usize, is_top: bool) -> Self {
        assert!(start <= end, "Bound start is bigger than end.");
        Self {
            start,
            end,
            sum: Rc::new(RefCell::new(end - start + 1)),
            is_top,
        }
    }

    fn from_bound(start: usize, end: usize, old_bound: &mut Bound) -> Self {
        assert!(start <= end, "Bound start is bigger than end.");
        old_bound.sum.replace((end - start + 1) + old_bound.sum.take());
        Self {
            start,
            end,
            sum: old_bound.sum.clone(),
            is_top: false,
        }
    }

    fn is_connected(&self, r_start: usize, r_end: usize) -> bool {
        self.start <= r_end && self.end >= r_start
    }
}

fn first_solution(input: &str) -> u64 {
    let matrix = Matrix::from(input);
    matrix.get_risk_levels_sum()
}

struct Matrix {
    vec: Vec<Vec<u32>>
}

impl Matrix {
    fn from(str: &str) -> Self {
        let vec = str.lines()
            .map(|l| l.chars().map(|c| c as u32 - 48).collect::<Vec<_>>())
            .collect::<Vec<_>>();
        Self {
            vec
        }
    }

    fn check_low_point(&self, row: usize, col: usize) -> bool {
        let r = self.vec.get(row).expect(&format!("Row {} does not exist", row));
        let value = *r.get(col).expect(&format!("Column {} does not exist", col));

        (col == 0 || value < r[col - 1]) &&
        (col >= r.len() - 1 || value < r[col + 1]) &&
        (row == 0 || value < self.vec[row - 1][col]) &&
        (row >= self.vec.len() - 1 || value < self.vec[row + 1][col])
    }

    fn get_risk_levels_sum(&self) -> u64 {
        let mut res = 0;
        for (i, row) in self.vec.iter().enumerate() {
            for (j, val) in row.iter().enumerate() {
                if self.check_low_point(i, j) {
                    res += *val as u64 + 1;
                }
            }
        }
        res
    }
}


mod tests {
    #[test]
    fn test1() {
        assert_eq!(2, 1 + 1);
    }
}