use std::{fs::read_to_string, str::FromStr};

const SHAPE_SIZE: usize = 3;

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
struct Shape {
    grid: [[bool; SHAPE_SIZE]; SHAPE_SIZE],
}

impl FromStr for Shape {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut shape = Shape::default();
        for (y, line) in s.lines().enumerate() {
            for (x, c) in line.as_bytes().iter().enumerate() {
                shape.grid[y][x] = *c == b'#';
            }
        }
        Ok(shape)
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct Region<const N: usize> {
    x: u8,
    y: u8,
    present_counts: [u8; N],
}

impl<const N: usize> FromStr for Region<N> {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (coords, present_counts) = s.split_once(": ").unwrap();
        let (x, y) = coords.split_once('x').unwrap();
        let x = x.parse::<u8>().unwrap();
        let y = y.parse::<u8>().unwrap();
        let present_counts_vec: Vec<u8> = present_counts
            .split(' ')
            .map(|count| count.parse::<u8>().unwrap())
            .collect();
        let mut present_counts: [u8; N] = [0; N];
        for (i, c) in present_counts_vec.into_iter().enumerate() {
            present_counts[i] = c;
        }
        Ok(Region {
            x,
            y,
            present_counts,
        })
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
struct Input<const N: usize> {
    shapes: [Shape; N],
    regions: Vec<Region<N>>,
}

impl<const N: usize> FromStr for Input<N> {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut shapes = [Shape::default(); N];
        let mut iter = s.lines();
        for i in 0..N {
            // skip label
            iter.next();
            // get shape string
            let mut shape = String::default();
            for _ in 0..SHAPE_SIZE {
                shape.push_str(iter.next().unwrap());
                shape.push('\n');
            }
            shapes[i] = shape.trim().parse::<Shape>().unwrap();
            // skip newline
            iter.next();
        }
        let regions = iter
            .map(|line| line.parse::<Region<N>>().unwrap())
            .collect();
        Ok(Input { shapes, regions })
    }
}

fn parse_input<const N: usize>(input: &str) -> Input<N> {
    input.parse().unwrap()
}

fn free_area(region: &[Vec<bool>]) -> usize {
    region
        .iter()
        .map(|l| l.iter().filter(|b| !**b).count())
        .sum()
}

fn rotate(shape: &Shape, rotation: Rotation) -> Shape {
    let grid = match rotation {
        Rotation::ZERO => shape.grid,
        Rotation::ONE => [
            [shape.grid[2][0], shape.grid[1][0], shape.grid[0][0]],
            [shape.grid[2][1], shape.grid[1][1], shape.grid[0][1]],
            [shape.grid[2][2], shape.grid[1][2], shape.grid[0][2]],
        ],
        Rotation::TWO => [
            [shape.grid[2][2], shape.grid[2][1], shape.grid[2][0]],
            [shape.grid[1][2], shape.grid[1][1], shape.grid[1][0]],
            [shape.grid[0][2], shape.grid[0][1], shape.grid[0][0]],
        ],
        Rotation::THREE => [
            [shape.grid[0][2], shape.grid[1][2], shape.grid[2][2]],
            [shape.grid[0][1], shape.grid[1][1], shape.grid[2][1]],
            [shape.grid[0][0], shape.grid[1][0], shape.grid[2][1]],
        ],
    };
    Shape { grid }
}

fn area(shape: &Shape) -> usize {
    shape
        .grid
        .iter()
        .map(|l| l.iter().filter(|b| **b).count())
        .sum()
}

fn simple_check(region: &[Vec<bool>], shape: &Shape) -> bool {
    free_area(region) >= area(shape)
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Rotation {
    ZERO,
    ONE,
    TWO,
    THREE,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct Placement {
    x: u8,
    y: u8,
    rotation: Rotation,
}

// Gets a list of spots a shape can be placed
fn valid_placements(region: &[Vec<bool>], shape: &Shape) -> Vec<Placement> {
    todo!()
}

fn place(region: &[Vec<bool>], shape: &Shape, placement: &Placement) -> Vec<Vec<bool>> {
    todo!()
}

// Gets a list of valid updated regions with the shape placed in it. Return value is empty if the
// present cannot fit.
fn fit_present(region: &[Vec<bool>], shape: &Shape) -> Vec<Vec<Vec<bool>>> {
    if !simple_check(region, shape) {
        return vec![];
    }
    valid_placements(region, shape)
        .into_iter()
        .map(|placement| place(region, shape, &placement))
        .collect()
}

fn part1<const N: usize>(input: &str) -> usize {
    let input = input.parse::<Input<N>>().unwrap();
    let mut count = 0;

    for region in input.regions {
        for (present_idx, present_count) in region.present_counts.iter().enumerate() {
            let present = input.shapes[present_idx];
            for _ in 0..*present_count {
                todo!()
            }
        }
    }
    count
}

/*
fn part2(input: &str) -> usize {
    todo!()
}
*/

fn main() {
    let input = read_to_string("input.txt").unwrap();
    let p1_answer = part1::<6>(&input);
    println!("part 1: {}", p1_answer);
    // let p2_answer = part2(&input);
    // println!("part 2: {}", p2_answer);
}

#[cfg(test)]
mod tests {
    use crate::part1;
    // use crate::part2;

    const BASIC_INPUT: &str = "0:
###
##.
##.

1:
###
##.
.##

2:
.##
###
##.

3:
##.
###
##.

4:
###
#..
###

5:
###
.#.
###

4x4: 0 0 0 0 2 0
12x5: 1 0 1 0 2 2
12x5: 1 0 1 0 3 2";

    #[test]
    fn basic_test_part1() {
        assert_eq!(part1::<6>(BASIC_INPUT), 2);
    }

    /*
    #[test]
    fn basic_test_part2() {
        assert_eq!(part2(BASIC_INPUT_P2), 2);
    }
    */
}
