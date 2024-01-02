use std::{ env::args, path::Path, fs::File, io::Read, ptr::NonNull };

use libimmixcons::{ immix_init, immix_noop_callback };
use parser::{ tokens::stream::TokenStream, ast::{ stream::ParseStream, block::Block } };
use runtime::{ codegen::{ Codegen, ToBytecode }, Thread, stack::Stack };

fn main() {
    let mut args = args().skip(0);
    if let Some(path) = args.next() {
        let mut buf = String::new();
        let text = File::open("./test.fused").unwrap().read_to_string(&mut buf).unwrap();
        init_gc();
        let tokens = TokenStream::from_string(buf).unwrap();
        let mut parse = ParseStream::new(tokens);
        let block = parse.parse::<Block>().unwrap();
        let mut codegen = Codegen::new();
        let result = block.to_bytecode(&mut codegen);
        codegen.emit_return(result);

        let chunk = codegen.chunk();
        let mut thread = Thread {
            stack: Stack::new(),
        };

        let result = thread.run_chunk(chunk).unwrap();
        println!("{result}")
    } else {
        run_repl()
    }
}

#[inline]
fn init_gc() {
    immix_init(0, 0, immix_noop_callback, std::ptr::null_mut())
}

fn run_repl() {
    todo!()
}
