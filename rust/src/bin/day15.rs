use aoc_2021::input_file;
use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
    fs,
};

fn parse(input: &str) -> Vec<Vec<usize>> {
    input
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect()
        })
        .collect()
}

fn limit(value: usize) -> usize {
    if value > 9 {
        value % 10 + 1
    } else {
        value
    }
}

fn solve(
    width: usize,
    height: usize,
    risk_func: impl Fn(usize, usize) -> usize,
) -> usize {
    let mut risks = HashMap::<(usize, usize), usize>::new();

    let mut heap = BinaryHeap::new();
    let bottom_right = (width - 1, height - 1);

    heap.push(Reverse((0, 0, 0)));

    while let Some(Reverse((risk, x, y))) = heap.pop() {
        if (x, y) == bottom_right {
            return risk;
        }

        if let Some(&old_risk) = risks.get(&(x, y)) {
            if old_risk < risk {
                continue;
            }
        }

        for new_pos @ (new_x, new_y) in [
            (x + 1, y),
            (x, y + 1),
            (x.wrapping_sub(1), y),
            (x, y.wrapping_sub(1)),
        ] {
            if new_x >= width || new_y >= height {
                continue;
            }

            let new_risk = risk + risk_func(new_x, new_y);
            if let Some(old_risk) = risks.get_mut(&new_pos) {
                if new_risk < risk {
                    heap.push(Reverse((new_risk, new_x, new_y)));
                    *old_risk = new_risk;
                }
            } else {
                heap.push(Reverse((new_risk, new_x, new_y)));
                risks.insert(new_pos, new_risk);
            }
        }
    }

    unreachable!()
}

fn part1(puzzle: &[Vec<usize>]) -> usize {
    let width = puzzle[0].len();
    let height = puzzle.len();
    solve(width, height, |x, y| puzzle[y][x])
}

fn part2(puzzle: &[Vec<usize>]) -> usize {
    let tile_width = puzzle[0].len();
    let tile_height = puzzle.len();

    let width = puzzle[0].len() * 5;
    let height = puzzle.len() * 5;

    solve(width, height, |x, y| {
        limit(
            puzzle[y % tile_height][x % tile_width]
                + (x / tile_width) as usize
                + (y / tile_height) as usize,
        )
    })
}

fn main() {
    let input = fs::read_to_string(input_file("input15.txt")).unwrap();
    let input = parse(&input);
    println!("part1 = {}", part1(&input));
    println!("part2 = {}", part2(&input));
}

#[test]
fn test_day15() {
    let input = "\
1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581";

    let input = parse(input);
    // assert_eq!(part1(&input), 40);
    assert_eq!(part2(&input), 315);
}
