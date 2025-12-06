use std::{fs::read_to_string, str::FromStr};

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
enum Operator {
    ADD,
    MULT,
}

impl Operator {
    fn add(acc: u64, operand: u64) -> u64 {
        acc + operand
    }

    fn mult(acc: u64, operand: u64) -> u64 {
        acc * operand
    }

    fn fold_fn(&self) -> impl Fn(u64, u64) -> u64 {
        match self {
            Self::ADD => Self::add,
            Self::MULT => Self::mult,
        }
    }

    fn acc_init(&self) -> u64 {
        match self {
            Self::ADD => 0,
            Self::MULT => 1,
        }
    }
}

impl FromStr for Operator {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "+" {
            Ok(Operator::ADD)
        } else {
            Ok(Operator::MULT)
        }
    }
}

#[derive(Debug)]
struct Input {
    nums: Vec<Vec<u64>>,
    ops: Vec<Operator>,
}

fn parse_input_part1(input: &str) -> Input {
    let num_lines = input.lines().count();
    let mut nums: Vec<Vec<u64>> = Vec::with_capacity(num_lines - 1);
    for (i, line) in input.lines().enumerate() {
        if i == num_lines - 1 {
            break;
        }
        let line = line
            .split_whitespace()
            .map(|num| num.parse::<u64>().unwrap())
            .collect();
        nums.push(line);
    }
    let ops = input
        .lines()
        .last()
        .unwrap()
        .split_whitespace()
        .map(|s| s.parse::<Operator>().unwrap())
        .collect();
    Input { nums, ops }
}

fn part1(input: &str) -> u64 {
    let input = parse_input_part1(input);
    let mut total = 0;
    for (col, op) in input.ops.iter().enumerate() {
        let fold_fn = op.fold_fn();
        let start_val = op.acc_init();
        let col_result = input
            .nums
            .iter()
            .map(|row| row[col])
            .fold(start_val, fold_fn);
        total += col_result;
    }
    total
}

struct Part2Problem {
    nums: Vec<u64>,
    op: Operator,
}

fn parse_digits(digits: &[u8]) -> Option<u64> {
    if digits.iter().all(|d| *d == b' ') {
        None
    } else {
        Some(
            digits
                .iter()
                .filter(|d| **d != b' ')
                .fold(0, |acc, d| acc * 10 + (d - b'0') as u64),
        )
    }
}

fn get_numbers(input: &str) -> Vec<Option<u64>> {
    let mut lines: Vec<&[u8]> = input
        .lines()
        .map(|line| line.as_bytes())
        .rev()
        .skip(1)
        .collect();
    lines.reverse();
    let mut numbers: Vec<Vec<u8>> = vec![];
    for col in 0..lines[0].len() {
        let cur_num = lines.iter().map(|line| line[col]).collect();
        numbers.push(cur_num);
    }
    numbers
        .into_iter()
        .map(|digits| parse_digits(&digits))
        .collect()
}

fn parse_input_part2(input: &str) -> Vec<Part2Problem> {
    let numbers = get_numbers(input);
    let ops: Vec<Operator> = input
        .lines()
        .last()
        .unwrap()
        .split_whitespace()
        .map(|s| s.parse::<Operator>().unwrap())
        .collect();
    let mut cur_numbers = vec![];
    let mut result = vec![];
    let mut op_idx = 0;
    for number in numbers {
        if let Some(num) = number {
            cur_numbers.push(num);
        } else {
            result.push(Part2Problem {
                nums: cur_numbers,
                op: ops[op_idx],
            });
            cur_numbers = vec![];
            op_idx += 1;
        }
    }
    result.push(Part2Problem {
        nums: cur_numbers,
        op: ops[op_idx],
    });
    result
}

fn part2(input: &str) -> u64 {
    let input = parse_input_part2(input);
    input
        .into_iter()
        .map(|prob| {
            prob.nums
                .into_iter()
                .fold(prob.op.acc_init(), prob.op.fold_fn())
        })
        .sum()
}

fn main() {
    let input = read_to_string("input.txt").unwrap();
    let p1_answer = part1(&input);
    assert_ne!(p1_answer, 5171058832173);
    assert!(p1_answer > 5171058832173);
    println!("day 1 part 1: {}", p1_answer);
    let p2_answer = part2(&input);
    println!("day 1 part 2: {}", p2_answer);
}

#[cfg(test)]
mod tests {
    use crate::{part1, part2};
    const BASIC_INPUT: &str = "123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  ";
    const DERIVED_INPUT: &str = "  6  8 99  
187 49 5422
635 82 2573
552 33 5463
*   *  +   ";

    #[test]
    fn basic_test_part1() {
        assert_eq!(part1(BASIC_INPUT), 4277556);
        assert_eq!(part1(DERIVED_INPUT), 394357749);
    }

    #[test]
    fn basic_test_part2() {
        assert_eq!(part2(BASIC_INPUT), 3263827);
    }
}
