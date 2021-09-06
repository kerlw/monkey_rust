use crate::lexer::lexer::Lexer;
use crate::lexer::token::Token;
use crate::parser::program::{Expression, Ident, Statement};
use crate::parser::Parser;

#[cfg(test)]
fn check_let_statement(st: &Statement, name_expect: &str, value_expected: &Expression) -> bool {
    if let Statement::LetStatement(name, v) = st {
        name_expect.eq(&name.0) && v.eq(value_expected)
    } else {
        false
    }
}

// #[cfg(test)]
// fn check_literal_expression() -> bool {
//     false
// }
//
// #[cfg(test)]
// fn check_infix_expression() -> bool {
//     false
// }

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
    let y = true;
    let foobar = y;
    ";
    let name_values = vec![
        ("x", Expression::IntLiteral(5)),
        ("y", Expression::IntLiteral(10)),
        ("foobar", Expression::IntLiteral(838383)),
        ("y", Expression::BoolLiteral(true)),
        ("foobar", Expression::Identifier(Ident("y".into()))),
    ];

    let l = Lexer::new(input);
    let mut p = Parser::new(l);
    let program = p.parse_program().unwrap();

    assert_eq!(program.statements.len(), name_values.len());
    for (pos, st) in program.statements.iter().enumerate() {
        assert!(check_let_statement(
            st,
            name_values[pos].0,
            &name_values[pos].1
        ));
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
        ("a + add(b * c) + d", "((a + add((b * c))) + d)"),
        (
            "add(a, b, 1, 2 * 3, 4 + 5, add(6, 7 * 8))",
            "add(a, b, 1, (2 * 3), (4 + 5), add(6, (7 * 8)))",
        ),
        (
            "add(a + b + c * d / f + g)",
            "add((((a + b) + ((c * d) / f)) + g))",
        ),
        (
            "a * [1, 2, 3, 4][b * c] * d",
            "((a * ([1, 2, 3, 4][(b * c)])) * d)",
        ),
        (
            "add(a * b[2], b[1], 2 * [1, 2][1])",
            "add((a * (b[2])), (b[1]), (2 * ([1, 2][1])))",
        ),
    ];

    for (input, expect) in tests {
        let l = Lexer::new(input);
        let mut p = Parser::new(l);
        let program = p.parse_program().unwrap();

        assert_eq!(program.to_string(), expect);
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

#[test]
fn test_string_literal() {
    let input = "\"hello world\";";

    let l = Lexer::new(input);
    let mut p = Parser::new(l);
    let program = p.parse_program().unwrap();

    assert_eq!(program.statements.len(), 1);
    if let Statement::ExpressionStatement(Expression::StringLiteral(v)) = &program.statements[0] {
        assert_eq!(v, "hello world");
    } else {
        assert!(
            false,
            "expect a string literal, but a {:?}",
            &program.statements[0]
        );
    }
}
