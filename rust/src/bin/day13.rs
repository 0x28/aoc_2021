use std::{collections::HashSet, fs};

use aoc_2021::input_file;

#[derive(Debug, PartialEq)]
struct Instructions {
    points: Vec<(i32, i32)>,
    folds: Vec<(char, i32)>,
}

fn parse(input: &str) -> Instructions {
    let parts = input.split("\n\n").collect::<Vec<_>>();
    let top = parts[0];
    let bottom = parts[1];
    let mut points = vec![];

    for line in top.lines() {
        let p = line.split(',').collect::<Vec<_>>();
        points.push((p[0].parse().unwrap(), p[1].parse().unwrap()));
    }

    let mut folds = vec![];

    for line in bottom.lines() {
        if line.contains('y') {
            folds.push(('y', line.split('=').nth(1).unwrap().parse().unwrap()));
        } else if line.contains('x') {
            folds.push(('x', line.split('=').nth(1).unwrap().parse().unwrap()));
        }
    }

    Instructions { points, folds }
}

fn part1(inst: &Instructions) -> usize {
    let mut points = HashSet::new();

    for p in &inst.points {
        match inst.folds[0].0 {
            'x' => {
                if p.0 > inst.folds[0].1 {
                    let mut p_mirror = *p;
                    p_mirror.0 -= 2 * (p_mirror.0 - inst.folds[0].1);
                    points.insert(p_mirror);
                } else {
                    points.insert(*p);
                }
            }
            'y' => {
                if p.1 > inst.folds[0].1 {
                    let mut p_mirror = *p;
                    p_mirror.1 -= 2 * (p_mirror.1 - inst.folds[0].1);
                    points.insert(p_mirror);
                } else {
                    points.insert(*p);
                }
            }
            _ => {}
        }
    }

    points.len()
}

fn part2(inst: &Instructions) {
    let mut points = HashSet::from_iter(inst.points.clone());

    for (dir, fold) in &inst.folds {
        let mut new_points = points.clone();

        for p in &points {
            match dir {
                'x' => {
                    if p.0 > *fold {
                        let mut p_mirror = *p;
                        p_mirror.0 -= 2 * (p_mirror.0 - fold);
                        new_points.insert(p_mirror);
                        new_points.remove(p);
                    } else {
                        new_points.insert(*p);
                    }
                }
                'y' => {
                    if p.1 > *fold {
                        let mut p_mirror = *p;
                        p_mirror.1 -= 2 * (p_mirror.1 - fold);
                        new_points.insert(p_mirror);
                        new_points.remove(p);
                    } else {
                        new_points.insert(*p);
                    }
                }
                _ => (),
            }
        }

        std::mem::swap(&mut points, &mut new_points);
    }

    display_points(&points);
}

fn display_points(points: &HashSet<(i32, i32)>) {
    let max_x = points
        .iter()
        .max_by(|(x1, _), (x2, _)| x1.cmp(x2))
        .unwrap()
        .0;
    let max_y = points
        .iter()
        .max_by(|(_, y1), (_, y2)| y1.cmp(y2))
        .unwrap()
        .1;

    for y in 0..=max_y {
        for x in 0..=max_x {
            if points.contains(&(x, y)) {
                print!("▇");
            } else {
                print!(" ");
            }
        }
        println!();
    }
}

fn main() {
    let input = fs::read_to_string(input_file("input13.txt")).unwrap();
    let input = parse(&input);
    println!("part1 = {}", part1(&input));
    println!("part2 = ");
    part2(&input);
}

#[test]
fn test_day13() {
    let input = "\
6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5";
    let input = parse(input);

    assert_eq!(part1(&input), 17);
    part2(&input);
}
