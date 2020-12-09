extern crate util;

use std::collections::{ HashSet };

use util::{ read_input };
use util::parse::{ Input, ParseResult, ParseResultEx };
use util::error::{ AppErr };

fn main() -> Result<(), AppErr> {
    /*
    --- Part One ---

    The boot code is represented as a text file with one instruction per line of text.
    Each instruction consists of an operation (acc, jmp, or nop) and an argument (a signed number like +4 or -20).

    - `acc`
        increases or decreases a single global value called the accumulator by the
        value given in the argument. For example, acc +7 would increase the accumulator
        by 7. The accumulator starts at 0. After an acc instruction, the instruction
        immediately below it is executed next.
    - `jmp`
        jumps to a new instruction relative to itself. The next instruction to execute
        is found using the argument as an offset from the jmp instruction; for example,
        jmp +2 would skip the next instruction, jmp +1 would continue to the instruction
        immediately below it, and jmp -20 would cause the instruction 20 lines above to
        be executed next.
    - `nop`
        stands for No OPeration - it does nothing.
        The instruction immediately below it is executed next.

    Run your copy of the boot code.
    Immediately before any instruction is executed a second time, what value is in the accumulator?
    */

    #[derive(Debug, PartialEq, Eq, Copy, Clone)]
    enum Op { Acc, Jmp, Nop }
    
    #[derive(Debug, PartialEq, Eq, Copy, Clone)]
    struct Instruction(Op, isize);
    
    fn parse_instruction(input: Input) -> ParseResult<Instruction> {
        let (input, op) = input.parse_token("acc").map_val(|_| Op::Acc)
            .or_try(|| input.parse_token("jmp").map_val(|_| Op::Jmp))
            .or_try(|| input.parse_token("nop").map_val(|_| Op::Nop))?;
        
        enum Sign { Neg, Pos }
        let (input, sign) = input.parse_token("+").val(Sign::Pos)
            .or_try(|| input.parse_token("-").val(Sign::Neg))?;
        
        let (input, value) = input.parse_i32()?;
        let value = match sign {
            Sign::Neg => (-value) as isize,
            Sign::Pos => value as isize,
        };
        Ok((input, Instruction(op, value)))
    }

    let instructions = read_input("input.txt")?
        .iter()
        .map(|line| Input::new(line))
        .map(|input| parse_instruction(input).map(|x| x.1))
        .collect::<Result<Vec<_>, _>>()?;

    enum Step { Continue, Break }
    fn run_program(instructions: &[Instruction], mut step: impl FnMut(Instruction, usize) -> Step) -> Option<isize> { 
        let mut acc: isize = 0;
        let mut pc: usize = 0;
        loop {
            let inst = instructions[pc];
            if let Step::Break = step(inst, pc) {
                return None;
            }
            match inst {
                Instruction(Op::Acc, value) => {
                    acc += value;
                    pc += 1;
                },
                Instruction(Op::Jmp, value) => {
                    pc = (value + pc as isize) as usize;
                },
                Instruction(Op::Nop, _) => {
                    pc += 1;
                },
            }
            // check for program termination
            if pc >= instructions.len() {
                return Some(acc);
            }
        }
    }

    let mut seen = HashSet::new();
    let acc = run_program(&instructions, |_, pc| {
        if seen.insert(pc) { Step::Continue } else { Step::Break }
    });

    println!("Part 1: acc before first repeated command = {:?}", acc);

    /*
    --- Part Two ---

    The program is supposed to terminate by attempting to execute an instruction immediately
    after the last instruction in the file. By changing exactly one jmp or nop, you can
    repair the boot code and make it terminate correctly.

    Fix the program so that it terminates normally by changing exactly one jmp (to nop)
    or nop (to jmp). What is the value of the accumulator after the program terminates?
    */

    fn test_program(instructions: &[Instruction]) -> Option<isize> {
        let mut seen = HashSet::new();
        run_program(&instructions, |_, pc| {
            if seen.insert(pc) { Step::Continue } else { Step::Break }
        })
    }

    /// Flip Nop to Jmp and Jmp to Nop, ignore Acc
    fn flip_instruction(inst: &mut Instruction) {
        inst.0 = match inst.0 {
            Op::Nop => Op::Jmp,
            Op::Jmp => Op::Nop,
            Op::Acc => Op::Acc,
        };
    }

    let mut instructions = instructions;
    for i in 0..instructions.len() {

        flip_instruction(&mut instructions[i]);

        if let Some(acc) = test_program(&instructions) {
            println!("Part 2: acc = {:?}", acc);
            break;
        }

        flip_instruction(&mut instructions[i]);
    }

    Ok(())
}
