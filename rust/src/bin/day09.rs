use aoc_2021::input_file;
use std::{collections::HashSet, fs};

fn parse(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect()
}

fn safe_cell(grid: &[Vec<u32>], x: usize, y: usize) -> Option<&u32> {
    grid.get(y).map(|r| r.get(x)).flatten()
}

fn part1(grid: &[Vec<u32>]) -> (i64, Vec<(usize, usize)>) {
    let mut lows = vec![];
    let mut low_points = vec![];
    for (y, row) in grid.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            if let Some(top) = safe_cell(grid, x, y.wrapping_sub(1)) {
                if top <= cell {
                    continue;
                }
            }
            if let Some(left) = safe_cell(grid, x.wrapping_sub(1), y) {
                if left <= cell {
                    continue;
                }
            }
            if let Some(right) = safe_cell(grid, x + 1, y) {
                if right <= cell {
                    continue;
                }
            }
            if let Some(bottom) = safe_cell(grid, x, y + 1) {
                if bottom <= cell {
                    continue;
                }
            }

            lows.push(cell);
            low_points.push((x, y));
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
                if let Some(top) = safe_cell(grid, x, y.wrapping_sub(1)) {
                    if top > &grid[y][x] && *top != 9 {
                        seen.insert((x, y - 1));
                    }
                }
                if let Some(left) = safe_cell(grid, x.wrapping_sub(1), y) {
                    if left > &grid[y][x] && *left != 9 {
                        seen.insert((x - 1, y));
                    }
                }
                if let Some(right) = safe_cell(grid, x + 1, y) {
                    if right > &grid[y][x] && *right != 9 {
                        seen.insert((x + 1, y));
                    }
                }
                if let Some(bottom) = safe_cell(grid, x, y + 1) {
                    if bottom > &grid[y][x] && *bottom != 9 {
                        seen.insert((x, y + 1));
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
