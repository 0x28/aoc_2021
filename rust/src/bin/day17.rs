use aoc_2021::input_file;
use std::fs;

struct Area {
    x1: i64,
    x2: i64,
    y1: i64,
    y2: i64,
}

fn parse(input: &str) -> Area {
    let mut num_str = String::new();
    let mut nums = vec![];
    for c in input.chars() {
        if c.is_numeric() || c == '-' {
            num_str.push(c);
        } else if !num_str.is_empty() {
            nums.push(num_str.parse::<i64>().unwrap());
            num_str.clear();
        }
    }

    nums.push(num_str.parse().unwrap());

    Area {
        x1: nums[0],
        x2: nums[1],
        y1: nums[2],
        y2: nums[3],
    }
}

fn step(pos: &mut (i64, i64), velocity: &mut (i64, i64)) {
    pos.0 += velocity.0;
    pos.1 += velocity.1;

    match velocity.0.cmp(&0) {
        std::cmp::Ordering::Greater => velocity.0 -= 1,
        std::cmp::Ordering::Less => velocity.0 += 1,
        std::cmp::Ordering::Equal => (),
    }

    velocity.1 -= 1;
}

fn in_area(area: &Area, (x, y): (i64, i64)) -> bool {
    (area.x1..=area.x2).contains(&x) && (area.y1..=area.y2).contains(&y)
}

fn part1(area: &Area) -> i64 {
    area.y1 * (area.y1 + 1) / 2
}

fn part2(area: &Area) -> i64 {
    let min_vx = 1;
    let max_vx = area.x2;
    let min_vy = std::cmp::min(area.y1, -area.y1);
    let max_vy = std::cmp::max(area.y1, -area.y1);
    let mut count = 0;

    for vx in min_vx..=max_vx {
        for vy in min_vy..=max_vy {
            let mut pos = (0, 0);
            let mut velocity = (vx, vy);

            while pos.0 <= area.x2 && pos.1 >= area.y1 {
                if in_area(area, pos) {
                    count += 1;
                    break;
                }

                step(&mut pos, &mut velocity);
            }
        }
    }

    count
}

fn main() {
    let input = fs::read_to_string(input_file("input17.txt")).unwrap();
    let input = input.trim();
    let input = parse(input);
    println!("part1 = {}", part1(&input));
    println!("part2 = {}", part2(&input));
}

#[test]
fn test_day() {
    let input = "target area: x=20..30, y=-10..-5";
    let input = parse(input);

    assert_eq!(part1(&input), 45);
    assert_eq!(part2(&input), 112);
}
