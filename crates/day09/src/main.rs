use std::fs::read_to_string;

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Clone, Copy)]
struct Coord {
    x: usize,
    y: usize,
}

#[derive(PartialEq, Eq, PartialOrd, Debug, Clone)]
struct TileGrid {
    tiles: Vec<bool>,
    largest_x: usize,
    largest_y: usize,
}

fn area(a: &Coord, b: &Coord) -> usize {
    (a.x.max(b.x) - a.x.min(b.x) + 1) * (a.y.max(b.y) - a.y.min(b.y) + 1)
}

fn parse_coord(line: &str) -> Coord {
    let mut split = line.split(',');
    let x = split.next().unwrap().parse().unwrap();
    let y = split.next().unwrap().parse().unwrap();
    Coord { x, y }
}

fn parse_input(input: &str) -> Vec<Coord> {
    input.lines().map(parse_coord).collect()
}

fn part1(input: &str) -> usize {
    let coords = parse_input(input);
    let mut largest = 0;
    for (i, a) in coords.iter().enumerate() {
        for b in coords.iter().skip(i) {
            let area = area(a, b);
            if area > largest {
                largest = area;
            }
        }
    }
    largest
}

fn tile_index(tiles: &TileGrid, x: usize, y: usize) -> usize {
    x + (tiles.largest_x + 1) * y
}

fn get_tile(tiles: &TileGrid, x: usize, y: usize) -> bool {
    tiles.tiles[tile_index(tiles, x, y)]
}

fn all_set(tiles: &TileGrid, min_x: usize, max_x: usize, y: usize) -> bool {
    tiles.tiles[tile_index(tiles, min_x, y)..=tile_index(tiles, max_x, y)]
        .iter()
        .all(|t| *t)
}

fn set_tile(tiles: &mut TileGrid, x: usize, y: usize) {
    let idx = tile_index(tiles, x, y);
    tiles.tiles[idx] = true
}

fn green_line(tiles: &mut TileGrid, prev: &Coord, coord: &Coord) {
    if prev.x != coord.x {
        let from = prev.x.min(coord.x);
        let to = prev.x.max(coord.x);
        for x in from..=to {
            set_tile(tiles, x, prev.y);
        }
    } else {
        let from = prev.y.min(coord.y);
        let to = prev.y.max(coord.y);
        for y in from..=to {
            set_tile(tiles, prev.x, y);
        }
    }
}

// Doesn't work, need a better way to check if coord is in the polygon
fn fill_grid(tiles: &mut TileGrid) {
    for y in 0..=tiles.largest_y {
        let mut on_green = false;
        for x in 0..=tiles.largest_x {
            if get_tile(tiles, x, y) {
                on_green = !on_green;
            } else if on_green {
                set_tile(tiles, x, y);
            }
        }
    }
}

fn generate_map(coords: &[Coord]) -> TileGrid {
    let largest_x = coords.iter().map(|coord| coord.x).max().unwrap();
    let largest_y = coords.iter().map(|coord| coord.y).max().unwrap();
    /*
        let largest_x = 99999;
        let largest_y = 99999;
    */
    let mut tiles = TileGrid {
        tiles: vec![false; (largest_x + 1) * (largest_y + 1)],
        largest_x,
        largest_y,
    };
    let mut prev = coords[0];
    set_tile(&mut tiles, prev.x, prev.y);
    for coord in coords.iter().skip(1) {
        green_line(&mut tiles, &prev, coord);
        print_grid(&tiles);
        prev = *coord;
    }
    green_line(&mut tiles, &prev, &coords[0]);
    print_grid(&tiles);
    fill_grid(&mut tiles);
    tiles
}

fn all_red_or_green(tiles: &TileGrid, a: &Coord, b: &Coord) -> bool {
    let min_y = a.y.min(b.y);
    let max_y = a.y.max(b.y);
    let min_x = a.x.min(b.x);
    let max_x = a.x.max(b.x);
    for y in min_y..=max_y {
        if !all_set(tiles, min_x, max_x, y) {
            return false;
        }
    }
    true
}

fn print_grid(tiles: &TileGrid) {
    println!();
    for y in 0..=tiles.largest_y {
        for x in 0..=tiles.largest_x {
            if get_tile(tiles, x, y) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
    println!();
}

fn part2(input: &str) -> usize {
    let coords = parse_input(input);
    let tiles = generate_map(&coords);
    print_grid(&tiles);
    println!("Map generated");
    let mut largest = 0;
    for (i, a) in coords.iter().enumerate() {
        println!("{}", i);
        for b in coords.iter().skip(i) {
            let area = area(a, b);
            if area > largest && all_red_or_green(&tiles, a, b) {
                largest = area;
            }
        }
    }
    largest
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
    const BASIC_INPUT: &str = "7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3";
    const CONCAVE_INPUT: &str = "7,1
11,1
11,7
9,7
9,5
7,5
7,4
3,4
3,5
2,5
2,3
7,3";

    /*
    ..............
    .......#XXX#..
    .......XXXXX..
    ..#XXXX#XXXX..
    ..X#XXX#XXXX..
    ..##...#X#XX..
    .........XXX..
    .........#X#..
    ..............
        */

    #[test]
    fn basic_test_part1() {
        assert_eq!(part1(BASIC_INPUT), 50);
    }

    #[test]
    fn basic_test_part2() {
        assert_eq!(part2(BASIC_INPUT), 24);
        // assert_eq!(part2(CONCAVE_INPUT), 15);
    }
}
