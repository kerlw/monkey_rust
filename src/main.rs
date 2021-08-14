mod lexer;

use crate::lexer::lexer::Lexer;
use std::io::Write;

const PROMPT: &str = ">>";


fn main() {
    loop {
        print!("{}", PROMPT);
        std::io::stdout().flush().unwrap();

        let mut buf = String::new();
        std::io::stdin().read_line(&mut buf).unwrap();

        let mut lx = Lexer::new(buf);
        loop {
            let token = lx.next_token();
            if token.is_eof() {
                break;
            }
            println!("{:?}", token);
        }
    }
}
