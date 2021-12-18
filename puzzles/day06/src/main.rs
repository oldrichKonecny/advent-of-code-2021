fn main() {
    let input = include_str!("../input.txt");

    println!("First solution: {}", first_solution(&input));
    println!("Second solution: {}", second_solution(&input));
}

fn second_solution(input: &str) -> u64 {
    let mut population = Population::from(&input, 9, 6);
    population.make_n_steps(256);
    population.sum_population()
}

fn first_solution(input: &str) -> u64 {
    let mut population = Population::from(&input, 9, 6);
    population.make_n_steps(80);
    population.sum_population()
}

struct Population {
    cycle: Vec<u64>,
    cycle_len: usize,
    zero_marker: usize,
    adult_fish_reset: usize,
}

impl Population {
    fn from(str: &str, cycle_len: usize, adult_fish_reset: usize) -> Self {
        let input = str
            .split(',')
            .filter_map(|s| s.parse::<usize>().ok())
            .collect::<Vec<_>>();
        let mut cycle = vec![0u64; cycle_len];
        for i in input {
            cycle[i] += 1;
        }

        Self {
            cycle,
            cycle_len,
            zero_marker: 0,
            adult_fish_reset,
        }
    }

    fn make_step(&mut self) {
        let adult_index = (self.zero_marker + self.adult_fish_reset + 1) % self.cycle_len;
        self.cycle[adult_index] += self.cycle[self.zero_marker];
        self.zero_marker = (self.zero_marker + 1) % self.cycle_len;
    }

    fn make_n_steps(&mut self, n: u32) {
        (0..n).for_each(|_| self.make_step());
    }

    fn sum_population(&self) -> u64 {
        self.cycle.iter().sum()
    }
}

mod tests {
    #[test]
    fn test1() {
        assert_eq!(2, 1 + 1);
    }
}
