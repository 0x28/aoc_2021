use aoc_2021::input_file;
use std::{
    collections::{HashMap, HashSet},
    fs,
};

fn parse(input: &str) -> HashMap<String, Vec<String>> {
    let mut map: HashMap<String, Vec<String>> = HashMap::new();
    let connections = input.lines().map(|l| {
        let start_end = l.split('-').collect::<Vec<_>>();

        (start_end[0], start_end[1])
    });

    for (from, to) in connections {
        if let Some(dests) = map.get_mut(from) {
            dests.push(to.to_string());
        } else {
            map.insert(from.to_string(), vec![to.to_string()]);
        }

        if let Some(dests) = map.get_mut(to) {
            dests.push(from.to_string());
        } else {
            map.insert(to.to_string(), vec![from.to_string()]);
        }
    }

    map
}

fn walk(
    current: &str,
    edges: &HashMap<String, Vec<String>>,
    visited: &HashSet<String>,
) -> i64 {
    if current == "end" {
        return 1;
    }
    let mut sum = 0;

    for path in edges.get(current).unwrap() {
        if visited.contains(path) {
            continue;
        }

        let mut visited = visited.clone();
        if path.chars().all(char::is_lowercase) {
            visited.insert(path.to_string());
        }

        sum += walk(path, edges, &visited);
    }

    sum
}

fn part1(edges: &HashMap<String, Vec<String>>) -> i64 {
    let mut visited = HashSet::new();
    visited.insert("start".to_string());
    walk("start", edges, &visited)
}

fn walk2(
    current: &str,
    edges: &HashMap<String, Vec<String>>,
    visited: &mut Vec<String>,
    ignore_double: bool,
) -> usize {
    if current == "end" {
        return 1;
    }
    let mut sum = 0;

    for node in edges.get(current).unwrap() {
        let mut ignore = ignore_double;
        if visited.contains(node) {
            if ignore && node != "start" {
                ignore = false;
            } else {
                continue;
            }
        }

        if node.chars().all(char::is_lowercase) {
            visited.push(node.to_string());
            sum += walk2(node, edges, visited, ignore);
            visited.pop();
        } else {
            sum += walk2(node, edges, visited, ignore);
        }
    }

    sum
}

fn part2(edges: &HashMap<String, Vec<String>>) -> usize {
    let mut visited = vec!["start".to_string()];

    walk2("start", edges, &mut visited, true)
}

fn main() {
    let input = fs::read_to_string(input_file("input12.txt")).unwrap();
    let input = parse(&input);

    println!("part1 = {}", part1(&input));
    println!("part2 = {}", part2(&input));
}

#[test]
fn test_day12() {
    let input = "\
start-A
start-b
A-c
A-b
b-d
A-end
b-end";
    let input = parse(input);

    assert_eq!(part1(&input), 10);
    assert_eq!(part2(&input), 36);
}
