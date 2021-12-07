use aoc_2021::input_file;
use std::cmp::min;
use std::fs;

fn parse(input: &str) -> Vec<i64> {
    input.split(',').flat_map(|n| n.trim().parse()).collect()
}

fn part1(pos: &[i64]) -> i64 {
    let max = *pos.iter().max().unwrap();
    let mut min_fuel = i64::MAX;

    for i in 0..=max {
        min_fuel = min(pos.iter().map(|n| (n - i).abs()).sum(), min_fuel);
    }

    min_fuel
}

fn part2(pos: &[i64]) -> i64 {
    let max = *pos.iter().max().unwrap();
    let mut min_fuel = i64::MAX;

    for i in 0..=max {
        min_fuel = min(
            pos.iter()
                .map(|p| {
                    let n = (p - i).abs();

                    n * (n + 1) / 2
                })
                .sum(),
            min_fuel,
        );
    }

    min_fuel
}

fn main() {
    let input = fs::read_to_string(input_file("input07.txt")).unwrap();
    let input = parse(&input);

    println!("part1 = {}", part1(&input));
    println!("part2 = {}", part2(&input));
}

#[test]
fn test_day07() {
    let input = "16,1,2,0,4,2,7,1,2,14";
    let input = parse(input);

    assert_eq!(part1(&input), 37);
    assert_eq!(part2(&input), 168);
}
