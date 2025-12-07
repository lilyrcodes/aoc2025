use std::{
    collections::{HashMap, HashSet, VecDeque},
    fs::read_to_string,
};

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, Debug)]
struct Location {
    y: usize,
    x: usize,
}

#[derive(PartialEq, Eq)]
struct Input {
    start: Location,
    splitters: HashSet<Location>,
}

fn parse_input(input: &str) -> Input {
    let splitters: HashSet<Location> = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.bytes()
                .enumerate()
                .filter_map(|(x, c)| {
                    if c == b'^' {
                        Some(Location { y, x })
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>()
        })
        .collect();
    let start = input
        .lines()
        .enumerate()
        .filter_map(|(y, line)| {
            line.bytes()
                .enumerate()
                .filter_map(|(x, c)| {
                    if c == b'S' {
                        Some(Location { y, x })
                    } else {
                        None
                    }
                })
                .next()
        })
        .next()
        .unwrap();
    Input { start, splitters }
}

fn part1(input: &str) -> u64 {
    let max_y = input.lines().count();
    let input = parse_input(input);
    let mut queue = VecDeque::new();
    queue.push_back(input.start);
    let mut considered: HashSet<Location> = HashSet::new();
    let mut split_count = 0;
    while let Some(pos) = queue.pop_front() {
        if considered.contains(&pos) {
            continue;
        }
        considered.insert(pos);
        if input.splitters.contains(&pos) {
            split_count += 1;
            queue.push_back(Location {
                y: pos.y,
                x: pos.x - 1,
            });
            queue.push_back(Location {
                y: pos.y,
                x: pos.x + 1,
            });
        } else if pos.y < max_y {
            queue.push_back(Location {
                y: pos.y + 1,
                x: pos.x,
            });
        }
    }
    split_count
}

fn part2(input: &str) -> usize {
    let max_y = input.lines().count();
    let max_x = input.lines().next().unwrap().len();
    let input = parse_input(input);
    let mut next_queue = VecDeque::new();
    next_queue.push_back(input.start);
    let mut times_hit: HashMap<Location, usize> = HashMap::new();
    times_hit.insert(input.start, 1);
    while !next_queue.is_empty() {
        let mut queue = next_queue;
        next_queue = VecDeque::new();
        while let Some(pos) = queue.pop_front() {
            let current_hits = times_hit.get(&pos).copied().unwrap();
            if input.splitters.contains(&pos) {
                let left = Location {
                    y: pos.y,
                    x: pos.x - 1,
                };
                let left_down = Location {
                    y: pos.y + 1,
                    x: pos.x - 1,
                };
                let right = Location {
                    y: pos.y,
                    x: pos.x + 1,
                };
                if let Some(count) = times_hit.get_mut(&right) {
                    *count += current_hits;
                } else {
                    times_hit.insert(right, current_hits);
                    queue.push_front(right);
                }
                if let Some(count) = times_hit.get_mut(&left) {
                    *count += current_hits;
                    if let Some(count) = times_hit.get_mut(&left_down) {
                        *count += current_hits;
                    }
                } else {
                    times_hit.insert(left, current_hits);
                    queue.push_front(left);
                }
            } else if pos.y < max_y {
                let next = Location {
                    y: pos.y + 1,
                    x: pos.x,
                };
                if let Some(count) = times_hit.get_mut(&next) {
                    *count += current_hits;
                } else {
                    times_hit.insert(next, current_hits);
                    next_queue.push_back(next);
                }
            }
        }
    }
    let mut timeline_count = 0;
    for x in 0..max_x {
        let loc = Location { y: max_y, x };
        if let Some(timelines) = times_hit.get(&loc) {
            timeline_count += timelines;
        }
    }
    timeline_count
}

fn main() {
    let input = read_to_string("input.txt").unwrap();
    let p1_answer = part1(&input);
    println!("part 1: {}", p1_answer);
    let p2_answer = part2(&input);
    println!("part 2: {}", p2_answer);
}

#[cfg(test)]
mod tests {
    use crate::{part1, part2};
    const BASIC_INPUT: &str = ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............";

    const SIMPLE_INPUT: &str = "..S..
.....
..^..
.....
.^.^.
.....";

    #[test]
    fn basic_test_part1() {
        assert_eq!(part1(BASIC_INPUT), 21);
    }

    #[test]
    fn basic_test_part2() {
        assert_eq!(part2(SIMPLE_INPUT), 4);
        assert_eq!(part2(BASIC_INPUT), 40);
    }
}
