use aoc_2021::input_file;
use rustc_hash::FxHashMap;
use std::fs;

enum Operand {
    Variable(usize),
    Literal(i64),
}

enum Instruction {
    Inp(usize),
    Add(usize, Operand),
    Mul(usize, Operand),
    Div(usize, Operand),
    Mod(usize, Operand),
    Eql(usize, Operand),
}

fn register_to_index(register: char) -> usize {
    match register {
        'w' => 0,
        'x' => 1,
        'y' => 2,
        'z' => 3,
        _ => unreachable!(),
    }
}

fn parse(input: &str) -> Vec<Instruction> {
    let mut instructions = vec![];
    for line in input.lines() {
        let token: Vec<_> = line.split_ascii_whitespace().collect();

        let left = token[1].chars().next().map(register_to_index).unwrap();
        let right;
        if token[0] == "inp" {
            right = Operand::Literal(0); // unused
        } else if token[2].chars().all(|c| c == '-' || c.is_digit(10)) {
            right = Operand::Literal(token[2].parse().unwrap())
        } else {
            right = Operand::Variable(
                token[2].chars().next().map(register_to_index).unwrap(),
            );
        };

        let instr = match token[0] {
            "inp" => Instruction::Inp(left),
            "add" => Instruction::Add(left, right),
            "mul" => Instruction::Mul(left, right),
            "div" => Instruction::Div(left, right),
            "mod" => Instruction::Mod(left, right),
            "eql" => Instruction::Eql(left, right),
            _ => unreachable!(),
        };

        instructions.push(instr);
    }

    instructions
}

fn load(registers: &[i64; 4], op: &Operand) -> i64 {
    match op {
        Operand::Variable(var) => registers[*var],
        Operand::Literal(lit) => *lit,
    }
}

fn eval(registers: &mut Registers, instr: &Instruction, input: i64) {
    match instr {
        Instruction::Inp(dest) => {
            registers[*dest] = input;
        }
        Instruction::Add(dest, src) => {
            registers[*dest] += load(registers, src);
        }
        Instruction::Mul(dest, src) => {
            registers[*dest] *= load(registers, src);
        }
        Instruction::Div(dest, src) => {
            registers[*dest] /= load(registers, src);
        }
        Instruction::Mod(dest, src) => {
            registers[*dest] %= load(registers, src);
        }
        Instruction::Eql(dest, src) => {
            registers[*dest] =
                (registers[*dest] == load(registers, src)) as i64;
        }
    }
}

type Registers = [i64; 4];
type Cache = FxHashMap<(Registers, usize), Option<i64>>;

fn solve(
    program: &[Instruction],
    ip: usize,
    registers: Registers,
    cache: &mut Cache,
    biggest: bool,
) -> Option<i64> {
    if let Some(res) = cache.get(&(registers, ip)) {
        return *res;
    }

    let digits: Vec<_> = if biggest {
        (1..=9).rev().collect()
    } else {
        (1..=9).collect()
    };

    for digit in digits {
        let mut registers = registers;
        eval(&mut registers, &program[ip], digit);
        let mut ip = ip + 1;

        while let Some(instr) = program.get(ip) {
            if let Instruction::Inp(_) = instr {
                if let Some(num) = solve(program, ip, registers, cache, biggest)
                {
                    let num = num * 10 + digit;
                    cache.insert((registers, ip), Some(num));
                    return Some(num);
                } else {
                    break;
                }
            } else {
                eval(&mut registers, instr, 0);
                ip += 1;
            }
        }

        if registers[register_to_index('z')] == 0 && ip >= program.len() {
            cache.insert((registers, ip), Some(digit));
            return Some(digit);
        }
    }

    cache.insert((registers, ip), None);
    None
}

fn main() {
    let input = fs::read_to_string(input_file("input24.txt")).unwrap();
    let input = parse(&input);

    let mut cache = FxHashMap::default();
    let p1: String =
        format!("{}", solve(&input, 0, [0; 4], &mut cache, true).unwrap())
            .chars()
            .rev()
            .collect();
    let p2: String =
        format!("{}", solve(&input, 0, [0; 4], &mut cache, false).unwrap())
            .chars()
            .rev()
            .collect();
    println!("part1 = {}", p1);
    println!("part2 = {}", p2);
}
