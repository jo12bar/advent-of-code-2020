use anyhow::{anyhow, Context, Result};
use std::{
    fmt::Display,
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};

const INPUT_FILE: &str = "./input/dec-08-part-01/input.txt";

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum InstructionType {
    /// Increase or decrease the global accumulator by the argument.
    /// Note that the accumulator starts at 0.
    Acc,
    /// Jump to a new instruction *relative to this `jmp` instruction*.
    Jmp,
    /// Do nothing. Go to the next instruction.
    Nop,
}

impl Display for InstructionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Acc => write!(f, "acc"),
            Self::Jmp => write!(f, "jmp"),
            Self::Nop => write!(f, "nop"),
        }
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
struct Instruction {
    /// The instruction's type
    pub ins_type: InstructionType,
    /// The instruction's argument
    pub arg: i64,
    /// The instruction's address in virtual memory
    pub ins_addr: u64,
    /// How many times the instruction has been run
    pub run_count: usize,
}

impl Instruction {
    pub fn new(ins_addr: u64, ins_type: InstructionType, arg: i64) -> Self {
        Self {
            ins_type,
            arg,
            ins_addr,
            run_count: 0,
        }
    }

    /// Runs the instruction according to some machine state, consuming it.
    /// Returns a new machine state for after the instruction is run.
    ///
    /// Also increments this instruction's `run_count`.
    pub fn run(&mut self, state: &Machine) -> Result<Machine> {
        let next_state = match self.ins_type {
            InstructionType::Acc => Machine {
                ins_ptr: state
                    .ins_ptr
                    .checked_add(1)
                    .ok_or_else(|| self.fmt_ins_ptr_overflow(&state))?,
                accumulator: state.accumulator + self.arg,
            },

            InstructionType::Jmp => Machine {
                ins_ptr: if self.arg.is_negative() {
                    self.ins_addr
                        .checked_sub(self.arg.wrapping_abs() as u64)
                        .ok_or_else(|| self.fmt_ins_ptr_overflow(&state))?
                } else {
                    self.ins_addr
                        .checked_add(self.arg as u64)
                        .ok_or_else(|| self.fmt_ins_ptr_overflow(&state))?
                },

                ..*state
            },

            InstructionType::Nop => Machine {
                ins_ptr: state
                    .ins_ptr
                    .checked_add(1)
                    .ok_or_else(|| self.fmt_ins_ptr_overflow(&state))?,
                ..*state
            },
        };

        self.run_count += 1;
        Ok(next_state)
    }

    pub fn swap_nop_and_jmp(&self) -> Self {
        Self {
            ins_type: match self.ins_type {
                InstructionType::Jmp => InstructionType::Nop,
                InstructionType::Nop => InstructionType::Jmp,
                t => t,
            },
            ..*self
        }
    }

    fn fmt_ins_ptr_overflow(&self, state: &Machine) -> anyhow::Error {
        anyhow!(
            "INS PTR OVERFLOW\n\t\
                 Instruction type:      {}\n\t\
                 Instruction arg:       {}\n\t\
                 Instruction addr:      {}\n\t\
                 Instruction run count: {}\n\t\
                 State ins ptr:         {}\n\t\
                 State accumulator:     {}",
            self.ins_type,
            self.arg,
            self.ins_addr,
            self.run_count - 1,
            state.ins_ptr,
            state.accumulator,
        )
    }
}

impl Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.ins_type, self.arg)
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Default)]
struct Machine {
    pub ins_ptr: u64,
    pub accumulator: i64,
}

