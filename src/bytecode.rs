#[derive(Debug, PartialEq)]
pub enum BfOperation {
    Add(crate::Cell),
    Sub(crate::Cell),
    Right(usize),
    Left(usize),
    Clear,
    Print,
    Read,
    Loop(Bytecode),
    StaticLoop(Bytecode),
}
pub type Bytecode = Vec<BfOperation>;

#[derive(PartialEq)]
enum BfContractable {
    Add,
    Sub,
    Left,
    Right,
    No,
}

pub fn parse_bf(bf: &mut std::str::Chars) -> Bytecode {
    parse_bf_internal(bf).0
}

fn parse_bf_internal(bf: &mut std::str::Chars) -> (Bytecode, bool) {
    let mut bytecode = Bytecode::new();
    let mut lastop: BfContractable = BfContractable::No;
    let mut lastcount: usize = 0;

    let mut is_static = true;
    let mut offset = 0;

    macro_rules! dump_contract {
        () => {
            if lastcount > 0 {
                match lastop {
                    BfContractable::Add => bytecode.push(BfOperation::Add(std::num::Wrapping(lastcount as u8))),
                    BfContractable::Sub => bytecode.push(BfOperation::Sub(std::num::Wrapping(lastcount as u8))),
                    BfContractable::Right => bytecode.push(BfOperation::Right(lastcount)),
                    BfContractable::Left => bytecode.push(BfOperation::Left(lastcount)),
                    BfContractable::No => {},
                }
            }
        };
    }

    macro_rules! contractable {
        ($BfOp: expr) => {
            if lastop == $BfOp {
                lastcount += 1;
            } else {
                dump_contract!();
                lastop = $BfOp;
                lastcount = 1;
            }
        };
    }

    macro_rules! non_contractable {
        ($BfOp: expr) => {
            {
                dump_contract!();
                lastop = BfContractable::No;
                lastcount = 0;
                bytecode.push($BfOp);
            }
        };
    }

    while let Some(c) = bf.next() {
        use BfOperation::*;
        match c {
            '+' => contractable!(BfContractable::Add),
            '-' => contractable!(BfContractable::Sub),
            '<' => {
                contractable!(BfContractable::Left);
                offset -= 1;
            },
            '>' => {
                contractable!(BfContractable::Right);
                offset += 1;
            },
            '.' => non_contractable!(Print),
            ',' => non_contractable!(Read),
            '[' => {
                let (child, child_static) = parse_bf_internal(bf);
                if !child_static {
                    is_static = false;
                }
                if child.len() == 1 && child[0] == Sub(std::num::Wrapping(1)) {
                    non_contractable!(Clear);
                } else {
                   match child_static { 
                        false => non_contractable!(Loop(child)),
                        true => non_contractable!(StaticLoop(child)),
                    }
                }
            },
            ']' => {
                dump_contract!();
                break;
            },
            _ => {} // Unknown characters are comments in brainfuck
        }
    }
    if offset != 0 {
        is_static = false;
    }
    return (bytecode, is_static);
}