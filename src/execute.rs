use crate::bytecode::Bytecode;
use std::io::Read;

pub fn execute(code: &Bytecode, tape_size: usize) {
    let mut tape = vec![crate::Cell::default(); match tape_size {
        0 => 1,
        _ => tape_size
    }];
    let mut mc = 0;
    execute_recurse(code, &mut tape, &mut mc, tape_size == 0);
}

pub fn execute_recurse(code: &Bytecode, tape: &mut Vec<crate::Cell>, mc: &mut usize, unbound_tape: bool) {
    for i in code {
        use crate::bytecode::BfOperation;
        match i {
            BfOperation::Add(x) => tape[*mc] += x,
            BfOperation::Sub(x) => tape[*mc] -= x,
            BfOperation::Right(x) => *mc += x,
            BfOperation::Left(x) => *mc -= x,
            BfOperation::Print => print!("{}", tape[*mc].0 as char),
            BfOperation::Read => tape[*mc] = std::num::Wrapping(std::io::stdin().bytes().next().unwrap().unwrap() as u8),
            BfOperation::Loop(c) => {
                while tape[*mc].0 != 0 {
                    execute_recurse(c, tape, mc, unbound_tape);
                }
            },
        }
        while unbound_tape && *mc >= tape.len() {
            tape.push(crate::Cell::default());
        }
    }
}