use crate::lexer::lexer::Lexer;
use crate::parser::program::{LetStatement, Statement, Node};
use crate::parser::Parser;

#[cfg(test)]
fn check_let_statement(st: &dyn Statement, name: &str) -> bool {
    match st.as_any().downcast_ref::<LetStatement>() {
        Some(statement) => {
            statement.name.token_literal().eq(name)
        },
        None => false,
    }
}

#[test]
fn test_let_statements() {
    let input = r"
    let x = 5;
    let y = 10;
    let foobar = 838383;
    ";
    let names = vec!["x", "y", "foobar"];

    let l = Lexer::new(input);
    let mut p = Parser::new(l);
    let program = p.parse_program().unwrap();

    assert_eq!(program.statements.len(), 3);
    for (pos, st) in program.statements.iter().enumerate() {
        assert!(check_let_statement(st.as_ref(), names[pos]));
    }
}
