use crate::bytecode::Bytecode;

pub fn compile_c(code: &Bytecode, tape_size: usize) -> String {
    let mut c_code: String = format!("#include<stdio.h>\n\
        #include <stdint.h>\n\
        #include <stdlib.h>\n\
        void main() {{\n\
            int size = {tape_size};\n\
            int *tape = calloc(sizeof(uint8_t), size);\n\
            int mi = 0;\n\
        ", tape_size = tape_size).to_string();
    c_code += &compile_c_recurse(code);
    c_code += "}";
    return c_code;
}

fn compile_c_recurse(code: &Bytecode) -> String {
    let mut c_code = String::new();
    for i in code {
        use crate::bytecode::BfOperation;
        c_code += &match i {
            BfOperation::Add(x) => format!("tape[mi] += {};\n", x),
            BfOperation::Sub(x) => format!("tape[mi] -= {};\n", x),
            BfOperation::Right(x) => format!("mi += {};\n", x),
            BfOperation::Left(x) => format!("mi -= {};\n", x),
            BfOperation::Clear => "tape[mi] = 0;\n".to_string(),
            BfOperation::Print => "putchar(tape[mi]);\n".to_string(),
            BfOperation::Read => "tape[mi] = getchar();\n".to_string(),
            BfOperation::Loop(c) | BfOperation::StaticLoop(c) => {
                let mut loopstr = "while(tape[mi]) {".to_string();
                loopstr += &compile_c_recurse(c);
                loopstr += "}\n";
                loopstr
            }
            _ => panic!(),
        }
    }
    return c_code;
}