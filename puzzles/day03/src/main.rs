fn main() {
    let input = include_str!("../input.txt");

    println!("First solution: {}", first_solution(&input));
    println!("Second solution: {}", second_solution(&input));
}

fn second_solution(input: &str) -> u64 {
    let input = input
        .lines()
        .map(|s| {
            s.bytes()
                .map(|b| if b == b'1' { true } else { false })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let more = determine_most(&input, 0, true);

    let mut one_vec = Vec::new();
    let mut zero_vec = Vec::new();
    for i in input.into_iter() {
        if i[0] == true {
            one_vec.push(i);
        } else {
            zero_vec.push(i);
        }
    }
    let (o2_vec, co2_vec) = if more {
        (&mut one_vec, &mut zero_vec)
    } else {
        (&mut zero_vec, &mut one_vec)
    };

    let mut i = 1;
    while o2_vec.len() > 1 {
        let retain = determine_most(&o2_vec, i, true);
        o2_vec.retain(|v| v[i] == retain);
        i += 1;
    }

    let mut i = 1;
    while co2_vec.len() > 1 {
        let retain = determine_most(&co2_vec, i, false);
        co2_vec.retain(|v| v[i] == retain);
        i += 1;
    }

    let o2 = convert_to_decimal(&o2_vec[0]);
    let co2 = convert_to_decimal(&co2_vec[0]);

    o2 * co2
}

fn determine_most(input: &[Vec<bool>], index: usize, most_common: bool) -> bool {
    let one = input.iter().filter(|v| v[index] == true).count();

    let zero = input.len() - one;
    let res = if most_common { one >= zero } else { zero > one };
    // println!("determine most: one={}, zero={}, most_common={}, index={}, input_len={}, return={}", one, zero, most_common, index, input.len(), res);
    res
}

fn first_solution(input: &str) -> u32 {
    let mut lines = input.lines();
    let mut counter = BitCounter::from(lines.next().unwrap());
    for line in lines {
        counter.add(line);
    }

    let (gamma, epsilon) = counter.get_decimal_numbers();
    gamma * epsilon
}

fn convert_to_decimal(input: &[bool]) -> u64 {
    let decimal_str = input
        .iter()
        .map(|b| if *b == true { '1' } else { '0' })
        .collect::<String>();
    u64::from_str_radix(&decimal_str, 2).unwrap()
}

struct BitCounter {
    vec: Vec<u32>,
    full_count: u32,
}

impl BitCounter {
    fn from(str: &str) -> Self {
        let bytes = str.as_bytes();
        let mut vec = vec![0; bytes.len()];
        for (i, b) in bytes.iter().enumerate() {
            if *b == b'1' {
                vec[i] += 1;
            }
        }
        Self { vec, full_count: 1 }
    }

    fn add(&mut self, str: &str) {
        let bytes = str.as_bytes();
        for (i, b) in bytes.iter().enumerate() {
            if *b == b'1' {
                self.vec[i] += 1;
            }
        }
        self.full_count += 1;
    }

    fn get_decimal_numbers(&self) -> (u32, u32) {
        let half = self.full_count / 2;
        let gamma = self
            .vec
            .iter()
            .map(|i| if *i > half { true } else { false })
            .collect::<Vec<_>>();
        let epsilon = gamma.iter().map(|b| !b).collect::<Vec<_>>();

        let gamma = gamma
            .iter()
            .map(|b| if *b { '1' } else { '0' })
            .collect::<String>();
        let epsilon = epsilon
            .iter()
            .map(|b| if *b { '1' } else { '0' })
            .collect::<String>();

        let gamma = u32::from_str_radix(&gamma, 2).unwrap();
        let epsilon = u32::from_str_radix(&epsilon, 2).unwrap();

        (gamma, epsilon)
    }
}

mod tests {
    #[test]
    fn test1() {
        assert_eq!(2, 1 + 1);
    }
}
