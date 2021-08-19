use crate::lexer::lexer::Lexer;
use crate::lexer::token::Token;
use crate::parser::Parser;
use crate::parser::program::{Expression, Statement};

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
fn test_if_else_expression() {
    let input = "if (x < y) { x } else { y }";

    let l = Lexer::new(input);
    let mut p = Parser::new(l);
    let program = p.parse_program().unwrap();

    assert_eq!(program.statements.len(), 1);
}

#[test]
fn test_function_literal() {
    let cases = [
        ("fn() {};", vec![]),
        ("fn(x) {};", vec!["x"]),
        ("fn(x, y, z) {};", vec!["x", "y", "z"]),
    ];

    for (input, expect) in cases {
        let l = Lexer::new(input);
        let mut p = Parser::new(l);
        let program = p.parse_program().unwrap();

        assert_eq!(program.statements.len(), 1);
        if let Statement::ExpressionStatement(
            Expression::FunctionExpression(params, _)
        ) = &program.statements[0] {
            assert_eq!(expect.len(), params.len());
            for (i, param) in expect.iter().enumerate() {
                assert_eq!(*param, params[i].0);
            }
        } else {
            println!("FunctionExpression expected, but {:?} instead.", program.statements[0]);
            assert!(false);
        }
    }
}
