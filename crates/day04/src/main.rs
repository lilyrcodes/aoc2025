use std::fs::read_to_string;

fn parse_input(input: &str) -> Vec<Vec<bool>> {
    input
        .lines()
        .map(|line| line.chars().map(|c| c != '.').collect())
        .collect()
}

fn is_blocked(x: isize, y: isize, buf: &[Vec<bool>]) -> bool {
    if x < 0 || y < 0 || x >= buf[0].len() as isize || y >= buf.len() as isize {
        false
    } else {
        buf[y as usize][x as usize]
    }
}

fn count_empty(x: isize, y: isize, buf: &[Vec<bool>]) -> usize {
    let mut count = 0;
    if !is_blocked(x - 1, y - 1, buf) {
        count += 1;
    }
    if !is_blocked(x, y - 1, buf) {
        count += 1;
    }
    if !is_blocked(x + 1, y - 1, buf) {
        count += 1;
    }
    if !is_blocked(x - 1, y, buf) {
        count += 1;
    }
    if !is_blocked(x + 1, y, buf) {
        count += 1;
    }
    if !is_blocked(x - 1, y + 1, buf) {
        count += 1;
    }
    if !is_blocked(x, y + 1, buf) {
        count += 1;
    }
    if !is_blocked(x + 1, y + 1, buf) {
        count += 1;
    }
    count
}

fn step(buf: &Vec<Vec<bool>>) -> (Vec<Vec<bool>>, u64) {
    let mut count = 0;
    let mut next = buf.clone();
    for y in 0..buf.len() {
        for x in 0..buf[0].len() {
            if buf[y][x] && count_empty(x as isize, y as isize, &buf) >= 5 {
                next[y][x] = false;
                count += 1;
            }
        }
    }
    (next, count)
}

fn part1(input: &str) -> u64 {
    let buf = parse_input(input);
    let (_, count) = step(&buf);
    count
}

fn part2(input: &str) -> u64 {
    let mut buf = parse_input(input);
    let mut count = 0;
    loop {
        let (next, removed) = step(&buf);
        if removed == 0 {
            break;
        }
        count += removed;
        buf = next;
    }
    count
}

fn main() {
    let input = read_to_string("input.txt").unwrap();
    let input = input.trim();
    let p1_answer = part1(&input);
    println!("day 1 part 1: {}", p1_answer);
    let p2_answer = part2(&input);
    println!("day 1 part 2: {}", p2_answer);
}

#[cfg(test)]
mod tests {
    use crate::{part1, part2};
    const BASIC_INPUT: &str = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";

    #[test]
    fn basic_test_part1() {
        assert_eq!(part1(BASIC_INPUT), 13)
    }

    #[test]
    fn basic_test_part2() {
        assert_eq!(part2(BASIC_INPUT), 43)
    }
}
