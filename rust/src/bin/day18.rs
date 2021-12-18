use aoc_2021::input_file;
use std::fs;

#[derive(Clone, Debug, PartialEq)]
enum Element {
    Open,
    Close,
    Number(i32),
}

fn parse(input: &str) -> Vec<Element> {
    let mut elements = vec![];
    let mut input = input.chars().peekable();
    while let Some(c) = input.next() {
        match c {
            '[' => elements.push(Element::Open),
            ']' => elements.push(Element::Close),
            n if n.is_digit(10) => {
                let mut num_str = String::new();
                num_str.push(n);
                while let Some(&c) = input.peek() {
                    if c.is_digit(10) {
                        input.next();
                        num_str.push(c);
                    } else {
                        break;
                    }
                }

                elements.push(Element::Number(num_str.parse().unwrap()))
            }
            _ => (),
        }
    }

    elements
}

fn explode(line: &[Element]) -> (bool, Vec<Element>) {
    let mut depth = 0;
    let mut elements = vec![];
    let mut carry = None;
    let mut last_number = None;
    let mut did_explode = false;

    let mut line = line.iter().peekable();

    while let Some(c) = line.next() {
        if depth == 5 && !did_explode {
            if let (Element::Number(l), Some(Element::Number(r))) =
                (c, line.peek())
            {
                elements.pop(); // remove [
                if let Some(idx) = last_number {
                    if let Some(Element::Number(n)) = elements.get_mut(idx) {
                        *n += l;
                    }
                }

                carry = Some(r);
                elements.push(Element::Number(0));
                line.next(); // skip r
                line.next(); // skip ]
                did_explode = true;
                continue;
            }
        }

        match c {
            Element::Open => {
                depth += 1;
                elements.push(Element::Open);
            }
            Element::Close => {
                depth -= 1;
                elements.push(Element::Close);
            }
            Element::Number(n) => {
                if let Some(m) = carry {
                    elements.push(Element::Number(n + m));
                    carry = None;
                } else {
                    elements.push(Element::Number(*n))
                }

                last_number = Some(elements.len() - 1);
            }
        }
    }

    (did_explode, elements)
}

fn split(line: &[Element]) -> (bool, Vec<Element>) {
    let mut elements = vec![];
    let mut did_split = false;

    for c in line {
        match c {
            Element::Number(n) => {
                if *n >= 10 && !did_split {
                    elements.push(Element::Open);
                    elements.push(Element::Number(n / 2));
                    elements.push(Element::Number((n + 1) / 2));
                    elements.push(Element::Close);
                    did_split = true;
                } else {
                    elements.push(Element::Number(*n));
                }
            }
            e => elements.push(e.clone()),
        }
    }

    (did_split, elements)
}

fn reduce(line: &[Element]) -> Vec<Element> {
    let mut e = line.to_vec();
    loop {
        let (did_explode, elements) = explode(&e);
        e = elements;

        if did_explode {
            continue;
        }

        let (did_split, elements) = split(&e);
        e = elements;

        if !did_split {
            break;
        }
    }

    e
}

fn add(left: &[Element], right: &[Element]) -> Vec<Element> {
    let mut sum = vec![Element::Open];
    sum.append(&mut left.to_vec());
    sum.append(&mut right.to_vec());
    sum.push(Element::Close);

    reduce(&sum)
}

fn magnitude(num: &[Element]) -> u64 {
    fn helper(it: &mut dyn Iterator<Item = &Element>) -> u64 {
        if let Some(e) = it.next() {
            match e {
                Element::Open => {
                    let res = 3 * helper(it) + 2 * helper(it);
                    it.next(); // skip ]
                    res
                }
                Element::Number(n) => *n as u64,
                Element::Close => unreachable!(),
            }
        } else {
            0
        }
    }

    helper(&mut num.iter())
}

fn part1(puzzle: &[Vec<Element>]) -> u64 {
    let sum = puzzle.to_vec().into_iter().reduce(|acc, e| add(&acc, &e));

    magnitude(&sum.unwrap())
}

fn part2(puzzle: &[Vec<Element>]) -> u64 {
    let mut max = 0;
    for left in puzzle {
        for right in puzzle {
            if left != right {
                max = std::cmp::max(magnitude(&add(left, right)), max);
            }
        }
    }

    max
}

fn main() {
    let input = fs::read_to_string(input_file("input18.txt")).unwrap();
    let input = input.lines().map(parse).collect::<Vec<_>>();
    println!("part1 = {}", part1(&input));
    println!("part2 = {}", part2(&input));
}

#[test]
fn test_day18() {
    let input = "\
[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
[[[5,[2,8]],4],[5,[[9,9],0]]]
[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
[[[[5,4],[7,7]],8],[[8,3],8]]
[[9,3],[[9,9],[6,[4,9]]]]
[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]"
        .lines()
        .map(parse)
        .collect::<Vec<_>>();

    assert_eq!(part1(&input), 4140);
    assert_eq!(part2(&input), 3993);
}
