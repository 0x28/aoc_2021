use aoc_2021::input_file;
use std::fs;

fn parse(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|l| l.chars().collect()).collect()
}

fn step(grid: &mut Vec<Vec<char>>) -> bool {
    let width = grid[0].len();
    let height = grid.len();
    let mut moved = false;

    let mut new_grid = grid.clone();
    for (y, row) in grid.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            if *cell == '>' {
                let (nx, ny) = ((x + 1) % width, y);
                if grid[ny][nx] == '.' {
                    new_grid[y][x] = '.';
                    new_grid[ny][nx] = '>';
                    moved = true;
                }
            }
        }
    }

    std::mem::swap(&mut new_grid, grid);
    new_grid = grid.clone();

    for (y, row) in grid.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            if *cell == 'v' {
                let (nx, ny) = (x, (y + 1) % height);
                if grid[ny][nx] == '.' {
                    new_grid[y][x] = '.';
                    new_grid[ny][nx] = 'v';
                    moved = true;
                }
            }
        }
    }
    std::mem::swap(&mut new_grid, grid);

    moved
}

fn solve(grid: &[Vec<char>]) -> i64 {
    let mut grid = grid.to_vec();

    for i in 1.. {
        if !step(&mut grid) {
            return i;
        }
    }

    unreachable!()
}

fn main() {
    let input = fs::read_to_string(input_file("input25.txt")).unwrap();
    let input = parse(&input);
    println!("part1 = {}", solve(&input));
}

#[test]
fn test_day25() {
    let input = "\
v...>>.vv>
.vv>>.vv..
>>.>v>...v
>>v>>.>.v.
v>v.vv.v..
>.>>..v...
.vv..>.>v.
v.v..>>v.v
....v..v.>";
    let input = parse(input);

    assert_eq!(solve(&input), 58);
}
