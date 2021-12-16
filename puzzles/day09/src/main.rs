use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    let input = include_str!("../input.txt");

    println!("First solution: {}", first_solution(&input)); // 526
    println!("Second solution: {}", second_solution(&input)); //
}

fn second_solution(input: &str) -> usize {
    let ranges = Matrix::from(input).get_ranges();
    let mut all_bounds: Vec<Vec<Bound>> = Vec::new();
    for (i, vec) in ranges.iter().enumerate() {
        let mut bounds = Vec::new();
        for r in vec.iter() {
            if i == 0 {
                bounds.push(Bound::from(r.0, r.1, r.2));
                continue;
            }

            let prev_bounds = all_bounds.get_mut(i - 1).unwrap();
            let mut affected_bounds = prev_bounds.iter_mut().filter(|b| b.is_connected(r.0, r.1)).collect::<Vec<_>>();
            match affected_bounds.len() {
                0 => bounds.push(Bound::from(r.0, r.1, r.2)),
                1 => bounds.push(Bound::from_bound(r.0, r.1, r.2,&mut affected_bounds[0])),
                _ => {
                    let new_bound = Bound::from_bound(r.0, r.1, r.2,&mut affected_bounds[0]);
                    affected_bounds[1..].iter_mut().for_each(|b| {
                        let b_val = b.sum.take() + new_bound.sum.take();
                        new_bound.sum.replace(b_val);
                        b.sum = new_bound.sum.clone();
                    });
                    bounds.push(new_bound);
                },
            }
        }
        all_bounds.push(bounds);
    }


    // for (i, vec) in all_bounds.iter().enumerate() {
    //     for (j, val) in vec.iter().enumerate() {
    //         println!("{}:  {:?}", i + 1, val);
    //     }
    //     println!();
    // }

    let mut sums = all_bounds.iter()
        .flat_map(|bounds| bounds.iter())
        .map(|b| { b.sum.take() })
        .collect::<Vec<_>>();
    sums.sort();

    sums.into_iter().rev().take(3).fold(1, |acc, s| s * acc)
}

#[derive(Debug)]
struct Bound {
    start: usize,
    end: usize,
    sum: Rc<RefCell<usize>>,
    is_sink: bool,
}

impl Bound {
    fn from(start: usize, end: usize, is_sink: bool) -> Self {
        assert!(start <= end, "Bound start is bigger than end.");
        Self {
            start,
            end,
            sum: Rc::new(RefCell::new(end - start + 1)),
            is_sink,
        }
    }

    fn from_bound(start: usize, end: usize, is_sink: bool, old_bound: &mut Bound) -> Self {
        assert!(start <= end, "Bound start is bigger than end.");
        old_bound.sum.replace((end - start + 1) + old_bound.sum.take());
        Self {
            start,
            end,
            sum: old_bound.sum.clone(),
            is_sink,
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

    fn get_ranges(&self) -> Vec<Vec<(usize, usize, bool)>> {
        self.vec.iter().enumerate()
            .map(|(i, vec)| {
                let mut res = Vec::new();
                let mut tmp = None;
                let mut is_sink = false;
                for (j, &v) in vec.iter().enumerate() {
                    if !is_sink {
                        is_sink = self.check_low_point(i, j)
                    }

                    if tmp.is_none() && v != 9 && j == vec.len() - 1 {
                        res.push((j, j, is_sink));
                    } else if tmp.is_none() && v != 9 {
                        tmp = Some(j);
                    } else if tmp.is_some() && v == 9 {
                        res.push((tmp.unwrap(), j - 1, is_sink));
                        tmp = None;
                        is_sink = false;
                    } else if tmp.is_some() && j == vec.len() - 1 {
                        res.push((tmp.unwrap(), j, is_sink));
                    }
                }
                res
            })
            .collect()
    }
}


mod tests {
    #[test]
    fn test1() {
        assert_eq!(2, 1 + 1);
    }
}