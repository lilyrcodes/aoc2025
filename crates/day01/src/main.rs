use std::fs::read_to_string;

fn parse_line_part1(line: &str) -> i8 {
    let value = line[1..].parse::<i32>().unwrap();
    if line.starts_with('L') {
        -(value % 100) as i8
    } else {
        (value % 100) as i8
    }
}

fn parse_input_part1(input: &str) -> Vec<i8> {
    input.lines().map(parse_line_part1).collect()
}

fn next_position(cur: u8, rotation: i8) -> u8 {
    let next: i16 = cur as i16 + rotation as i16;
    if next < 0 {
        (next + 100) as u8
    } else if next >= 100 {
        (next - 100) as u8
    } else {
        next as u8
    }
}

fn part1(input: &str) -> u32 {
    let rotations = parse_input_part1(input);
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

fn main() {
    let input = read_to_string("input.txt").unwrap();
    let p1_answer = part1(&input);
    println!("day 1 part 1: {}", p1_answer);
}

#[cfg(test)]
mod tests {
    use crate::part1;

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
}
