use std::fs::read_to_string;
use std::ops::RangeInclusive;

fn parse_range(r: &str) -> RangeInclusive<u64> {
    let mut split = r.split('-');
    let first = split.next().unwrap();
    let first = first.parse().unwrap();
    let last = split.next().unwrap();
    let last = last.parse().unwrap();
    first..=last
}

fn parse_input(input: &str) -> Vec<RangeInclusive<u64>> {
    input.split(',').map(parse_range).collect()
}

fn is_valid_id(id: u64) -> bool {
    let s = id.to_string();
    if s.len() % 2 != 0 {
        true
    } else {
        s[0..s.len() / 2] != s[s.len() / 2..]
    }
}

fn is_repeat_of(s: &str, pat: &str) -> bool {
    let step = pat.len();
    if s.len() % step != 0 {
        return false;
    }
    for i in (0..s.len()).step_by(step) {
        if &s[i..i + step] != pat {
            return false;
        }
    }
    true
}

fn is_valid_id_full(id: u64) -> bool {
    let s = id.to_string();
    for i in 1..=s.len() / 2 {
        let sub = &s[0..i];
        if is_repeat_of(&s, sub) {
            return false;
        }
    }
    true
}

fn part1(input: &str) -> u64 {
    let ranges = parse_input(input);
    let mut sum = 0;
    for r in ranges {
        for id in r {
            if !is_valid_id(id) {
                sum += id;
            }
        }
    }
    sum
}

fn part2(input: &str) -> u64 {
    let ranges = parse_input(input);
    let mut sum = 0;
    for r in ranges {
        for id in r {
            if !is_valid_id_full(id) {
                sum += id;
            }
        }
    }
    sum
}

fn main() {
    let mut input = read_to_string("input.txt").unwrap();
    if let Some(trimmed) = input.strip_suffix('\n') {
        input = trimmed.to_string();
    }
    let p1_answer = part1(&input);
    println!("day 1 part 1: {}", p1_answer);
    let p2_answer = part2(&input);
    println!("day 1 part 2: {}", p2_answer);
}

#[cfg(test)]
mod tests {
    use crate::{is_valid_id, is_valid_id_full, part1, part2};
    const BASIC_INPUT: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

    #[test]
    fn test_is_valid_id() {
        assert!(is_valid_id(12));
        assert!(is_valid_id(121));
        assert!(is_valid_id(121212));
        assert!(is_valid_id(11111112));
        assert!(is_valid_id(111));
        assert!(!is_valid_id(11));
        assert!(!is_valid_id(12341234));
    }

    #[test]
    fn basic_test_part1() {
        assert_eq!(part1(BASIC_INPUT), 1227775554)
    }

    #[test]
    fn test_is_valid_id_full() {
        assert!(is_valid_id_full(12));
        assert!(is_valid_id_full(121));
        assert!(is_valid_id_full(11111112));
        assert!(!is_valid_id_full(121212));
        assert!(!is_valid_id_full(11));
        assert!(!is_valid_id_full(111));
        assert!(!is_valid_id_full(12341234));
        assert!(!is_valid_id_full(123123123));
    }

    #[test]
    fn basic_test_part2() {
        assert_eq!(part2(BASIC_INPUT), 4174379265);
    }
}
