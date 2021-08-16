use crate::lexer::lexer::Lexer;
use crate::parser::program::Statement;
use crate::parser::Parser;

#[cfg(test)]
fn check_let_statement(st: &Statement, name_expect: &str) -> bool {
    if let Statement::LetStatement(name, _) = st {
        name_expect.eq(&name.0)
    } else {
        false
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
        assert!(check_let_statement(st, names[pos]));
    }
}
