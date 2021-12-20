use aoc_2021::input_file;
use std::{collections::HashMap, fs};

#[derive(Debug, PartialEq)]
struct Trench {
    algorithm: Vec<char>,
    image: Vec<Vec<char>>,
}

fn parse(input: &str) -> Trench {
    let mut split = input.split("\n\n");
    let algorithm = split.next().unwrap();
    let image = split.next().unwrap();

    let algorithm = algorithm.chars().collect();
    let image = image.lines().map(|l| l.chars().collect()).collect();

    Trench { algorithm, image }
}

fn enhance(algorithm: &[char], window: &[char]) -> char {
    let index = window
        .iter()
        .fold(0, |acc, bit| acc * 2 + if *bit == '#' { 1 } else { 0 });

    algorithm[index as usize]
}

fn neighbors(
    image: &HashMap<(i64, i64), char>,
    x: i64,
    y: i64,
    default: char,
) -> Vec<char> {
    let dirs = [-1, 0, 1];
    let mut neighbors = vec![];

    for dir_y in dirs {
        for dir_x in dirs {
            neighbors.push(
                image
                    .get(&(x + dir_x, y + dir_y))
                    .copied()
                    .unwrap_or(default),
            )
        }
    }

    neighbors
}

fn solve(trench: &Trench, steps: usize) -> usize {
    let mut image = HashMap::<(i64, i64), char>::new();
    let mut min_x = 0;
    let mut max_x = trench.image[0].len() as i64;
    let mut min_y = 0;
    let mut max_y = trench.image.len() as i64;

    for (y, row) in trench.image.iter().enumerate() {
        for (x, &pixel) in row.iter().enumerate() {
            image.insert((x as i64, y as i64), pixel);
        }
    }

    let mut default1 = *trench.algorithm.first().unwrap();
    let mut default2 = if default1 == '.' {
        '.'
    } else {
        *trench.algorithm.last().unwrap()
    };

    for _ in 0..steps {
        min_x -= 1;
        min_y -= 1;
        max_x += 1;
        max_y += 1;

        let mut new_image = HashMap::<(i64, i64), char>::new();
        for y in min_y..=max_y {
            for x in min_x..=max_x {
                let n = neighbors(&image, x, y, default2);

                new_image.insert((x, y), enhance(&trench.algorithm, &n));
            }
        }

        std::mem::swap(&mut image, &mut new_image);
        std::mem::swap(&mut default1, &mut default2);
    }

    image.iter().filter(|(_, &pixel)| pixel == '#').count()
}

#[allow(unused)]
fn print_image(
    image: &HashMap<(i64, i64), char>,
    min_x: i64,
    max_x: i64,
    min_y: i64,
    max_y: i64,
    default: char,
) {
    for y in min_y..max_y {
        for x in min_x..max_x {
            print!("{}", image.get(&(x, y)).unwrap_or(&default))
        }

        println!();
    }
}

fn main() {
    let input = fs::read_to_string(input_file("input20.txt")).unwrap();
    let input = parse(&input);
    println!("part1 = {}", solve(&input, 2));
    println!("part2 = {}", solve(&input, 50));
}

#[test]
fn test_day20() {
    let input = "\
..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..##\
#..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###\
.######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#.\
.#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#.....\
.#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#..\
...####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.....\
..##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#

#..#.
#....
##..#
..#..
..###";

    let input = parse(input);

    assert_eq!(solve(&input, 2), 35);
    assert_eq!(solve(&input, 50), 3351);
}
