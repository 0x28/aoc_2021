use aoc_2021::input_file;
use std::fs;

fn parse(input: &str) -> Vec<usize> {
    input.split(',').flat_map(|s| s.trim().parse()).collect()
}

fn solution(init: &[usize], iterations: usize) -> u64 {
    let mut population = [0; 9];

    for &fish in init {
        population[fish] += 1;
    }
    for _ in 0..iterations {
        let mut new_population = [0; 9];
        for (fish, &number) in population.iter().enumerate() {
            match fish {
                0 => {
                    new_population[8] += number;
                    new_population[6] += number;
                }
                n => new_population[n-1] += number,
            }
        }

        std::mem::swap(&mut new_population, &mut population);
    }

    population.iter().sum()
}

fn main() {
    let input = fs::read_to_string(input_file("input06.txt")).unwrap();
    let fish = parse(&input);

    println!("part1 = {}", solution(&fish, 80));
    println!("part2 = {}", solution(&fish, 256));
}

#[test]
fn test_day06() {
    let input = "3,4,3,1,2";
    let init = parse(input);
    assert_eq!(solution(&init, 18), 26);
    assert_eq!(solution(&init, 80), 5934);
}
