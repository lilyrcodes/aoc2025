use std::fs::read_to_string;

fn parse_line(line: &str) -> i64 {
    let value = line[1..].parse::<i64>().unwrap();
    if line.starts_with('L') { -value } else { value }
}

fn parse_input(input: &str) -> Vec<i64> {
    input.lines().map(parse_line).collect()
}

fn next_position(cur: i64, rotation: i64) -> i64 {
    let next = (cur + rotation) % 100;
    if next < 0 { next + 100 } else { next }
}

fn passes_zero_count(cur: i64, rotation: i64) -> i64 {
    let passes = ((cur + rotation) / 100).abs();
    if cur + rotation <= 0 && cur != 0 {
        passes + 1
    } else {
        passes
    }
}

fn part1(input: &str) -> i64 {
    let rotations = parse_input(input);
    let mut position = 50;
    let mut answer = 0;
    for rotation in rotations {
        position = next_position(position, rotation);
        if position == 0 {
            answer += 1;
        }
    }
    answer
}

fn part2(input: &str) -> i64 {
    let rotations = parse_input(input);
    let mut position = 50;
    let mut answer = 0;
    for rotation in rotations {
        answer += passes_zero_count(position, rotation);
        position = next_position(position, rotation);
    }
    answer
}

fn main() {
    let input = read_to_string("input.txt").unwrap();
    let p1_answer = part1(&input);
    println!("day 1 part 1: {}", p1_answer);
    let p2_answer = part2(&input);
    println!("day 1 part 2: {}", p2_answer);
}

#[cfg(test)]
mod tests {
    use crate::{part1, part2};

    #[test]
    fn basic_test_part1() {
        let basic_input = "L68
L30
R48
L5
R60
L55
L1
L99
R114
L182";
        let final_position = part1(basic_input);
        assert_eq!(final_position, 3);
    }

    #[test]
    fn basic_test_part2() {
        let basic_input = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";
        let count = part2(basic_input);
        assert_eq!(count, 6);
    }
}
