#[derive(Debug)]
pub enum BfOperation {
    Add,
    Sub,
    Right,
    Left,
    Print,
    Read,
    Loop(Bytecode),
}

pub type Bytecode = Vec<BfOperation>;

pub fn parse_bf(bf: &mut std::str::Chars) -> Bytecode {
    let mut bytecode = Bytecode::new();
    while let Some(c) = bf.next() {
        use BfOperation::*;
        match c {
            '+' => bytecode.push(Add),
            '-' => bytecode.push(Sub),
            '<' => bytecode.push(Left),
            '>' => bytecode.push(Right),
            '.' => bytecode.push(Print),
            ',' => bytecode.push(Read),
            '[' => bytecode.push(Loop(parse_bf(bf))),
            ']' => break,
            _ => {} // Unknown characters are comments in brainfuck
        }
    }
    return bytecode;
}