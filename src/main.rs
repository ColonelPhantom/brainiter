mod bytecode;
mod execute;

type Cell = std::num::Wrapping<u8>;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        println!("usage: bf <file.bf>");
        std::process::exit(1);
    }
    let source = std::fs::read_to_string(&args[1]).expect("Error reading file");
    let bytecode = bytecode::parse_bf(source.chars().by_ref());


    println!("{:?}", bytecode);
    execute::execute(&bytecode, 0);
}
