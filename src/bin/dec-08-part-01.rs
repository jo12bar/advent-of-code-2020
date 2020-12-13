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

#[derive(Copy, Clone, Debug, Eq, PartialEq, Default)]
struct Machine {
    pub ins_ptr: u64,
    pub accumulator: i64,
}

fn main() -> Result<()> {
    let mut instructions =
        parse_instructions(INPUT_FILE).context("Could not parse instructions")?;
    let mut state = Machine::default();

    let old_accumlator = loop {
        let old_ins_ptr = state.ins_ptr as usize;
        let old_accumulator = state.accumulator;

        state = instructions[old_ins_ptr].run(&state)?;

        if instructions[old_ins_ptr].run_count == 2 {
            break (old_ins_ptr, old_accumulator);
        }
    };

    println!(
        "Machine state after running an instruction twice:\n{:#?}\n",
        state
    );

    println!(
        "(ins_ptr, accumulator) value *before* running an instruction twice:\n{:?}",
        old_accumlator,
    );

    Ok(())
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
