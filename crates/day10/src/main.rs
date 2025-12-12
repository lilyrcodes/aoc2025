use std::{collections::VecDeque, fs::read_to_string, str::FromStr};

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Clone)]
struct Button {
    toggles_lights: Vec<usize>,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Clone)]
struct Machine {
    desired_lights: Vec<bool>,
    buttons: Vec<Button>,
    desired_joltages: Vec<usize>,
}

impl FromStr for Machine {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (desired_lights, buttons) = s.split_once(']').unwrap();
        let desired_lights = &desired_lights[1..];
        let desired_lights = desired_lights.chars().map(|c| c == '#').collect();
        let (buttons, desired_joltages) = buttons.split_once('{').unwrap();
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
        let desired_joltages = &desired_joltages[..desired_joltages.len()-1];
        let desired_joltages = desired_joltages.split(',').map(|num| num.parse::<usize>().unwrap()).collect();
        Ok(Machine {
            desired_lights,
            buttons,
            desired_joltages,
        })
    }
}

fn compute_state(machine: &Machine, button_presses: &[usize]) -> Vec<bool> {
    let mut state: Vec<bool> = vec![false; machine.desired_lights.len()];
    for press in 0..button_presses.len() {
        if button_presses[press] % 2 == 1 {
            for light in machine.buttons[press].toggles_lights.iter() {
                state[*light] = !state[*light];
            }
        }
    }
    state
}

fn button_presses_for_lights(machine: &Machine) -> usize {
    let mut queue: VecDeque<Vec<usize>> = VecDeque::new();
    queue.push_back(vec![0; machine.buttons.len()]);
    while let Some(presses) = queue.pop_front() {
        if compute_state(machine, &presses) == machine.desired_lights {
            return presses.into_iter().sum();
        }
        for button in 0..machine.buttons.len() {
            let mut next_presses = presses.clone();
            next_presses[button] += 1;
            queue.push_back(next_presses);
        }
    }
    panic!("No solution found");
}

fn parse_input(input: &str) -> Vec<Machine> {
    input
        .lines()
        .map(|line| line.parse::<Machine>().unwrap())
        .collect()
}

fn part1(input: &str) -> usize {
    parse_input(input).iter().map(button_presses_for_lights).sum()
}

fn compute_joltages(machine: &Machine, button_presses: &[usize]) -> Vec<usize> {
    let mut joltages: Vec<usize> = vec![0; machine.desired_joltages.len()];
    for (button_idx, presses) in button_presses.iter().enumerate() {
        for light in machine.buttons[button_idx].toggles_lights.iter() {
            joltages[*light] += presses;
        }
    }
    joltages
}

fn button_presses_for_joltages(machine: &Machine) -> usize {
    let mut queue: VecDeque<Vec<usize>> = VecDeque::new();
    queue.push_back(vec![0; machine.buttons.len()]);
    while let Some(presses) = queue.pop_front() {
        let joltages = compute_joltages(machine, &presses);
        if joltages == machine.desired_joltages {
            return presses.into_iter().sum();
        }
        if joltages.iter().zip(machine.desired_joltages.iter()).any(|(c, desired)| c > desired) {
            continue;
        }

        if compute_state(machine, &presses) == machine.desired_lights {
            return presses.into_iter().sum();
        }
        for button in 0..machine.buttons.len() {
            let mut next_presses = presses.clone();
            next_presses[button] += 1;
            queue.push_back(next_presses);
        }
    }
    panic!("No solution found");
}

fn part2(input: &str) -> usize {
    parse_input(input).iter().map(button_presses_for_joltages).sum()
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
    use crate::{Button, Machine, compute_joltages, compute_state, part1, part2};
    const BASIC_INPUT: &str = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}";

    fn make_machine() -> Machine {
        Machine {
            desired_lights: vec![false, true, true, false],
            buttons: vec![
                Button {
                    toggles_lights: vec![3],
                },
                Button {
                    toggles_lights: vec![1, 3],
                },
                Button {
                    toggles_lights: vec![2],
                },
                Button {
                    toggles_lights: vec![2, 3],
                },
                Button {
                    toggles_lights: vec![0, 2],
                },
                Button {
                    toggles_lights: vec![0, 1],
                },
            ],
            desired_joltages: vec![3,5,4,7],
        }
    }

    #[test]
    fn compute_state_works() {
        let machine = make_machine();
        let button_presses = vec![0, 0, 0, 0, 1, 1];
        assert_eq!(
            compute_state(&machine, &button_presses),
            machine.desired_lights
        );
    }

    #[test]
    fn single_machine() {
        assert_eq!(part1(BASIC_INPUT.lines().next().unwrap()), 2);
    }

    #[test]
    fn basic_test_part1() {
        assert_eq!(part1(BASIC_INPUT), 7);
    }

    #[test]
    fn compute_joltages_works() {
        let machine = make_machine();
        let button_presses = vec![1, 3, 0, 3, 1, 2];
        assert_eq!(compute_joltages(&machine, &button_presses), machine.desired_joltages);
    }

    #[test]
    fn single_machine_joltage() {
        assert_eq!(part2(BASIC_INPUT.lines().next().unwrap()), 10);
    }
    /*
    #[test]
    fn basic_test_part2() {
        assert_eq!(part2(BASIC_INPUT), 25272);
    }
    */
}
