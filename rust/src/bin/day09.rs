use aoc_2021::input_file;
use std::{collections::HashSet, fs};

fn parse(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect()
}

fn neighbors(grid: &[Vec<u32>], x: usize, y: usize) -> Vec<(usize, usize)> {
    [
        (x, y + 1),
        (x + 1, y),
        (x.wrapping_sub(1), y),
        (x, y.wrapping_sub(1)),
    ]
    .iter()
    .filter(|(x, y)| {
        if let Some(row) = grid.get(*y) {
            row.get(*x).is_some()
        } else {
            false
        }
    })
    .copied()
    .collect()
}

fn part1(grid: &[Vec<u32>]) -> (i64, Vec<(usize, usize)>) {
    let mut lows = vec![];
    let mut low_points = vec![];
    for (y, row) in grid.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            if neighbors(grid, x, y)
                .iter()
                .all(|(x, y)| grid[*y][*x] > *cell)
            {
                lows.push(cell);
                low_points.push((x, y));
            }
        }
    }

    (lows.iter().map(|&n| (n + 1) as i64).sum(), low_points)
}

fn part2(grid: &[Vec<u32>], low_points: &[(usize, usize)]) -> usize {
    let mut basin = vec![];

    for low in low_points {
        let mut seen = HashSet::<(usize, usize)>::new();
        let mut old_size = 0;

        seen.insert(*low);

        while old_size != seen.len() {
            old_size = seen.len();
            for (x, y) in seen.clone() {
                for pos @ (n_x, n_y) in neighbors(grid, x, y) {
                    if seen.contains(&pos) {
                        continue;
                    }
                    let neighbor = grid[n_y][n_x];
                    if grid[y][x] < neighbor && neighbor < 9 {
                        seen.insert((n_x, n_y));
                    }
                }
            }
        }

        basin.push(seen.len());
    }

    basin.sort_unstable();
    basin.iter().rev().take(3).product()
}

fn main() {
    let input = fs::read_to_string(input_file("input09.txt")).unwrap();
    let input = parse(&input);
    let (p1, lows) = part1(&input);
    println!("part1 = {}", p1);
    println!("part2 = {}", part2(&input, &lows));
}

#[test]
fn test_day09() {
    let input = "\
2199943210
3987894921
9856789892
8767896789
9899965678";

    let input = parse(input);
    let (p1, lows) = part1(&input);
    assert_eq!(p1, 15);

    assert_eq!(part2(&input, &lows), 1134);
}