fn main() -> Result<()> {
    let mut program = parse_instructions(INPUT_FILE).context("Could not parse instructions")?;
    let mut state = Machine::default();

    let last_ins_addr = program.last().unwrap().ins_addr;

    // let (terminating_instruction, final_accumulator) = loop {
    //     let ins_ptr = state.ins_ptr as usize;
    //     let instruction = &mut instructions[ins_ptr];
    //     let mut swapped_instruction = instruction.swap_nop_and_jmp();

    //     let next_state = instruction.run(&state)?;
    //     let next_swapped_state = swapped_instruction.run(&state)?;

    //     if next_swapped_state.ins_ptr > last_ins_addr {
    //         break (swapped_instruction, next_swapped_state.accumulator);
    //     }

    //     state = next_state;
    // };

    // Run the program once, logging which instructions were hit. Terminate once
    // an instruction is run twice.
    println!("Running program until instruction is run twice...");
    let (_, _, trace) = run_program(program.clone())?;
    println!("Terminated!");

    // Start at the end of the instruction list and go backwards until you hit
    // the first negative jump. Anything in the range after the first negative
    // jump and until the end of the program + 1 will lead to program termination.
    // So, mark where the potential landing spots for program termination are.
    println!("Finding range of instructions garunteed to lead to program termination...");
    let mut potential_landing_spots = vec![false; program.len() + 1];
    let mut i = program.len();
    loop {
        potential_landing_spots[i] = true;
        i -= 1;

        if &program[i].ins_type == &InstructionType::Jmp && &program[i].arg < &0 {
            break;
        }
    }

    println!("Finding instruction to swap...");
    let start = i;

    let to_swap = if trace[i] {
        // If the first negative jump instruction was hit, change it to a nop
        // to lead to program termination!
        println!("Found last negative jmp to be changed into a nop");
        i
    } else {
        loop {
            i -= 1;
            let fake_state = Machine {
                ins_ptr: i as u64,
                ..Machine::default()
            };

            if potential_landing_spots[i] {
                continue;
            } else if &program[i].ins_type == &InstructionType::Nop {
                // If this instruction was hit, and swapping it would lead to
                // jumping to an address in our garunteed termination range, then
                // swap it.
                let swapped_state = program[i].swap_nop_and_jmp().run(&fake_state)?;

                if trace[i] && potential_landing_spots[swapped_state.ins_ptr as usize] {
                    println!("Found hit nop to change into jmp");
                    break i;
                }
            } else if &program[i].ins_type == &InstructionType::Jmp {
                let next_state = program[i].clone().run(&fake_state)?;

                // If this instruction is a jmp and WAS NOT hit and would lead to
                // the gaurunteed termination range...
                if !trace[i] && potential_landing_spots[next_state.ins_ptr as usize] {
                    // Find a jmp instruction somewhere before this one
                    let mut j = i - 1;
                    loop {
                        if &program[j].ins_type == &InstructionType::Jmp {
                            break;
                        }
                        j -= 1;
                    }

                    if trace[j] {
                        // If this jmp was preceded by a hit jmp, swap the hit jmp into a nop
                        // so that this one gets hit eventually
                        println!("Found unhit jmp preceded by a hit jmp, which will be changed into a nop");
                        break j;
                    } else {
                        // If this jmp was preceded by a non-hit jmp, then add this range of
                        // instructions to our gaurenteed termination range!
                        println!("Found instructions to add to gaurenteed termination range");
                        potential_landing_spots[j + 1..=i]
                            .iter_mut()
                            .for_each(|ins| {
                                *ins = true;
                            });
                        i = start;
                    }
                }
            }
        }
    };

    // Swap the instruction!
    println!("Swapping instruction...");
    program[to_swap] = program[to_swap].swap_nop_and_jmp();

    println!("Running fixed program...");
    let (terminated_naturally, final_accumulator, _) = run_program(program.clone())?;

    if !terminated_naturally {
        return Err(anyhow!(
            "Didn't terminate naturally for some reason! Ahhhhhhhhhhhh!!!!!1!"
        ));
    }

    println!("Terminated!");

    println!(
        "Changed `{}` into a `{}` at address `{}`.",
        program[to_swap].swap_nop_and_jmp(),
        program[to_swap],
        to_swap
    );
    println!("Final accumulator: {}", final_accumulator);

    Ok(())
}

/// Run a program until either:
///
/// 1. The program counter advances past the end of the program (i.e. it naturally terminates).
/// 2. An instruction is ran twice.
///
/// Returns a triple of:
///
/// 1. A `bool` which is `true` if the program terminated naturally (case 1. above);
/// 2. The program accumulator;
/// 3. A vector of booleans indicating which instructions were ran at least once.
fn run_program(mut program: Vec<Instruction>) -> Result<(bool, i64, Vec<bool>)> {
    let mut seen = vec![false; program.len()];
    let mut state = Machine::default();

    loop {
        // If the instruction pointer is past the program's end, the program
        // terminated naturally!
        if state.ins_ptr as usize >= program.len() {
            return Ok((true, state.accumulator, seen));
        }

        // If this instruction has already been seen, we're about to enter
        // an infinite loop. Terminate early.
        if seen[state.ins_ptr as usize] {
            return Ok((false, state.accumulator, seen));
        }

        // Now we actually run the instruction. Log that we've seen this one.
        seen[state.ins_ptr as usize] = true;

        state = program[state.ins_ptr as usize].run(&state)?;
    }
}

fn parse_instructions<P: AsRef<Path> + Display + Clone>(input_file: P) -> Result<Vec<Instruction>> {
    let input = File::open(input_file.clone())
        .with_context(|| format!("Could not open file {}", input_file))?;
    let input_reader = BufReader::new(input);

    let mut instructions = vec![];

    for (line_num, line) in input_reader.lines().enumerate() {
        let line = line?;
        let line = line.trim();

        let instruction = &line[0..3];
        let arg = line[4..]
            .parse::<i64>()
            .with_context(|| format!("Could not parse argument to integer: {}", &line[4..]))?;

        let ins_type = match instruction {
            "acc" => InstructionType::Acc,
            "jmp" => InstructionType::Jmp,
            "nop" => InstructionType::Nop,
            _ => {
                eprintln!(
                    "Warning: unknown instruction `{}` with argument `{}`. Skipping.",
                    instruction, arg
                );
                continue;
            }
        };

        instructions.push(Instruction::new(line_num as u64, ins_type, arg));
    }

    Ok(instructions)
}
