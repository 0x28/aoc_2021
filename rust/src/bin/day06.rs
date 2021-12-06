use aoc_2021::input_file;
use std::{collections::HashMap, fs};

fn parse(input: &str) -> Vec<u64> {
    input.split(',').flat_map(|s| s.trim().parse()).collect()
}

fn increment_population(
    fish: u64,
    by: u64,
    population: &mut HashMap<u64, u64>,
) {
    if let Some(&value) = population.get(&fish) {
        population.insert(fish, value + by);
    } else {
        population.insert(fish, by);
    }
}

fn part1(init: &[u64], iterations: usize) -> u64 {
    let mut population = HashMap::new();

    for fish in init {
        increment_population(*fish, 1, &mut population);
    }
    for _ in 0..iterations {
        let mut new_population = HashMap::new();
        for (fish, number) in &population {
            match fish {
                0 => {
                    increment_population(8, *number, &mut new_population);
                    increment_population(6, *number, &mut new_population);
                }
                n => increment_population(
                    n - 1,
                    *number,
                    &mut &mut new_population,
                ),
            }
        }

        std::mem::swap(&mut new_population, &mut population);
    }

    population.iter().map(|(_, number)| number).sum()
}

fn main() {
    let input = fs::read_to_string(input_file("input06.txt")).unwrap();
    let fish = parse(&input);

    println!("part1 = {}", part1(&fish, 80));
    println!("part2 = {}", part1(&fish, 256));
}

#[test]
fn test_day06() {
    let input = "3,4,3,1,2";
    let init = parse(input);
    assert_eq!(part1(&init, 18), 26);
    assert_eq!(part1(&init, 80), 5934);
}
