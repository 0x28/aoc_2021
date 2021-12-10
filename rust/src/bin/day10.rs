use std::fs;

use aoc_2021::input_file;

fn parse(input: &str) -> Vec<&str> {
    input.lines().collect()
}

fn check(line: &&str) -> Result<Vec<char>, i64> {
    let mut stack = Vec::new();

    for c in line.chars() {
        match c {
            '(' | '[' | '{' | '<' => stack.push(c),
            ')' => {
                if stack.last() != Some(&'(') {
                    return Err(3);
                }
                stack.pop();
            }
            ']' => {
                if stack.last() != Some(&'[') {
                    return Err(57);
                }
                stack.pop();
            }
            '}' => {
                if stack.last() != Some(&'{') {
                    return Err(1197);
                }
                stack.pop();
            }
            '>' => {
                if stack.last() != Some(&'<') {
                    return Err(25137);
                }
                stack.pop();
            }
            _ => (),
        }
    }

    Ok(stack)
}

fn part1(lines: &[&str]) -> i64 {
    fn errors(result: Result<Vec<char>, i64>) -> i64 {
        match result {
            Err(n) => n,
            _ => 0,
        }
    }
    lines.iter().map(check).map(errors).sum()
}

fn part2(lines: &[&str]) -> i64 {
    let missing = lines.iter().flat_map(check).collect::<Vec<_>>();

    fn score(sum: i64, c: &char) -> i64 {
        sum * 5
            + match c {
                '(' => 1,
                '[' => 2,
                '{' => 3,
                '<' => 4,
                _ => 0,
            }
    }

    let mut missing = missing
        .iter()
        .map(|stack| stack.iter().rev().fold(0, score))
        .collect::<Vec<_>>();

    missing.sort_unstable();

    missing[missing.len()/2]
}

fn main() {
    let input = fs::read_to_string(input_file("input10.txt")).unwrap();
    let input = parse(&input);

    println!("part1 = {}", part1(&input));
    println!("part2 = {}", part2(&input));
}

#[test]
fn test_day10() {
    let input = "\
[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]";
    let input = parse(input);

    assert_eq!(part1(&input), 26397);
    assert_eq!(part2(&input), 288957);
}
