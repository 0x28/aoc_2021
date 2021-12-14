use aoc_2021::input_file;
use std::{collections::HashMap, fs, hash::Hash, ops::AddAssign};

fn parse(input: &str) -> (&str, HashMap<&str, char>) {
    let mut split = input.split("\n\n");

    let initial = split.next().unwrap();
    let mut transitions = HashMap::new();

    for line in split.next().unwrap().lines() {
        let mut line_split = line.split(" -> ");

        transitions.insert(
            line_split.next().unwrap(),
            line_split.next().unwrap().chars().next().unwrap(),
        );
    }

    (initial, transitions)
}

fn inc_in_map<K, V>(map: &mut HashMap<K, V>, key: &K, by: V)
where
    K: Eq + Hash + Clone,
    V: AddAssign,
{
    if let Some(value) = map.get_mut(key) {
        *value += by;
    } else {
        map.insert(key.clone(), by);
    }
}

fn solve(
    initial: &str,
    transitions: &HashMap<&str, char>,
    steps: usize,
) -> i64 {
    let mut pairs = HashMap::new();
    let initial = Vec::from_iter(initial.chars());

    for window in initial.windows(2) {
        let window = String::from_iter(window);
        inc_in_map(&mut pairs, &window, 1);
    }

    for _ in 0..steps {
        let mut new_pairs = HashMap::new();

        for (pair, &count) in &pairs {
            let transition = transitions.get(pair.as_str()).unwrap();

            let pair1 =
                String::from_iter([pair.chars().next().unwrap(), *transition]);
            let pair2 =
                String::from_iter([*transition, pair.chars().nth(1).unwrap()]);

            inc_in_map(&mut new_pairs, &pair1, count);
            inc_in_map(&mut new_pairs, &pair2, count);
        }

        std::mem::swap(&mut new_pairs, &mut pairs);
    }

    let mut letter_count = HashMap::new();

    letter_count.insert(*initial.last().unwrap(), 1);

    for (pair, &count) in &pairs {
        let letter = pair.chars().next().unwrap();
        inc_in_map(&mut letter_count, &letter, count);
    }

    let max = letter_count
        .iter()
        .max_by(|(_, v1), (_, v2)| v1.cmp(v2))
        .unwrap()
        .1;
    let min = letter_count
        .iter()
        .min_by(|(_, v1), (_, v2)| v1.cmp(v2))
        .unwrap()
        .1;

    max - min
}

fn main() {
    let input = fs::read_to_string(input_file("input14.txt")).unwrap();
    let (init, trans) = parse(&input);
    println!("part1 = {}", solve(init, &trans, 10));
    println!("part2 = {}", solve(init, &trans, 40));
}

#[test]
fn test_day14() {
    let input = "\
NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C";
    let (init, trans) = parse(input);

    assert_eq!(solve(init, &trans, 10), 1588);
}
