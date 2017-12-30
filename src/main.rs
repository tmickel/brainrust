use std::io;
use std::io::Read;
use std::env;
use std::fs::File;

const MEMORY_SIZE : usize = 30000;
type InstructionIndex = usize;
type CellType = u8;
#[derive(Clone)]
enum Op {
    IncreaseDPBy(usize),
    DecreaseDPBy(usize),
    IncreaseByteAtDPBy(CellType),
    DecreaseByteAtDPBy(CellType),
    OutputByteAtDP,
    InputByteToDP,
    IfByteZeroJumpTo(Option<InstructionIndex>),
    IfByteNotZeroJumpTo(InstructionIndex)
}
type Program = Vec<Op>;

fn parse(input : &str) -> Program {
    let mut parsed: Program = Vec::new();
    let mut bracket_stack: Vec<InstructionIndex> = Vec::new();
    let mut index : InstructionIndex = 0;
    let mut previous_instruction : Option<Op> = None;
    for instruction in input.chars() {
        match instruction {
            // Repeat > < + - are coalesced into single ops for execution,
            // as an optimization. For example, >>>>> will be IncreaseDPBy(5).
            // TODO: Can refactor lots of duplicate logic here, probably.
            '>' => {
                match previous_instruction {
                    Some(Op::IncreaseDPBy(n)) =>
                        parsed[index - 1] = Op::IncreaseDPBy(n + 1),
                    _ =>
                        parsed.push(Op::IncreaseDPBy(1))
                }
            },
            '<' => {
                match previous_instruction {
                    Some(Op::DecreaseDPBy(n)) =>
                        parsed[index - 1] = Op::DecreaseDPBy(n + 1),
                    _ =>
                        parsed.push(Op::DecreaseDPBy(1))
                }
            },
            '+' => {
                match previous_instruction {
                    Some(Op::IncreaseByteAtDPBy(n)) =>
                        parsed[index - 1] = Op::IncreaseByteAtDPBy(n + 1),
                    _ => 
                        parsed.push(Op::IncreaseByteAtDPBy(1))
                }
            },
            '-' => {
                match previous_instruction {
                    Some(Op::DecreaseByteAtDPBy(n)) =>
                        parsed[index - 1] = Op::DecreaseByteAtDPBy(n + 1),
                    _ =>
                        parsed.push(Op::DecreaseByteAtDPBy(1))
                }
            },
            '.' => parsed.push(Op::OutputByteAtDP),
            ',' => parsed.push(Op::InputByteToDP),
            '[' => {
                // For now we don't know the matching `]`,
                // so push an unknown to the stack.
                bracket_stack.push(index);
                parsed.push(Op::IfByteZeroJumpTo(None));
            },
            ']' => {
                let matching_start = bracket_stack.pop();
                match matching_start {
                    Some(matching_start_index) => {
                        // Set matching ['s value to the index after this one.
                        parsed[matching_start_index] =
                            Op::IfByteZeroJumpTo(
                                Some(index + 1)
                            );
                        // And set ours to the index after the matching ]'s.
                        parsed.push(
                            Op::IfByteNotZeroJumpTo(
                                matching_start_index + 1
                            )
                        );
                    }
                    None => panic!("End bracket found with no start match")
                }
            },
            _ => {
                continue; // Invalid characters are skipped.
            }
        }
        // Makes a copy, because otherwise, we would need to hold
        // an immutable reference of `parsed`? (Not possible while we
        // hold a mutable reference).
        previous_instruction = Some(parsed[parsed.len() - 1].clone());
        index = parsed.len();
    }
    if bracket_stack.len() != 0 {
        panic!("Start bracket found with no end match");
    }
    parsed
}

fn execute(program : Program) {
    let mut memory : Vec<CellType> = vec![0; MEMORY_SIZE];
    let mut instruction_pointer : InstructionIndex = 0;
    let mut data_pointer : usize = 0;
    
    let last_instruction_pointer = program.len() - 1;
    while instruction_pointer <= last_instruction_pointer {
        let instruction = &program[instruction_pointer];
        match *instruction {
            Op::IncreaseDPBy(n) => data_pointer += n,
            Op::DecreaseDPBy(n) => data_pointer -= n,
            Op::IncreaseByteAtDPBy(n) => memory[data_pointer] += n,
            Op::DecreaseByteAtDPBy(n) => memory[data_pointer] -= n,
            Op::OutputByteAtDP => print!("{}", memory[data_pointer] as char),
            Op::InputByteToDP => {
                let mut input_char : [u8; 1] = [0];
                io::stdin().read_exact(&mut input_char)
                    .expect("Something went wrong reading in");
                memory[data_pointer] = input_char[0];
            }
            Op::IfByteZeroJumpTo(Some(jump_location)) => {
                if memory[data_pointer] == 0 {
                    instruction_pointer = jump_location;
                    continue;
                }
            },
            Op::IfByteZeroJumpTo(None) =>
                panic!("Start bracket found with no end match"),
            Op::IfByteNotZeroJumpTo(jump_location) => {
                if memory[data_pointer] != 0 {
                    instruction_pointer = jump_location;
                    continue;
                }
            }
        }
        instruction_pointer += 1;
    } 
}

fn main() {
    let mut program_input = String::new();
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("Please specific a .bf file to run.");
    }
    let filename = &args[1];
    let mut f = File::open(filename).expect("File not found");
    f.read_to_string(&mut program_input)
        .expect("something went wrong reading the file");
    execute(parse(&program_input));
}
