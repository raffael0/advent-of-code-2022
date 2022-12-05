use crate::common::to_num;
use std::borrow::BorrowMut;
use std::str::FromStr;

pub fn day5() {
    println!("result : {}", day5b(include_str!("../inputs/day5")));
}

/*
        [J] [N] [D] [F] [J] [H] [B]
    [Q] [F] [W] [S] [V] [N] [F] [N]
[W] [N] [H] [M] [L] [B] [R] [T] [Q]
[L] [T] [C] [R] [R] [J] [W] [Z] [L]
[S] [J] [S] [T] [T] [M] [D] [B] [H]
 1   2   3   4   5   6   7   8   9
 */
fn parse_field(input: &str) -> Vec<Vec<char>> {
    let mut vector: Vec<Vec<char>> = Vec::new();

    let a = input
        .lines()
        .filter(|line| !line.contains('1'))
        .map(|line| {
            line.chars()
                .collect::<Vec<char>>()
                .chunks(4)
                .map(|chunk| {
                    if !chunk.contains(&'[') {
                        char::from_str("_").unwrap()
                    } else {
                        chunk[1]
                    }
                })
                .collect::<Vec<char>>()
        })
        .collect::<Vec<Vec<char>>>();
    vector.resize(a.len() + 1, Vec::new());

    for height in 0..a.len() {
        for length in 0..a.iter().map(|a| a.len()).max().unwrap() {
            println!("{height}");
            if a.get(height).unwrap().get(length).is_some()
                && *a.get(height).unwrap().get(length).unwrap() != '_'
            {
                vector[length].insert(0, *a.get(height).unwrap().get(length).unwrap())
            }
        }
    }
    return vector;
}

fn day5a(input: &str) -> String {
    let (mut field, instructions) = input
        .split_once("\n\n")
        .map(|inp| (parse_field(inp.0), parse_instructions(inp.1)))
        .unwrap();
    println!("{:?}", instructions);

    for instruction in instructions.iter() {
        // pop x values from pos 1
        let len = field[instruction.1 - 1].len();
        let mut values = field[instruction.1 - 1].split_off(len - instruction.0);
        values.reverse();
        field[instruction.2 - 1].extend_from_slice(values.as_slice());
        println!("{:?}", instruction);
        println!("{:?}", field);
    }
    let output: String = field
        .iter()
        .map(|stack| stack.last().unwrap_or(&'0'))
        .filter(|letter| **letter != '0')
        .collect();
    return output;
}
fn day5b(input: &str) -> String {
    let (mut field, instructions) = input
        .split_once("\n\n")
        .map(|inp| (parse_field(inp.0), parse_instructions(inp.1)))
        .unwrap();
    println!("{:?}", instructions);

    for instruction in instructions.iter() {
        // pop x values from pos 1
        let len = field[instruction.1 - 1].len();
        let mut values = field[instruction.1 - 1].split_off(len - instruction.0);
        field[instruction.2 - 1].extend_from_slice(values.as_slice());
        println!("{:?}", instruction);
        println!("{:?}", field);
    }
    let output: String = field
        .iter()
        .map(|stack| stack.last().unwrap_or(&'0'))
        .filter(|letter| **letter != '0')
        .collect();
    return output;
}

fn parse_instructions(input: &str) -> Vec<(usize, usize, usize)> {
    input
        .lines()
        .map(|line| {
            let chars = line.split(" ").collect::<Vec<&str>>();
            return (to_num(chars[1]), to_num(chars[3]), to_num(chars[5]));
        })
        .collect()
}
