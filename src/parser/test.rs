use crate::lexer::lexer::Lexer;
use crate::lexer::token::Token;
use crate::parser::program::{Expression, Statement};
use crate::parser::Parser;

#[cfg(test)]
fn check_let_statement(st: &Statement, name_expect: &str) -> bool {
    if let Statement::LetStatement(name, _) = st {
        name_expect.eq(&name.0)
    } else {
        false
    }
}

#[cfg(test)]
fn check_literal_expression() -> bool {
    false
}

#[cfg(test)]
fn check_infix_expression() -> bool {
    false
}

#[cfg(test)]
fn check_function_expression(st: &Statement, expects: &Vec<&str>) -> bool {
    if let Statement::ExpressionStatement(Expression::FunctionExpression(params, _)) = st {
        assert_eq!(expects.len(), params.len());
        for (i, param) in params.iter().enumerate() {
            assert_eq!(param.0, expects[i]);
        }
        true
    } else {
        eprintln!("FunctionExpression expected, but {:?} instead.", st);
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

#[test]
fn test_infix_expression() {
    let input = "4 + 5 + 10;";

    let l = Lexer::new(input);
    let mut p = Parser::new(l);
    let program = p.parse_program().unwrap();

    assert_eq!(program.statements.len(), 1);
    assert_eq!(
        program.statements[0],
        Statement::ExpressionStatement(Expression::InfixExpression(
            Box::new(Expression::IntLiteral(4)),
            Token::Plus,
            Box::new(Expression::InfixExpression(
                Box::new(Expression::IntLiteral(5)),
                Token::Plus,
                Box::new(Expression::IntLiteral(10))
            ))
        ))
    );
}

#[test]
fn test_operator_precedence() {
    let tests = vec![
        ("-a * b", "((-a) * b)"),
        ("!-a", "(!(-a))"),
        ("a + b + c", "((a + b) + c)"),
        ("a + b - c", "((a + b) - c)"),
        ("a * b * c", "((a * b) * c)"),
        ("a * b / c", "((a * b) / c)"),
        ("a + b / c", "(a + (b / c))"),
        ("a + b * c + d / e - f", "(((a + (b * c)) + (d / e)) - f)"),
        ("3 + 4; -5 * 5", "(3 + 4)((-5) * 5)"),
        ("5 > 4 == 3 < 4", "((5 > 4) == (3 < 4))"),
        ("5 < 4 != 3 > 4", "((5 < 4) != (3 > 4))"),
        (
            "3 + 4 * 5 == 3 * 1 + 4 * 5",
            "((3 + (4 * 5)) == ((3 * 1) + (4 * 5)))",
        ),
        (
            "3 + 4 * 5 == 3 * 1 + 4 * 5",
            "((3 + (4 * 5)) == ((3 * 1) + (4 * 5)))",
        ),
    ];

    for (input, expect) in tests {
        let l = Lexer::new(input);
        let mut p = Parser::new(l);
        let program = p.parse_program().unwrap();

        //TODO
    }
}

#[test]
fn test_if_else_expression() {
    let input = "if (x < y) { x } else { y }";

    let l = Lexer::new(input);
    let mut p = Parser::new(l);
    let program = p.parse_program().unwrap();

    assert_eq!(program.statements.len(), 1);
}

#[test]
fn test_function_literal() {
    let cases = vec![
        ("fn() {};", vec![]),
        ("fn(x) {};", vec!["x"]),
        ("fn(x, y, z) {};", vec!["x", "y", "z"]),
    ];

    for (input, expect) in cases {
        let l = Lexer::new(input);
        let mut p = Parser::new(l);
        let program = p.parse_program().unwrap();

        assert!(check_function_expression(&program.statements[0], &expect));
    }
}

#[test]
fn test_call_expression() {
    let input = "add(1, 2 * 3, 4 + 5);";
    let l = Lexer::new(input);
    let mut p = Parser::new(l);
    let program = p.parse_program().unwrap();

    assert_eq!(program.statements.len(), 1);
    println!("{:?}", program.statements[0]);
}
