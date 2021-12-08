use aoc_2021::input_file;
use std::{
    collections::{HashMap, HashSet},
    fs,
};

#[derive(Debug)]
struct Line {
    signals: Vec<String>,
    output: Vec<String>,
}

fn parse(lines: &str) -> Vec<Line> {
    let mut result = vec![];

    for line in lines.lines() {
        let parts = line.split(" | ").collect::<Vec<&str>>();
        let left = parts[0];
        let right = parts[1];

        result.push(Line {
            signals: left.split_ascii_whitespace().map(str::to_owned).collect(),
            output: right.split_ascii_whitespace().map(str::to_owned).collect(),
        });
    }

    result
}

fn part1(lines: &[Line]) -> usize {
    fn count_unique(line: &Line) -> usize {
        line.output
            .iter()
            .map(String::len)
            .filter(|&size| size == 2 || size == 3 || size == 4 || size == 7)
            .count()
    }

    lines.iter().map(count_unique).sum()
}

fn deduce(line: &Line) -> usize {
    let mut number_map = HashMap::new();
    let mut six_segments = vec![];
    let mut five_segments = vec![];

    for signal in &line.signals {
        let set = HashSet::<char>::from_iter(signal.chars());
        match signal.len() {
            2 => {
                number_map.insert(1, set);
            }
            3 => {
                number_map.insert(7, set);
            }
            4 => {
                number_map.insert(4, set);
            }
            7 => {
                number_map.insert(8, set);
            }
            6 => {
                six_segments.push(set);
            }
            5 => {
                five_segments.push(set);
            }
            _ => (),
        }
    }

    for number in six_segments {
        if number.intersection(number_map.get(&1).unwrap()).count() == 1 {
            number_map.insert(6, number);
        } else if number.intersection(number_map.get(&4).unwrap()).count() == 3
        {
            number_map.insert(0, number);
        } else {
            number_map.insert(9, number);
        }
    }

    for number in five_segments {
        if number.intersection(number_map.get(&1).unwrap()).count() == 2 {
            number_map.insert(3, number);
        } else if number.intersection(number_map.get(&6).unwrap()).count() == 5
        {
            number_map.insert(5, number);
        } else {
            number_map.insert(2, number);
        }
    }

    let mut result = 0;

    for output in &line.output {
        for number in 0..=9 {
            if number_map.get(&number)
                == Some(&HashSet::<char>::from_iter(output.chars()))
            {
                result = result * 10 + number;
                break;
            }
        }
    }

    result
}

fn part2(lines: &[Line]) -> usize {
    lines.iter().map(deduce).sum()
}

fn main() {
    let input = fs::read_to_string(input_file("input08.txt")).unwrap();
    let input = parse(&input);

    println!("part1 = {}", part1(&input));
    println!("part2 = {}", part2(&input));
}

#[test]
fn test_day08() {
    let input = "\
be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce";
    let input = parse(input);

    assert_eq!(part1(&input), 26);

    let input2 =
        "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf";
    let input2 = parse(&input2);
    assert_eq!(deduce(&input2[0]), 5353)
}
