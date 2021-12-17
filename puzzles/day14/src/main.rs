use std::collections::{HashMap, VecDeque};

fn main() {
    let input = include_str!("../input.txt");

    // println!("First solution: {}", first_solution(&input));
    println!("Second solution: {}", second_solution(&input));
}

fn second_solution(input: &str) -> usize {
    let mut polymer = Polymer::from(input);
    let mut polymer2 = Polymer::from(input);

    let mut pairs: HashMap<(char, char), usize> = HashMap::new();
    for win in polymer.template.windows(2) {
        pairs.entry((win[0], win[1]))
            .and_modify(|x| *x += 1)
            .or_insert(1);
    }

    let mut results: HashMap<char, usize> = HashMap::new();
    for c in polymer.template.iter() {
        results.entry(*c)
            .and_modify(|x| *x += 1)
            .or_insert(1);
    }

    (0..40).for_each(|_| {
        let mut new_pairs: HashMap<(char, char), usize> = HashMap::new();
        pairs.iter().for_each(|((c1, c2), n)| {
            let x = polymer.rules.get(&(*c1, *c2)).unwrap();
            add_to_map(&mut new_pairs, (*c1, *x), *n);
            add_to_map(&mut new_pairs, (*x, *c2), *n);
            results.entry(*x)
                .and_modify(|val| *val += *n)
                .or_insert(*n);
        });

        pairs = new_pairs;

        let min = results.values().min().unwrap();
        let max = results.values().max().unwrap();
        println!("{:?}", pairs);
        println!("{:?}", results);
        println!("fast result: {}", *max - *min);
    });

    0
}

fn to_char_map(pairs: &HashMap<(char, char), usize>) -> HashMap<char, usize> {
    let mut res = HashMap::new();
    pairs.iter().for_each(|((c1, c2), n)| {
        res.entry(*c1)
            .and_modify(|x| *x += *n)
            .or_insert(*n);
        res.entry(*c2)
            .and_modify(|x| *x += *n)
            .or_insert(*n);
    });
    res
}

fn add_to_map(map: &mut HashMap<(char, char), usize>, key: (char, char), n: usize) {
    map.entry(key).and_modify(|x| *x += n).or_insert(n);
}

fn first_solution(input: &str) -> usize {
    let mut polymer = Polymer::from(input);
    polymer.make_n_steps(10);
    polymer.count_max_min_diff()
}

#[derive(Debug)]
struct Polymer {
    template: Vec<char>,
    rules: HashMap<(char, char), char>,
    occurrences: HashMap<char, usize>,
}

impl Polymer {
    fn from(str: &str) -> Self {
        let mut lines = str.lines();
        let template = lines.next().unwrap().chars().collect::<Vec<_>>();

        let mut occurrences = HashMap::new();
        for c in template.iter() {
            occurrences.entry(*c)
                .and_modify(|val| *val += 1)
                .or_insert(1);
        }

        lines.next();
        let mut rules = HashMap::new();
        for line in lines {
            let (first_part, second_part) = line.split_once(" -> ").unwrap_or_else(|| panic!("Cannot parse line: {}", line));
            let mut first_part = first_part.chars();
            let key = (first_part.next().unwrap(), first_part.next().unwrap());
            let value = second_part.chars().next().unwrap();
            rules.insert(key, value);
        }

        Self {
            template,
            rules,
            occurrences
        }
    }

    fn print_template(&self) {
        let mut buff = String::with_capacity(self.template.len());
        for c in self.template.iter() {
            buff.push(*c);
        }

        let max_min_diff = self.count_max_min_diff();
        println!("{}|  {}", buff.len(), buff);
        println!("{:?}, {} - {} = {}", self.occurrences, self.occurrences.values().max().unwrap(), self.occurrences.values().min().unwrap(), max_min_diff);
        println!("{} / {} = {}", buff.len(), max_min_diff, buff.len() as f64 / max_min_diff as f64);
    }

    fn make_step(&mut self) {
        let mut insertions: Vec<(usize, char)> = Vec::new();
        for (i, win) in self.template.windows(2).enumerate() {
            let key = (win[0], win[1]);
            if let Some(val) = self.rules.get(&key) {
                insertions.push((i + 1, *val));
            }
        }


        for (i, (index, c)) in insertions.into_iter().enumerate() {
            self.occurrences.entry(c)
                .and_modify(|val| *val += 1)
                .or_insert(1);
            self.template.insert(index + i, c);
        }
    }

    fn make_n_steps(&mut self, n: usize) {
        (0..n).for_each(|i| self.make_step())
    }

    fn count_max_min_diff(&self) -> usize {
        let max = self.occurrences.values().max().unwrap();
        let min = self.occurrences.values().min().unwrap();

        *max - *min
    }
}

mod tests {
    #[test]
    fn test1() {
        assert_eq!(2, 1 + 1);
    }
}