use std::collections::HashSet;
use std::fmt::format;
use std::ops::{Range, RangeInclusive};

use peg::parser;

const MAX_DEPTH: usize = 4;

fn main() {
    let input = include_str!("../input_tst.txt");

    println!("First solution: {}", first_solution(&input));
    println!("Second solution: {}", second_solution(&input));
}

fn second_solution(input: &str) -> usize {
    0
}

fn first_solution(input: &str) -> u64 {
    let instructions = input.lines()
        .flat_map(Instruction::parse)
        .collect::<Vec<Instruction>>();

    let mut areas = Vec::new();
    for instruction in instructions.into_iter() {
        if area_is_50(&instruction.area) {
            match instruction.switch {
                Switch::On => add_to_areas(&mut areas, instruction.area),
                Switch::Off => remove_from_areas(&mut areas, &instruction.area),
            }
        }
    }

    areas.iter()
        .map(Area::num_of_cubes)
        .sum()
}

fn add_to_areas(areas: &mut Vec<Area>, mut new_area: Area) {

}

fn remove_from_areas(areas: &mut Vec<Area>, new_area: &Area) {

}

fn area_is_50(area: &Area) -> bool {
    area.x.0 >= -50 && area.x.1 <= 50 &&
    area.y.0 >= -50 && area.y.1 <= 50 &&
    area.z.0 >= -50 && area.z.1 <= 50
}

#[derive(Debug)]
struct Instruction {
    switch: Switch,
    area: Area,
}

#[derive(Debug, Clone)]
struct Area {
    x: (i64, i64),
    y: (i64, i64),
    z: (i64, i64),
}

impl Area {
    fn num_of_cubes(&self) -> u64 {
        self.x.0.abs_diff(self.x.1) * self.y.0.abs_diff(self.y.1) * self.z.0.abs_diff(self.z.1)
    }
}

#[derive(Debug)]
enum Switch {
    On,
    Off,
}

impl Instruction {
    fn parse(line: &str) -> Result<Self, String> {
        peg::parser! {
            grammar parser() for str {
                rule switch_on() -> Switch
                = "on" { Switch::On }

                rule switch_off() -> Switch
                = "off" { Switch::Off }

                rule switch() -> Switch
                = switch_on() / switch_off()

                rule number() -> i64
                = n:$(['-' | '0'..='9']+) { n.parse().unwrap() }

                pub(crate) rule root() -> Instruction
                = s:switch() " x=" xn1:number() ".." xn2:number() ",y=" yn1:number() ".." yn2:number() ",z=" zn1:number() ".." zn2:number() {
                    Instruction {
                        switch: s,
                        area: Area {
                            x: (xn1, xn2),
                            y: (yn1, yn2),
                            z: (zn1, zn2),
                        }
                    }
                }
            }
        }
        parser::root(line)
            .map_err(|e| format!("Cannot parse line ({}), because of error: {}", line, e.to_string()))
    }
}

