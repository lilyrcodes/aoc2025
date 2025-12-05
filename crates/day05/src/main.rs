use std::{fs::read_to_string, ops::RangeInclusive};

struct Input {
    ranges: Vec<RangeInclusive<u64>>,
    to_check: Vec<u64>,
}

fn parse_range(line: &str) -> RangeInclusive<u64> {
    let mut split = line.split('-');
    let low = split.next().unwrap().parse::<u64>().unwrap();
    let high = split.next().unwrap().parse::<u64>().unwrap();
    low..=high
}

fn parse_input(input: &str) -> Input {
    let mut ranges = vec![];
    let mut to_check = vec![];
    let mut done_ranges = false;
    for line in input.lines() {
        if line.trim().is_empty() {
            done_ranges = true;
        } else if !done_ranges {
            ranges.push(parse_range(line));
        } else {
            to_check.push(line.parse::<u64>().unwrap());
        }
    }
    Input { ranges, to_check }
}

fn part1(input: &str) -> u64 {
    let input = parse_input(input);
    let mut count = 0;
    for check in input.to_check {
        for range in input.ranges.iter() {
            if range.contains(&check) {
                count += 1;
                break;
            }
        }
    }
    count
}

fn range_intersects(left: &RangeInclusive<u64>, right: &RangeInclusive<u64>) -> bool {
    left.contains(right.start())
        || left.contains(right.end())
        || right.contains(left.start())
        || right.contains(left.end())
}

fn merge_ranges(ranges: &mut [RangeInclusive<u64>]) -> Vec<RangeInclusive<u64>> {
    ranges.sort_by(|left, right| left.start().cmp(right.start()));
    let mut new_ranges = vec![ranges[0].clone()];
    for to_consider in ranges.iter().skip(1) {
        let last_idx = new_ranges.len() - 1;
        if range_intersects(&new_ranges[last_idx], to_consider) {
            new_ranges[last_idx] = (*to_consider.start()).min(*new_ranges[last_idx].start())
                ..=(*to_consider.end()).max(*new_ranges[last_idx].end());
        } else {
            new_ranges.push(to_consider.clone());
        }
    }
    new_ranges
}

fn part2(input: &str) -> u64 {
    let mut input = parse_input(input);
    let merged = merge_ranges(&mut input.ranges);
    merged.into_iter().map(|r| r.end() - r.start() + 1).sum()
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
    const BASIC_INPUT: &str = "3-5
10-14
16-20
12-18

1
5
8
11
17
32";

    #[test]
    fn basic_test_part1() {
        assert_eq!(part1(BASIC_INPUT), 3)
    }

    #[test]
    fn basic_test_part2() {
        assert_eq!(part2(BASIC_INPUT), 14)
    }
}
