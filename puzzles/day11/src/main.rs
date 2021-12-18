fn main() {
    let input = include_str!("../input.txt");

    println!("First solution: {}", first_solution(&input));
    println!("Second solution: {}", second_solution(&input));
}

fn second_solution(input: &str) -> u32 {
    let mut mat = Matrix::from(input);
    let mut counter = 0;
    loop {
        counter += 1;
        if make_step(&mut mat) >= 100 {
            break;
        }
    }
    counter
}

fn first_solution(input: &str) -> u32 {
    let mut mat = Matrix::from(input);
    make_n_steps(&mut mat, 100)
}

fn increment(mat: &mut Matrix, row: isize, col: isize) {
    assert!(row <= mat.row as isize);
    assert!(col <= mat.col as isize);

    if mat.inc(row as usize, col as usize) {
        if row > 0 {
            increment(mat, row - 1, col);
            if col > 0 {
                increment(mat, row - 1, col - 1);
            }
            if col < mat.col as isize {
                increment(mat, row - 1, col + 1);
            }
        }
        if row < mat.row as isize {
            increment(mat, row + 1, col);
            if col > 0 {
                increment(mat, row + 1, col - 1);
            }
            if col < mat.col as isize {
                increment(mat, row + 1, col + 1);
            }
        }
        if col > 0 {
            increment(mat, row, col - 1);
        }
        if col < mat.col as isize {
            increment(mat, row, col + 1);
        }
    }
}

fn make_step(mat: &mut Matrix) -> u32 {
    for i in 0..=mat.row {
        for j in 0..=mat.col {
            increment(mat, i as isize, j as isize);
        }
    }
    mat.zero_and_count_flashed()
}

fn make_n_steps(mat: &mut Matrix, n: usize) -> u32 {
    (0..n).map(|_| make_step(mat)).sum()
}

#[derive(Debug)]
struct Matrix {
    vec: Vec<u32>,
    col: usize,
    row: usize,
}

impl Matrix {
    fn from(str: &str) -> Self {
        let mut vec = Vec::new();
        let mut col = 0;
        let mut row = 0;
        for (i, line) in str.lines().enumerate() {
            for (j, b) in line.bytes().enumerate() {
                vec.push(b as u32 - 48);
                if j > col {
                    col = j;
                }
            }
            if row < i {
                row = i;
            }
        }

        Self { vec, col, row }
    }

    fn inc(&mut self, row: usize, col: usize) -> bool {
        let x = &mut self.vec[row * (self.col + 1) + col];
        *x += 1;
        if *x == 10 {
            true
        } else {
            false
        }
    }

    fn zero_and_count_flashed(&mut self) -> u32 {
        self.vec.iter_mut().fold(0, |acc, x| {
            if *x > 9 {
                *x = 0;
                acc + 1
            } else {
                acc
            }
        })
    }
}

mod tests {
    #[test]
    fn test1() {
        assert_eq!(2, 1 + 1);
    }
}
