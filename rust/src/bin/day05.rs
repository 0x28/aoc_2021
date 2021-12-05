use aoc_2021::input_file;
use std::{
    cmp::{max, min},
    collections::HashMap,
    fs,
};

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
            begin.split(',').map(|n| n.parse().unwrap()).collect();
        let end: Vec<i32> =
            end.split(',').map(|n| n.parse().unwrap()).collect();

        lines.push(Line {
            begin: (begin[0], begin[1]),
            end: (end[0], end[1]),
        })
    }

    lines
}

fn line_to_points(line: &Line, diagonal: bool) -> Vec<(i32, i32)> {
    let mut points = vec![];

    if line.begin.0 == line.end.0 {
        let begin = min(line.begin.1, line.end.1);
        let end = max(line.begin.1, line.end.1);
        for y in begin..=end {
            points.push((line.begin.0, y))
        }
    } else if line.begin.1 == line.end.1 {
        let begin = min(line.begin.0, line.end.0);
        let end = max(line.begin.0, line.end.0);
        for x in begin..=end {
            points.push((x, line.begin.1))
        }
    } else if diagonal {
        let begin_x;
        let end_x;
        let begin_y;
        let end_y;

        if line.begin.0 < line.end.0 {
            begin_x = line.begin.0;
            begin_y = line.begin.1;
            end_x = line.end.0;
            end_y = line.end.1;
        } else {
            begin_x = line.end.0;
            begin_y = line.end.1;
            end_x = line.begin.0;
            end_y = line.begin.1;
        }

        let step = if begin_y < end_y { 1 } else { -1 };

        for (i, x) in (begin_x..=end_x).enumerate() {
            let y = (i as i32) * step;
            points.push((x, begin_y + y))
        }
    }

    points
}

fn solution(lines: &[Line], diagonal: bool) -> usize {
    let mut field = HashMap::new();

    for line in lines {
        for point in line_to_points(&line, diagonal) {
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
fn test_part1() {
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
