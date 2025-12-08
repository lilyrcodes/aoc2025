use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, HashSet},
    fs::read_to_string,
    sync::{Arc, Mutex},
};

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Clone, Copy)]
struct Coord {
    x: i64,
    y: i64,
    z: i64,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy, Hash)]
struct PairWithDistance {
    distance: i64,
    a: Coord,
    b: Coord,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Circuit {
    coords: HashSet<Coord>,
}

impl PartialOrd for Circuit {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.coords.len().partial_cmp(&other.coords.len())
    }
}

impl Ord for Circuit {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.coords.len().cmp(&other.coords.len())
    }
}

impl Circuit {
    fn contains(&self, coord: &Coord) -> bool {
        self.coords.contains(coord)
    }
}

fn distance_squared(a: &Coord, b: &Coord) -> i64 {
    let xdiff = b.x - a.x;
    let ydiff = b.y - a.y;
    let zdiff = b.z - a.z;
    xdiff * xdiff + ydiff * ydiff + zdiff * zdiff
}

fn parse_input(input: &str) -> Vec<Coord> {
    input
        .lines()
        .map(|line| {
            line.split(',')
                .map(|num| num.parse::<i64>().unwrap())
                .collect::<Vec<i64>>()
        })
        .map(|v| Coord {
            x: v[0],
            y: v[1],
            z: v[2],
        })
        .collect()
}

fn compute_distances(coords: &[Coord]) -> BinaryHeap<Reverse<PairWithDistance>> {
    let mut heap = BinaryHeap::new();
    for (i, a) in coords.iter().enumerate() {
        for b in coords.iter().skip(i + 1) {
            let distance = distance_squared(a, b);
            heap.push(Reverse(PairWithDistance {
                distance,
                a: *a,
                b: *b,
            }));
        }
    }
    heap
}

fn add_connection_to_circuit(
    circuits: &mut HashMap<Coord, Arc<Mutex<Circuit>>>,
    a: &Coord,
    b: &Coord,
) {
    let circuit_a = circuits.remove(a).unwrap();
    if circuit_a.lock().unwrap().contains(b) {
        circuits.insert(*a, circuit_a);
        return;
    }
    let circuit_b = circuits.remove(b).unwrap();
    let circuit_b = circuit_b.lock().unwrap();
    {
        let mut circuit_a = circuit_a.lock().unwrap();
        for coord in circuit_b.coords.iter().copied() {
            circuit_a.coords.insert(coord);
        }
    }
    for coord in circuit_b.coords.iter().copied() {
        circuits.insert(coord, circuit_a.clone());
    }
    circuits.insert(*a, circuit_a);
}

fn init_circuits(coords: &[Coord]) -> HashMap<Coord, Arc<Mutex<Circuit>>> {
    coords
        .iter()
        .copied()
        .map(|c| {
            let mut h = HashSet::new();
            h.insert(c);
            (c, Arc::new(Mutex::new(Circuit { coords: h })))
        })
        .collect()
}

fn part1(input: &str, connections: u64) -> usize {
    let coords = parse_input(input);
    let mut dist_heap = compute_distances(&coords);
    let mut circuits = init_circuits(&coords);
    for _ in 0..connections {
        let d = dist_heap.pop().unwrap().0;
        add_connection_to_circuit(&mut circuits, &d.a, &d.b);
    }

    // Sort circuit refs by size, then get the top three distinct circuits
    let mut circuits = circuits
        .into_values()
        .map(|c| c.lock().unwrap().clone())
        .collect::<Vec<Circuit>>();
    circuits.sort();
    let mut iter = circuits.into_iter().rev();
    let a = iter.next().unwrap();
    let mut b = iter.next().unwrap();
    while a == b {
        b = iter.next().unwrap();
    }
    let mut c = iter.next().unwrap();
    while b == c {
        c = iter.next().unwrap();
    }
    a.coords.len() * b.coords.len() * c.coords.len()
}

fn part2(input: &str) -> i64 {
    let coords = parse_input(input);
    let mut dist_heap = compute_distances(&coords);
    let mut circuits = init_circuits(&coords);
    while let Some(d) = dist_heap.pop() {
        let d = d.0;
        add_connection_to_circuit(&mut circuits, &d.a, &d.b);
        if circuits.get(&d.a).unwrap().lock().unwrap().coords.len() == coords.len() {
            return d.a.x * d.b.x;
        }
    }
    panic!("Did not connect graph")
}

fn main() {
    let input = read_to_string("input.txt").unwrap();
    let p1_answer = part1(&input, 1000);
    println!("part 1: {}", p1_answer);
    let p2_answer = part2(&input);
    println!("part 2: {}", p2_answer);
}

#[cfg(test)]
mod tests {
    use crate::{part1, part2};
    const BASIC_INPUT: &str = "162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689";

    #[test]
    fn basic_test_part1() {
        assert_eq!(part1(BASIC_INPUT, 10), 40);
    }

    #[test]
    fn basic_test_part2() {
        assert_eq!(part2(BASIC_INPUT), 25272);
    }
}
