mod lexer;
mod parser;

use crate::lexer::lexer::Lexer;
use std::io::Write;
use async_ctrlc::CtrlC;
use async_std::prelude::*;

const PROMPT: &str = ">>";

#[async_std::main]
async fn main() {
    let ctrlc = CtrlC::new().expect("cannot create Ctrl+C handler!");
    println!("Welcome to Monkey Language REPL, press Ctrl+C to quit.");

    ctrlc.race(async {
        loop {
            print!("{}", PROMPT);
            std::io::stdout().flush().unwrap();

            let mut buf = String::new();
            async_std::io::stdin().read_line(&mut buf).await.unwrap();

            let mut lx = Lexer::new(buf);
            loop {
                let token = lx.next_token();
                if token.is_eof() {
                    break;
                }
                println!("{:?}", token);
            }
        }
    }).await;
    println!("\nCtrl+C pressed, quiting")
}
