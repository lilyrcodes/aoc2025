use std::{fs::read_to_string, str::FromStr};

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Clone)]
struct Button {
    toggles_lights: Vec<usize>,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Clone)]
struct Machine {
    desired_state: Vec<bool>,
    buttons: Vec<Button>,
}

impl FromStr for Machine {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (desired_state, buttons) = s.split_once(']').unwrap();
        let desired_state = &desired_state[1..];
        let desired_state = desired_state.chars().map(|c| c == '#').collect();
        let (buttons, _) = buttons.split_once('{').unwrap();
        let buttons = buttons
            .trim()
            .split_whitespace()
            .map(|tup| &tup[1..tup.len() - 1])
            .map(|lst| Button {
                toggles_lights: lst
                    .split(',')
                    .map(|num| num.parse::<usize>().unwrap())
                    .collect(),
            })
            .collect();
        Ok(Machine {
            desired_state,
            buttons,
        })
    }
}

fn parse_input(input: &str) -> Vec<Machine> {
    todo!()
}

fn part1(input: &str) -> usize {
    todo!()
}

fn part2(input: &str) -> i64 {
    todo!()
}

fn main() {
    let input = read_to_string("input.txt").unwrap();
    let p1_answer = part1(&input);
    println!("part 1: {}", p1_answer);
    /*
    let p2_answer = part2(&input);
    println!("part 2: {}", p2_answer);
    */
}

#[cfg(test)]
mod tests {
    use crate::{part1, part2};
    const BASIC_INPUT: &str = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}";

    #[test]
    fn basic_test_part1() {
        assert_eq!(part1(BASIC_INPUT.lines().next().unwrap()), 2);
        assert_eq!(part1(BASIC_INPUT), 7);
    }

    /*
    #[test]
    fn basic_test_part2() {
        assert_eq!(part2(BASIC_INPUT), 25272);
    }
    */
}
