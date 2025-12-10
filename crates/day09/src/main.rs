use std::{
    collections::{HashMap, HashSet, VecDeque},
    fs::read_to_string,
};

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Clone, Copy)]
struct Coord {
    x: usize,
    y: usize,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Clone, Copy)]
enum Tile {
    UNKNOWN,
    VALID,
    INVALID,
}

#[derive(PartialEq, Eq, PartialOrd, Debug, Clone)]
struct TileGrid {
    tiles: Vec<Tile>,
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

fn get_tile(tiles: &TileGrid, x: usize, y: usize) -> Tile {
    tiles.tiles[tile_index(tiles, x, y)]
}

fn all_set(tiles: &TileGrid, bools: &[bool], min_x: usize, max_x: usize, y: usize) -> bool {
    bools[tile_index(tiles, min_x, y)..=tile_index(tiles, max_x, y)]
        .iter()
        .all(|b| *b)
}

fn to_bools(tiles: &TileGrid) -> Vec<bool> {
    tiles
        .tiles
        .iter()
        .map(|t| *t == Tile::VALID || *t == Tile::UNKNOWN)
        .collect()
}

fn set_tile(tiles: &mut TileGrid, x: usize, y: usize, tile: Tile) {
    let idx = tile_index(tiles, x, y);
    tiles.tiles[idx] = tile
}

fn green_line(tiles: &mut TileGrid, prev: &Coord, coord: &Coord) {
    if prev.x != coord.x {
        let from = prev.x.min(coord.x);
        let to = prev.x.max(coord.x);
        for x in from..=to {
            set_tile(tiles, x, prev.y, Tile::VALID);
        }
    } else {
        let from = prev.y.min(coord.y);
        let to = prev.y.max(coord.y);
        for y in from..=to {
            set_tile(tiles, prev.x, y, Tile::VALID);
        }
    }
}

fn is_explored(tiles: &TileGrid, explored: &[bool], x: usize, y: usize) -> bool {
    explored[tile_index(tiles, x, y)]
}

fn set_explored(tiles: &TileGrid, explored: &mut [bool], x: usize, y: usize) {
    explored[tile_index(tiles, x, y)] = true
}

fn fill_grid(tiles: &mut TileGrid) {
    let mut queue: VecDeque<Coord> = VecDeque::new();
    let mut explored: Vec<bool> = vec![false; (tiles.largest_x + 2) * (tiles.largest_y + 2)];
    queue.push_back(Coord { x: 0, y: 0 });
    let mut count: usize = 0;
    while let Some(coord) = queue.pop_front() {
        if count % 1_000_000 == 999_999 {
            println!("{} / {}", count / 1_000_000, explored.len() / 1_000_000);
        }
        if is_explored(tiles, &explored, coord.x, coord.y) {
            continue;
        }
        if get_tile(tiles, coord.x, coord.y) == Tile::VALID {
            count += 1;
            set_explored(tiles, &mut explored, coord.x, coord.y);
            continue;
        }
        set_tile(tiles, coord.x, coord.y, Tile::INVALID);
        set_explored(tiles, &mut explored, coord.x, coord.y);
        count += 1;
        if coord.x != 0 {
            let left = Coord {
                x: coord.x - 1,
                y: coord.y,
            };
            if !is_explored(tiles, &explored, left.x, left.y) {
                queue.push_back(left);
            }
        }
        if coord.y != 0 {
            let up = Coord {
                x: coord.x,
                y: coord.y - 1,
            };
            if !is_explored(tiles, &explored, up.x, up.y) {
                queue.push_back(up);
            }
        }
        if coord.x < tiles.largest_x {
            let right = Coord {
                x: coord.x + 1,
                y: coord.y,
            };
            if !is_explored(tiles, &explored, right.x, right.y) {
                queue.push_back(right);
            }
        }
        if coord.y < tiles.largest_y {
            let down = Coord {
                x: coord.x,
                y: coord.y + 1,
            };
            if !is_explored(tiles, &explored, down.x, down.y) {
                queue.push_back(down);
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
        tiles: vec![Tile::UNKNOWN; (largest_x + 2) * (largest_y + 2)],
        largest_x,
        largest_y,
    };
    let mut prev = coords[0];
    set_tile(&mut tiles, prev.x, prev.y, Tile::VALID);
    for coord in coords.iter().skip(1) {
        green_line(&mut tiles, &prev, coord);
        //print_grid(&tiles);
        prev = *coord;
    }
    green_line(&mut tiles, &prev, &coords[0]);
    println!("lines done");
    //print_grid(&tiles);
    fill_grid(&mut tiles);
    //print_grid(&tiles);
    tiles
}

fn all_red_or_green(tiles: &TileGrid, bools: &[bool], a: &Coord, b: &Coord) -> bool {
    let min_y = a.y.min(b.y);
    let max_y = a.y.max(b.y);
    let min_x = a.x.min(b.x);
    let max_x = a.x.max(b.x);
    for y in min_y..=max_y {
        if !all_set(tiles, bools, min_x, max_x, y) {
            return false;
        }
    }
    true
}

fn print_grid(tiles: &TileGrid) {
    println!();
    for y in 0..=tiles.largest_y + 1 {
        for x in 0..=tiles.largest_x + 1 {
            let tile = get_tile(tiles, x, y);
            if tile == Tile::VALID {
                print!("#");
            } else if tile == Tile::UNKNOWN {
                print!("?");
            } else {
                print!(".");
            }
        }
        println!();
    }
    println!();
}

fn translate_coords(coords: &[Coord]) -> (HashMap<Coord, Coord>, Vec<Coord>) {
    let mut temp: Vec<usize> = coords
        .iter()
        .map(|c| c.x)
        .collect::<HashSet<usize>>()
        .into_iter()
        .collect();
    temp.sort();
    let x_lookup: HashMap<usize, usize> = temp
        .into_iter()
        .enumerate()
        .map(|(i, x)| (x, (i + 1) * 3))
        .collect();
    let mut temp: Vec<usize> = coords
        .iter()
        .map(|c| c.y)
        .collect::<HashSet<usize>>()
        .into_iter()
        .collect();
    temp.sort();
    let y_lookup: HashMap<usize, usize> = temp
        .into_iter()
        .enumerate()
        .map(|(i, y)| (y, (i + 1) * 3))
        .collect();
    (
        coords
            .iter()
            .map(|c| {
                (
                    Coord {
                        x: *x_lookup.get(&c.x).unwrap(),
                        y: *y_lookup.get(&c.y).unwrap(),
                    },
                    *c,
                )
            })
            .collect(),
        coords
            .iter()
            .map(|c| Coord {
                x: *x_lookup.get(&c.x).unwrap(),
                y: *y_lookup.get(&c.y).unwrap(),
            })
            .collect(),
    )
}

fn part2(input: &str) -> usize {
    let coords = parse_input(input);
    let (translation, coords) = translate_coords(&coords);
    let tiles = generate_map(&coords);
    let bools = to_bools(&tiles);
    println!("Map generated");
    let mut largest = 0;
    for (i, a) in coords.iter().enumerate() {
        println!("{}", i);
        for b in coords.iter().skip(i) {
            let translated_a = translation.get(a).unwrap();
            let translated_b = translation.get(b).unwrap();
            let area = area(translated_a, translated_b);
            if area > largest && all_red_or_green(&tiles, &bools, a, b) {
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
        assert_eq!(part2(CONCAVE_INPUT), 25);
    }
}
