use std::io::Write;

use async_ctrlc::CtrlC;
use async_std::prelude::*;

use crate::eval::evaluator;
use crate::lexer::lexer::Lexer;
use crate::parser::Parser;
use crate::eval::environment::Environment;
use crate::eval::evaluator::Evaluator;

mod eval;
mod lexer;
mod parser;

const PROMPT: &str = ">>";

#[async_std::main]
async fn main() {
    env_logger::init();

    let ctrlc = CtrlC::new().expect("cannot create Ctrl+C handler!");
    println!("Welcome to Monkey Language REPL, press Ctrl+C to quit.");

    ctrlc
        .race(async {
            let mut env = Environment::default();

            loop {
                print!("{}", PROMPT);
                std::io::stdout().flush().unwrap();

                let mut buf = String::new();
                async_std::io::stdin().read_line(&mut buf).await.unwrap();

                let lx = Lexer::new(buf);
                let mut p = Parser::new(lx);
                if let Err(e) = p.parse_program().and_then(|program| {
                    let mut evaluator = Evaluator::with_env(&program.statements, env.clone());
                    evaluator.eval().and_then(|obj| {
                        println!("{:?}", obj);
                        env = evaluator.get_env();
                        Ok(())
                    })
                }) {
                    eprintln!("{:?}", e);
                }
            }
        })
        .await;
    println!("\nCtrl+C pressed, quiting")
}
