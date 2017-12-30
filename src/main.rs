use std::io;
use std::io::Read;
use std::env;
use std::fs::File;

const MEMORY_SIZE : usize = 30000;
type InstructionIndex = usize;
enum Op {
    IncrementDP,
    DecrementDP,
    IncrementByteAtDP,
    DecrementByteAtDP,
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
    for instruction in input.chars() {
        match instruction {
            '>' => parsed.push(Op::IncrementDP),
            '<' => parsed.push(Op::DecrementDP),
            '+' => parsed.push(Op::IncrementByteAtDP),
            '-' => parsed.push(Op::DecrementByteAtDP),
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
        index += 1;
    }
    if bracket_stack.len() != 0 {
        panic!("Start bracket found with no end match");
    }
    parsed
}

fn execute(program : Program) {
    let mut memory : Vec<u8> = vec![0; MEMORY_SIZE];
    let mut instruction_pointer : InstructionIndex = 0;
    let mut data_pointer : usize = 0;
    
    let last_instruction_pointer = program.len() - 1;
    while instruction_pointer <= last_instruction_pointer {
        let instruction = &program[instruction_pointer];
        match *instruction {
            Op::IncrementDP => {
                if data_pointer == MEMORY_SIZE - 1 {
                    panic!("Data pointer out of bounds");
                }
                data_pointer += 1;
            }
            Op::DecrementDP => {
                if data_pointer == 0 {
                    panic!("Data pointer out of bounds");
                }
                data_pointer -= 1;
            }
            Op::IncrementByteAtDP => memory[data_pointer] += 1,
            Op::DecrementByteAtDP => memory[data_pointer] -= 1,
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
    let filename = &args[1];
    let mut f = File::open(filename).expect("File not found");
    f.read_to_string(&mut program_input)
        .expect("something went wrong reading the file");
    execute(parse(&program_input));
}
