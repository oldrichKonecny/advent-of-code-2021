use std::collections::HashMap;

fn main() {
    let polymer = Polymer::from(&include_str!("../input.txt"));

    println!("First solution: {}", first_solution(&polymer));
    println!("Second solution: {}", second_solution(&polymer));
}

fn second_solution(polymer: &Polymer) -> usize {
    count_max_min_diff(&polymer.occurrences_map_in_step(40))
}

fn first_solution(polymer: &Polymer) -> usize {
    count_max_min_diff(&polymer.occurrences_map_in_step(10))
}

fn count_max_min_diff(occurrences: &HashMap<char, usize>) -> usize {
    let max = occurrences.values().max().unwrap();
    let min = occurrences.values().min().unwrap();

    *max - *min
}

#[derive(Debug)]
struct Polymer {
    template: Vec<char>,
    rules: HashMap<(char, char), char>,
}

impl Polymer {
    fn from(str: &str) -> Self {
        let mut lines = str.lines();
        let template = lines.next().unwrap().chars().collect::<Vec<_>>();

        lines.next();

        let mut rules = HashMap::new();
        for line in lines {
            let (first_part, second_part) = line
                .split_once(" -> ")
                .unwrap_or_else(|| panic!("Cannot parse line: {}", line));
            let mut first_part = first_part.chars();
            let key = (first_part.next().unwrap(), first_part.next().unwrap());
            let value = second_part.chars().next().unwrap();
            rules.insert(key, value);
        }

        Self { template, rules }
    }

    fn occurrences_map_in_step(&self, step: usize) -> HashMap<char, usize> {
        let mut pairs: HashMap<(char, char), usize> = HashMap::new();
        for win in self.template.windows(2) {
            pairs
                .entry((win[0], win[1]))
                .and_modify(|x| *x += 1)
                .or_insert(1);
        }

        let mut results: HashMap<char, usize> = HashMap::new();
        for c in self.template.iter() {
            results.entry(*c).and_modify(|x| *x += 1).or_insert(1);
        }

        fn add_to_map(map: &mut HashMap<(char, char), usize>, key: (char, char), n: usize) {
            map.entry(key).and_modify(|x| *x += n).or_insert(n);
        }

        for _ in 0..step {
            let mut new_pairs: HashMap<(char, char), usize> = HashMap::new();
            pairs.iter().for_each(|((c1, c2), n)| {
                let x = self.rules.get(&(*c1, *c2)).unwrap();
                add_to_map(&mut new_pairs, (*c1, *x), *n);
                add_to_map(&mut new_pairs, (*x, *c2), *n);
                results.entry(*x).and_modify(|val| *val += *n).or_insert(*n);
            });
            pairs = new_pairs;
        }

        results
    }
}

mod tests {
    #[test]
    fn test1() {
        assert_eq!(2, 1 + 1);
    }
}
