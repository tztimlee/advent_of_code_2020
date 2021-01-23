use self::Instruction::*;
use core::panic;
use std::{fs, println, str::FromStr};

fn main() {
    let contents = fs::read_to_string("input").expect("Something went wrong reading the file");

    let program = contents
        .lines()
        .map(|x| x.parse::<Instruction>().unwrap())
        .collect::<Vec<Instruction>>();

    // for i in program{
    //     println!("{:?}", i);
    // }
    // println!("{:?}", program);
    let mut ip = 0;
    let mut acc = 0;
    let mut visited = Vec::new();

    loop {
        if ip as usize == program.len() {
            println!("Terminated Successfully: {}", acc);
            break;
        }

        // let Instruction{op, val} = &program[ip as usize];
        visited.push(ip);

        let instr = &program[ip as usize];

        match instr {
            Jmp(val) => ip += val,
            Acc(val) => {
                acc += val;
                ip += 1;
            }
            Nop(_) => ip += 1,
        }

        if visited.contains(&ip) {
            // println!("{:?}", visited);
            // visited.sort();
            // println!("{:?}", visited);

            println!("{}", acc);
            break;
        }
    }
}

#[derive(Debug, Clone)]
enum Instruction {
    Jmp(i32),
    Acc(i32),
    Nop(i32),
}
// #[derive(Debug)]
// struct Instruction {
//     op: Opcode,
//     val: i64
//

impl FromStr for Instruction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let vals = s.split_whitespace().collect::<Vec<&str>>();
        let val = vals[1].parse::<i32>().unwrap();
        let op = match vals[0] {
            "jmp" => Jmp(val),
            "acc" => Acc(val),
            "nop" => Nop(val),
            _ => panic!("unknown opcode found"),
        };
        Ok(op)
    }
}
