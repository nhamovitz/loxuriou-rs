use std::{
    io::{BufRead, Write},
    process::{ExitCode, ExitStatus},
};

use chunk::{Chunk, OpCode};
use vm::InterpretResult;

mod chunk;
mod compiler;
mod scanner;
mod test;
mod vm;

fn repl() {
    let mut line = String::with_capacity(1024);
    let stdin = std::io::stdin();
    let mut stdout = std::io::stdout();

    loop {
        line.clear();
        print!("> ");

        if let Err(e) = stdout.flush() {
            eprintln!(
                "error flushing stdout while preparing for next prompt: {:?}",
                e
            );
            break;
        }
        let input_size = stdin.lock().read_line(&mut line);

        match input_size {
            Ok(len) => {
                // newline might be \n or \r\n
                if len <= 2 {
                    break;
                } else {
                    interpret(&line);
                }
            }
            Err(e) => eprintln!("Error reading line from stdin: {:?}", e),
        }
    }
}

fn interpret(source: &str) -> InterpretResult {
    compiler::compile(source);
    InterpretResult::Ok
}

fn run_file(file: &str) -> ExitCode {
    let source = std::fs::read_to_string(file);
    if let Ok(source) = source {
        let res = interpret(&source);
        match res {
            InterpretResult::Ok => ExitCode::SUCCESS,
            InterpretResult::CompileError => ExitCode::from(65),
            InterpretResult::RuntimeError => ExitCode::from(70),
        }
    } else {
        eprintln!("error reading file to memory {:?}", &file);
        ExitCode::FAILURE
    }
}

fn main() -> ExitCode {
    let args = std::env::args().collect::<Vec<_>>();

    interpret("hello class 1 3\n1.2 *\n// comment\ntrue = >= > 250123 var3 or");

    if args.len() == 1 {
        repl();
        ExitCode::SUCCESS
    } else if args.len() == 2 {
        run_file(&args[1])
    } else {
        eprintln!("usage: loxuriou-rs [path]");
        ExitCode::from(64)
    }
}
