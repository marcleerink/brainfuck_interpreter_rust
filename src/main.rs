use std::io::Read;
use structopt::StructOpt;

const SIZE: usize = 30000;

#[derive(Debug, Clone)]
enum Character {
    IncrementPointer,
    DecrementPointer,
    IncrementValue,
    DecrementValue,
    Output,
    Input,
    JumpForward,
    JumpBackward,
}
type Program = Vec<Character>;

#[derive(StructOpt, Debug)]
#[structopt(name = "brainfuck")]
struct Opt {
    #[structopt(parse(from_os_str))]
    file: std::path::PathBuf,
}
fn get_character(c: char) -> Option<Character> {
    match c {
        '>' => Some(Character::IncrementPointer),
        '<' => Some(Character::DecrementPointer),
        '+' => Some(Character::IncrementValue),
        '-' => Some(Character::DecrementValue),
        '.' => Some(Character::Output),
        ',' => Some(Character::Input),
        '[' => Some(Character::JumpForward),
        ']' => Some(Character::JumpBackward),
        _ => None,
    }
}

fn translate(program: &str) -> Vec<Character> {
    program.chars().filter_map(|c| get_character(c)).collect()
}

fn interpret(program: Program) {
    let mut memory: [u8; SIZE] = [0; SIZE];
    let mut pointer = 0;
    let mut pc = 0;
    let mut jump_locations = vec![0; program.len()];
    let mut stack = Vec::new();

    for (i, instruction) in program.iter().enumerate() {
        match instruction {
            Character::JumpForward => {
                stack.push(i);
            }
            Character::JumpBackward => {
                let jump_forward_location = stack.pop().unwrap();
                jump_locations[i] = jump_forward_location;
                jump_locations[jump_forward_location] = i;
            }
            _ => {}
        }
    }
    while pc < program.len() {
        match program[pc] {
            Character::IncrementPointer => {
                pointer += 1;
            }
            Character::DecrementPointer => {
                pointer -= 1;
            }
            Character::IncrementValue => {
                memory[pointer] = memory[pointer].wrapping_add(1);
            }
            Character::DecrementValue => {
                memory[pointer] = memory[pointer].wrapping_sub(1);
            }
            Character::Output => {
                print!("{}", memory[pointer] as char);
            }
            Character::Input => {
                let mut buffer = [0; 1];
                std::io::stdin().read_exact(&mut buffer).unwrap();
                memory[pointer] = buffer[0];
            }
            Character::JumpForward => {
                if memory[pointer] == 0 {
                    let mut count = 1;
                    while count != 0 {
                        pc += 1;
                        match program[pc] {
                            Character::JumpForward => count += 1,
                            Character::JumpBackward => count -= 1,
                            _ => {}
                        }
                    }
                } else {
                    stack.push(pc);
                }
            }
            Character::JumpBackward => {
                if memory[pointer] != 0 {
                    pc = stack[stack.len() - 1];
                } else {
                    stack.pop();
                }
            }
        }
        pc += 1;
    }
}

fn main() {
    let opt = Opt::from_args();

    let program_str = std::fs::read_to_string(opt.file).expect("Could not read file");

    let program: Program = translate(&program_str);
    if program.is_empty() {
        println!("Empty program");
        return;
    }

    if program.len() > SIZE {
        println!("Program too big");
        return;
    }
    interpret(program)
}
