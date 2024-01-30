use std::{ env::args, fs::File, io::Read };

use parser::{ tokens::stream::TokenStream, ast::{ stream::ParseStream, block::Block } };
use runtime::{ codegen::{ Codegen, ToBytecode }, instructions::Instruction, stack::Stack, Thread };

fn main() {
    let mut args = args().skip(0);
    if let Some(_path) = args.next() {
        let mut buf = String::new();
        let file_len = File::open("./test.fused").unwrap().read_to_string(&mut buf).unwrap();
        let time = std::time::Instant::now();
        let tokens = TokenStream::from_string(buf).unwrap();
        let mut parse = ParseStream::new(tokens);
        let block = parse.parse::<Block>().unwrap();
        let mut codegen = Codegen::new();
        _ = block.to_bytecode(&mut codegen).unwrap();
        codegen.emit_simple(Instruction::Return).unwrap();

        let chunk = codegen.chunk();
        let size = chunk.size();
        println!(
            " File size: {file_len}\n Chunk size: {size}\n C/F Ratio: {:.2}%",
            ((size as f64) / (file_len as f64)) * 100f64
        );
        println!("{}", chunk);
        let mut thread = Thread {
            stack: Stack::new(chunk.var_count ),
        };

        let result = thread.run_chunk(chunk).unwrap();
        let done = time.elapsed();

        println!("{result} in {done:?}")
    } else {
        run_repl()
    }
}

fn run_repl() {
    todo!()
}
