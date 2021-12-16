use aoc_2021::input_file;
use std::{cmp::Ordering, fs};

#[derive(Debug, PartialEq)]
enum Payload {
    Literal(Vec<u8>),
    SubPackets(Vec<Packet>),
}

#[derive(Debug, PartialEq)]
struct Packet {
    size: usize,
    version: u8,
    type_id: u8,
    payload: Payload,
}

impl Packet {
    fn new(size: usize, version: u8, type_id: u8, payload: Payload) -> Self {
        Self {
            size,
            version,
            type_id,
            payload,
        }
    }
}

fn one_zero(num: u8) -> u8 {
    if num > 0 {
        1
    } else {
        0
    }
}

fn parse(input: &str) -> Packet {
    let mut bits = vec![];
    for hex in input.chars() {
        let hex = u8::from_str_radix(&hex.to_string(), 16).unwrap();
        bits.extend_from_slice(&[
            one_zero(hex & 0x8),
            one_zero(hex & 0x4),
            one_zero(hex & 0x2),
            one_zero(hex & 0x1),
        ]);
    }

    parse_package(&bits)
}

fn parse_package(bits: &[u8]) -> Packet {
    let mut packet = Packet::new(6, 0, 0, Payload::Literal(vec![]));
    if let [v1, v2, v3, t1, t2, t3, rest @ ..] = bits {
        packet.version = v1 << 2 | v2 << 1 | v3;
        packet.type_id = t1 << 2 | t2 << 1 | t3;
        if packet.type_id == 4 {
            let (len, literal) = parse_literal(rest);
            packet.size += len;
            packet.payload = Payload::Literal(literal);
        } else {
            let (len, subpackets) = parse_subpackets(rest);
            packet.size +=
                subpackets.iter().map(|p| p.size).sum::<usize>() + len;
            packet.payload = Payload::SubPackets(subpackets);
        }
    }

    packet
}

fn parse_literal(bits: &[u8]) -> (usize, Vec<u8>) {
    let mut literal = vec![];
    let mut size = 0;
    for chunk in bits.chunks(5) {
        size += 5;
        literal.extend_from_slice(&chunk[1..]);
        if chunk[0] == 0 {
            break;
        }
    }

    (size, literal)
}

fn bits_to_number(bits: &[u8]) -> usize {
    let mut num = 0;
    for (shift, bit) in bits.iter().rev().enumerate() {
        num |= (*bit as usize) << shift;
    }

    num
}

fn parse_subpackets(bits: &[u8]) -> (usize, Vec<Packet>) {
    let mut packets = vec![];
    let length_size;
    if bits[0] == 0 {
        let mut b = bits_to_number(&bits[1..=15]);
        let mut bit_window = &bits[16..];

        while b != 0 {
            let packet = parse_package(bit_window);
            b -= packet.size;
            bit_window = &bit_window[packet.size..];
            packets.push(packet);
        }
        length_size = 16;
    } else {
        let n = bits_to_number(&bits[1..=11]);
        let mut bit_window = &bits[12..];

        for _ in 0..n {
            let packet = parse_package(bit_window);
            bit_window = &bit_window[packet.size..];
            packets.push(packet);
        }
        length_size = 12;
    }

    (length_size, packets)
}

fn part1(packet: &Packet) -> i64 {
    packet.version as i64
        + match &packet.payload {
            Payload::Literal(_) => 0,
            Payload::SubPackets(packets) => packets.iter().map(part1).sum(),
        }
}

fn part2(packet: &Packet) -> usize {
    match &packet.payload {
        Payload::Literal(bits) => bits_to_number(bits),
        Payload::SubPackets(sub) => match packet.type_id {
            0 => sub.iter().map(part2).sum(),
            1 => sub.iter().map(part2).product(),
            2 => sub.iter().map(part2).min().unwrap(),
            3 => sub.iter().map(part2).max().unwrap(),
            5 | 6 | 7 => {
                if let [first, second] = &sub[..] {
                    match part2(first).cmp(&part2(second)) {
                        Ordering::Less => {
                            if packet.type_id == 6 {
                                1
                            } else {
                                0
                            }
                        }
                        Ordering::Equal => {
                            if packet.type_id == 7 {
                                1
                            } else {
                                0
                            }
                        }
                        Ordering::Greater => {
                            if packet.type_id == 5 {
                                1
                            } else {
                                0
                            }
                        }
                    }
                } else {
                    unreachable!()
                }
            }
            _ => unreachable!(),
        },
    }
}

fn main() {
    let input = fs::read_to_string(input_file("input16.txt")).unwrap();
    let input = input.trim();
    let input = parse(input);
    println!("part1 = {}", part1(&input));
    println!("part2 = {}", part2(&input));
}

#[test]
fn test_day16() {
    let input = parse("8A004A801A8002F478");
    assert_eq!(part1(&input), 16);
    let input = parse("620080001611562C8802118E34");
    assert_eq!(part1(&input), 12);
    let input = parse("C0015000016115A2E0802F182340");
    assert_eq!(part1(&input), 23);
    let input = parse("A0016C880162017C3686B18A3D4780");
    assert_eq!(part1(&input), 31);

    let input = parse("C200B40A82");
    assert_eq!(part2(&input), 3);
    let input = parse("04005AC33890");
    assert_eq!(part2(&input), 54);
    let input = parse("880086C3E88112");
    assert_eq!(part2(&input), 7);
    let input = parse("CE00C43D881120");
    assert_eq!(part2(&input), 9);
    let input = parse("D8005AC2A8F0");
    assert_eq!(part2(&input), 1);
    let input = parse("F600BC2D8F");
    assert_eq!(part2(&input), 0);
    let input = parse("9C005AC2F8F0");
    assert_eq!(part2(&input), 0);
    let input = parse("9C0141080250320F1802104A08");
    assert_eq!(part2(&input), 1);
}
