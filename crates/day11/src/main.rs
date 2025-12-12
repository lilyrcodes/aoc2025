use petgraph::{
    algo::{all_simple_paths, is_cyclic_directed},
    graph::{DefaultIx, DiGraph, NodeIndex},
};
use std::{
    collections::{HashMap, VecDeque},
    fs::read_to_string,
    hash::RandomState,
};

#[derive(Clone, Debug)]
struct Input {
    graph: DiGraph<(), ()>,
    start: NodeIndex<DefaultIx>,
    end: NodeIndex<DefaultIx>,
}

fn parse_input(input: &str) -> Input {
    let mut graph = DiGraph::new();
    let mut lookup: HashMap<&str, NodeIndex<DefaultIx>> = HashMap::new();
    for line in input.lines() {
        let (from_node, to_nodes) = line.split_once(": ").unwrap();
        let to_nodes: Vec<&str> = to_nodes.split_whitespace().collect();
        if !lookup.contains_key(from_node) {
            lookup.insert(from_node, graph.add_node(()));
        }
        let from_idx = *lookup.get(from_node).unwrap();
        for to_node in to_nodes {
            if !lookup.contains_key(to_node) {
                lookup.insert(to_node, graph.add_node(()));
            }
            let to_idx = *lookup.get(to_node).unwrap();
            graph.add_edge(from_idx, to_idx, ());
        }
    }
    let start = *lookup.get("you").unwrap();
    let end = *lookup.get("out").unwrap();
    Input { graph, start, end }
}

fn get_path_count(input: &Input) -> usize {
    all_simple_paths::<Vec<_>, _, RandomState>(&input.graph, input.start, input.end, 0, None)
        .count()
}

fn part1(input: &str) -> usize {
    let input = parse_input(input);
    get_path_count(&input)
}

type IDX = NodeIndex<DefaultIx>;

#[derive(Clone, Debug)]
struct InputP2 {
    graph: DiGraph<(), ()>,
    svr: NodeIndex<DefaultIx>,
    dac: NodeIndex<DefaultIx>,
    fft: NodeIndex<DefaultIx>,
    out: NodeIndex<DefaultIx>,
}

fn parse_input_p2(input: &str) -> InputP2 {
    let mut graph = DiGraph::new();
    let mut lookup: HashMap<&str, IDX> = HashMap::new();
    for line in input.lines() {
        let (from_node, to_nodes) = line.split_once(": ").unwrap();
        let to_nodes: Vec<&str> = to_nodes.split_whitespace().collect();
        if !lookup.contains_key(from_node) {
            lookup.insert(from_node, graph.add_node(()));
        }
        let from_idx = *lookup.get(from_node).unwrap();
        for to_node in to_nodes {
            if !lookup.contains_key(to_node) {
                lookup.insert(to_node, graph.add_node(()));
            }
            let to_idx = *lookup.get(to_node).unwrap();
            graph.add_edge(from_idx, to_idx, ());
        }
    }
    let svr = *lookup.get("svr").unwrap();
    let dac = *lookup.get("dac").unwrap();
    let fft = *lookup.get("fft").unwrap();
    let out = *lookup.get("out").unwrap();

    InputP2 {
        graph,
        svr,
        dac,
        fft,
        out,
    }
}

fn count_paths(graph: &DiGraph<(), ()>, from: IDX, to: IDX) -> usize {
    let mut queue: VecDeque<IDX> = VecDeque::new();
    let mut paths: HashMap<IDX, usize> = HashMap::new();
    queue.push_back(from);
    paths.insert(from, 1);
    while let Some(node) = queue.pop_front() {
        if node == to {
            continue;
        }
        let cur_count = *paths.get(&node).unwrap();
        for neighbor in graph.neighbors(node) {
            if let Some(count) = paths.get_mut(&neighbor) {
                *count += cur_count;
            } else {
                paths.insert(neighbor, cur_count);
            }
            queue.push_back(neighbor);
        }
    }
    paths.get(&to).copied().unwrap_or(0)
}

fn get_path_count_p2(input: &InputP2) -> usize {
    let svr_to_dac = count_paths(&input.graph, input.svr, input.dac);
    println!("svr_to_dac {}", svr_to_dac);
    let svr_to_fft = count_paths(&input.graph, input.svr, input.fft);
    println!("svr_to_fft {}", svr_to_fft);
    let dac_to_fft = count_paths(&input.graph, input.dac, input.fft);
    println!("dac_to_fft {}", dac_to_fft);
    let fft_to_dac = count_paths(&input.graph, input.fft, input.dac);
    println!("fft_to_dac {}", fft_to_dac);
    let fft_to_out = count_paths(&input.graph, input.fft, input.out);
    println!("fft_to_out {}", fft_to_out);
    let dac_to_out = count_paths(&input.graph, input.dac, input.out);
    println!("dac_to_out {}", dac_to_out);
    svr_to_dac * dac_to_fft * fft_to_out + svr_to_fft * fft_to_dac * dac_to_out
}

fn part2(input: &str) -> usize {
    let input = parse_input_p2(input);
    assert!(!is_cyclic_directed(&input.graph));
    get_path_count_p2(&input)
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
    use crate::part1;
    use crate::part2;
    const BASIC_INPUT: &str = "aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out";
    const BASIC_INPUT_P2: &str = "svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out";

    #[test]
    fn basic_test_part1() {
        assert_eq!(part1(BASIC_INPUT), 5);
    }

    #[test]
    fn basic_test_part2() {
        assert_eq!(part2(BASIC_INPUT_P2), 2);
    }
}
