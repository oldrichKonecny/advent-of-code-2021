fn main() {
    let input = include_str!("../input.txt");

    println!("First solution: {}", first_solution(&input));
    println!("Second solution: {}", second_solution(&input));
}

fn second_solution(input: &str) -> usize {
    let bits = parse_input(input);
    let packet = Packet::parse(&bits).unwrap().0;
    packet.compute_value()
}

fn first_solution(input: &str) -> u32 {
    let bits = parse_input(input);
    let packet = Packet::parse(&bits).unwrap().0;
    packet.sum_versions()
}

#[derive(Debug)]
enum Packet {
    Value {
        version: u8,
        type_id: u8,
        value: usize,
    },
    Operator {
        version: u8,
        type_id: u8,
        length_id: bool,
        sub_packets: Vec<Packet>,
    },
}

impl Packet {
    fn parse(bits: &[bool]) -> Option<(Self, usize)> {
        if bits.len() < 11 {
            return None;
        }
        let version = u8_from(&bits[0..3]);
        let type_id = u8_from(&bits[3..6]);
        if type_id == 4 {
            let mut value = Vec::new();
            let mut value_size = 6;
            for chunk in bits[6..].chunks(5) {
                if chunk.len() < 5 {
                    break;
                }
                value_size += 5;
                for b in &chunk[1..] {
                    value.push(*b);
                }
                if chunk[0] == false {
                    break;
                }
            }
            let value = usize_from(&value);

            Some((Self::Value {
                version,
                type_id,
                value
            }, value_size))
        } else {
            let mut sub_packets = Vec::new();
            let length_id = bits[6];
            let mut value_size = 7;
            if length_id {
                let num_of_sub_packets = usize_from(&bits[7..18]);
                let mut sub_bits = &bits[18..];
                value_size += 11;
                for _ in 0..num_of_sub_packets {
                    if let Some((packet, size)) = Packet::parse(sub_bits) {
                        sub_packets.push(packet);
                        sub_bits = &sub_bits[size..];
                        value_size += size;
                    }
                }
            } else {
                let bits_length = usize_from(&bits[7..22]);
                let mut sub_bits = &bits[22..22 + bits_length];
                value_size += 15;
                while let Some((packet, size)) = Packet::parse(sub_bits) {
                    sub_packets.push(packet);
                    sub_bits = &sub_bits[size..];
                    value_size += size;
                }
            }

            Some((Self::Operator {
                version,
                type_id,
                length_id,
                sub_packets,
            }, value_size))
        }
    }

    fn compute_value(&self) -> usize {
        match self {
            Packet::Value { value, .. } => *value,
            Packet::Operator { type_id, sub_packets, .. } => match type_id {
                0 => sub_packets.iter().map(|p| p.compute_value()).sum(),
                1 => sub_packets.iter().map(|p| p.compute_value()).product(),
                2 => sub_packets.iter().map(|p| p.compute_value()).min().unwrap(),
                3 => sub_packets.iter().map(|p| p.compute_value()).max().unwrap(),
                5 => if sub_packets[0].compute_value() > sub_packets[1].compute_value() { 1 } else { 0 },
                6 => if sub_packets[0].compute_value() < sub_packets[1].compute_value() { 1 } else { 0 },
                7 => if sub_packets[0].compute_value() == sub_packets[1].compute_value() { 1 } else { 0 },
                id => panic!("Invalid type_id: {}", id),
            },
        }
    }

    fn sum_versions(&self) -> u32 {
        match self {
            Packet::Value { version, .. } => *version as u32,
            Packet::Operator { version, sub_packets, .. } => {
                let inner_sum: u32 = sub_packets.iter()
                    .map(|p| p.sum_versions() as u32)
                    .sum();
                inner_sum + *version as u32
            },
        }
    }
}

fn u8_from(bits: &[bool]) -> u8 {
    let str = bits.iter()
        .map(|b| if *b { "1" } else { "0" })
        .collect::<String>();
    u8::from_str_radix(&str, 2).unwrap()
}

fn usize_from(bits: &[bool]) -> usize {
    let str = bits.iter()
        .map(|b| if *b { "1" } else { "0" })
        .collect::<String>();
    usize::from_str_radix(&str, 2).unwrap()
}

fn parse_input(input: &str) -> Vec<bool> {
    input.chars()
        .flat_map(|c| match c {
            '0' => vec![false, false, false, false],
            '1' => vec![false, false, false, true],
            '2' => vec![false, false, true, false],
            '3' => vec![false, false, true, true],
            '4' => vec![false, true, false, false],
            '5' => vec![false, true, false, true],
            '6' => vec![false, true, true, false],
            '7' => vec![false, true, true, true],
            '8' => vec![true, false, false, false],
            '9' => vec![true, false, false, true],
            'A' => vec![true, false, true, false],
            'B' => vec![true, false, true, true],
            'C' => vec![true, true, false, false],
            'D' => vec![true, true, false, true],
            'E' => vec![true, true, true, false],
            'F' => vec![true, true, true, true],
            x => panic!("Unexpected value: {} in input", x),
        })
        .collect()
}

fn print_bits(slice: &[bool]) {
    for b in slice {
        print!("{}", if *b == true { 1 } else { 0 });
    }
    println!();
}

mod tests {
    #[test]
    fn test1() {
        assert_eq!(2, 1 + 1);
    }
}


