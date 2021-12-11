use std::fs;

use aoc_2021::input_file;

fn parse(input: &str) -> Vec<Vec<(bool, u32)>> {
    input
        .lines()
        .map(|l| {
            l.chars()
                .flat_map(|c| c.to_digit(10))
                .map(|d| (false, d))
                .collect()
        })
        .collect()
}

fn step(grid: &mut Vec<Vec<(bool, u32)>>, x: usize, y: usize) {
    if y >= grid.len() || x >= grid[0].len() || grid[y][x].0 {
        return;
    }

    let mut octopus = &mut grid[y][x];

    octopus.1 += 1;
    if octopus.1 > 9 {
        octopus.1 = 0;
        octopus.0 = true;

        step(grid, x + 1, y);
        step(grid, x + 1, y + 1);
        step(grid, x + 1, y.wrapping_sub(1));
        step(grid, x.wrapping_sub(1), y);
        step(grid, x.wrapping_sub(1), y + 1);
        step(grid, x.wrapping_sub(1), y.wrapping_sub(1));
        step(grid, x, y + 1);
        step(grid, x, y.wrapping_sub(1));
    }
}

#[allow(clippy::ptr_arg)]
fn solve(grid: &Vec<Vec<(bool, u32)>>) -> (usize, usize) {
    let mut current = grid.clone();
    let mut next = current.clone();
    let mut flashes = 0;

    for i in 0.. {
        if next.iter().all(|r| r.iter().all(|o| o.1 == 0)) {
            return (flashes, i);
        }

        for (y, row) in current.iter().enumerate() {
            for (x, _) in row.iter().enumerate() {
                step(&mut next, x, y);
            }
        }

        for row in &mut next {
            for octopus in row {
                if octopus.0 && i < 100 {
                    flashes += 1;
                }
                octopus.0 = false;
            }
        }

        current = next.clone();
    }

    unreachable!()
}

fn main() {
    let input = parse(&fs::read_to_string(input_file("input11.txt")).unwrap());
    let (part1, part2) = solve(&input);

    println!("part1 = {}", part1);
    println!("part2 = {}", part2);
}

#[test]
fn test_day11() {
    let input = "\
5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526";
    let grid = parse(input);

    assert_eq!(solve(&grid).0, 1656);
}
