use crate::scanner::{self, TokenType};

pub(crate) fn compile(source: &str) {
    println!("compiling");

    let mut scanner = scanner::Scanner::new(source);

    let mut line = None;
    loop {
        let token = scanner.scan_token();

        if line != Some(token.line) {
            print!("{: >4?} ", token.line);
            line = Some(token.line);
        } else {
            print!("   | ");
        }

        println!(
            "Type: {:?};  Length:{:?};  Start_idx:{:?}",
            token.token_type, token.length, token.start
        );

        match token.token_type {
            TokenType::EOF => break,
            _ => {}
        }
    }
}
