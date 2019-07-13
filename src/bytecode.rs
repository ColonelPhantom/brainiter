#[derive(Debug)]
pub enum BfOperation {
    Add(crate::Cell),
    Sub(crate::Cell),
    Right(usize),
    Left(usize),
    Print,
    Read,
    Loop(Bytecode),
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
    let mut bytecode = Bytecode::new();
    let mut lastop: BfContractable = BfContractable::No;
    let mut lastcount: usize = 0;

    macro_rules! dump_contract {
        () => {
            match lastop {
                BfContractable::Add => bytecode.push(BfOperation::Add(std::num::Wrapping(lastcount as u8))),
                BfContractable::Sub => bytecode.push(BfOperation::Sub(std::num::Wrapping(lastcount as u8))),
                BfContractable::Right => bytecode.push(BfOperation::Right(lastcount)),
                BfContractable::Left => bytecode.push(BfOperation::Left(lastcount)),
                BfContractable::No => {},
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
            '<' => contractable!(BfContractable::Left),
            '>' => contractable!(BfContractable::Right),
            '.' => non_contractable!(Print),
            ',' => non_contractable!(Read),
            '[' => non_contractable!(Loop(parse_bf(bf))),
            ']' => break,
            _ => {} // Unknown characters are comments in brainfuck
        }
    }
    return bytecode;
}