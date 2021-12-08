use aoc_2021::input_file;
use std::{collections::HashMap, fs};

#[derive(Debug, PartialEq)]
struct Line {
    begin: (i32, i32),
    end: (i32, i32),
}

fn parse(input: &str) -> Vec<Line> {
    let mut lines = vec![];
    for line in input.lines() {
        let points = line.split(" -> ").collect::<Vec<_>>();
        let begin = points[0];
        let end = points[1];

        let begin: Vec<i32> =
            begin.split(',').flat_map(|n| n.parse()).collect();
        let end: Vec<i32> =
            end.split(',').flat_map(|n| n.parse()).collect();

        lines.push(Line {
            begin: (begin[0], begin[1]),
            end: (end[0], end[1]),
        })
    }

    lines
}

fn line_to_points(line: &Line, diagonal: bool) -> Vec<(i32, i32)> {
    let mut points = vec![];
    let mut current_point = line.begin;

    if !diagonal && line.begin.0 != line.end.0 && line.begin.1 != line.end.1 {
        return points;
    }

    while current_point != line.end {
        points.push(current_point);

        if current_point.0 < line.end.0 {
            current_point.0 += 1;
        }

        if current_point.0 > line.end.0 {
            current_point.0 -= 1;
        }

        if current_point.1 < line.end.1 {
            current_point.1 += 1;
        }

        if current_point.1 > line.end.1 {
            current_point.1 -= 1;
        }
    }

    points.push(current_point);

    points
}

fn solution(lines: &[Line], diagonal: bool) -> usize {
    let mut field = HashMap::new();

    for line in lines {
        for point in line_to_points(line, diagonal) {
            if let Some(&value) = field.get(&point) {
                field.insert(point, value + 1);
            } else {
                field.insert(point, 1);
            }
        }
    }

    field.iter().filter(|(_, value)| **value > 1).count()
}

fn main() {
    let input = fs::read_to_string(input_file("input05.txt")).unwrap();

    println!("part1 = {}", solution(&parse(&input), false));
    println!("part2 = {}", solution(&parse(&input), true));
}

#[test]
fn test_day05() {
    let input = "0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2";

    assert_eq!(solution(&parse(input), false), 5);
    assert_eq!(solution(&parse(input), true), 12);
}
