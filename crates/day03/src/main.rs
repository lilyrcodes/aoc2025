use std::fs::read_to_string;

fn parse_input(input: &str) -> Vec<Vec<u64>> {
    input
        .trim()
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as u64)
                .collect()
        })
        .collect()
}

fn get_largest_joltage(line: &[u64]) -> u64 {
    let mut largest = 0;
    for (i, left) in line.iter().enumerate() {
        for right in line[i + 1..].iter() {
            let num = left * 10 + right;
            if num > largest {
                largest = num;
            }
        }
    }
    largest
}

fn part1(input: &str) -> u64 {
    let parsed = parse_input(input);
    parsed.iter().map(|line| get_largest_joltage(&line)).sum()
}

fn digits_to_num(digits: &[u64]) -> u64 {
    let mut num = 0;
    for d in digits {
        num *= 10;
        num += d;
    }
    num
}

fn get_largest_with_one_removed(num: &[u64]) -> Vec<u64> {
    let mut largest = vec![];
    let mut largest_value = 0;
    for i in 0..num.len() {
        let digit_removed: Vec<u64> = num[0..i]
            .iter()
            .copied()
            .chain(num[i + 1..].iter().copied())
            .collect();
        let as_num = digits_to_num(&digit_removed);
        if as_num > largest_value {
            largest_value = as_num;
            largest = digit_removed;
        }
    }
    largest
}

fn step(digit_to_add: u64, cur_largest: &mut Vec<u64>) {
    if digit_to_add >= cur_largest[0] {
        let mut rest = get_largest_with_one_removed(cur_largest);
        cur_largest.clear();
        cur_largest.push(digit_to_add);
        cur_largest.append(&mut rest);
    }
}

fn get_largest_joltage2(line: &[u64]) -> u64 {
    let mut cur_largest: Vec<u64> = line[line.len() - 12..].iter().copied().collect();
    for i in (0..line.len() - 12).rev() {
        let digit = line[i];
        step(digit, &mut cur_largest);
    }
    digits_to_num(&cur_largest)
}

fn part2(input: &str) -> u64 {
    let parsed = parse_input(input);
    parsed.iter().map(|line| get_largest_joltage2(&line)).sum()
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
    use crate::{get_largest_joltage, get_largest_joltage2, part1, part2};
    const BASIC_INPUT: &str = "987654321111111
811111111111119
234234234234278
818181911112111";

    #[test]
    fn get_largest_joltage_works() {
        assert_eq!(get_largest_joltage(&[9, 8, 7, 6, 5, 4, 3, 2, 1, 1]), 98);
        assert_eq!(get_largest_joltage(&[8, 1, 1, 1, 1, 9]), 89);
        assert_eq!(get_largest_joltage(&[2, 3, 4, 2, 3, 4, 2, 7, 8]), 78);
        assert_eq!(get_largest_joltage(&[8, 1, 8, 1, 9, 1, 1, 2, 1]), 92);
    }

    #[test]
    fn basic_test_part1() {
        assert_eq!(part1(BASIC_INPUT), 357)
    }

    #[test]
    fn get_largest_joltage2_works() {
        assert_eq!(
            get_largest_joltage2(&[9, 8, 7, 6, 5, 4, 3, 2, 1, 1, 1, 1, 1, 1, 1]),
            987654321111
        );
        assert_eq!(
            get_largest_joltage2(&[8, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 9]),
            811111111119
        );
        assert_eq!(
            get_largest_joltage2(&[2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 7, 8]),
            434234234278
        );
        assert_eq!(
            get_largest_joltage2(&[8, 1, 8, 1, 8, 1, 9, 1, 1, 1, 1, 2, 1, 1, 1]),
            888911112111
        );
    }

    #[test]
    fn basic_test_part2() {
        assert_eq!(part2(BASIC_INPUT), 3121910778619)
    }
}
