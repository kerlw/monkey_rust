use crate::eval::ObjectWrapper;
use crate::lexer::lexer::Lexer;
use crate::parser::Parser;
use crate::eval::evaluator::eval;
use crate::parser::Result;

#[cfg(test)]
fn test_eval(input: &str) -> Result<ObjectWrapper> {
    let l = Lexer::new(input);
    let mut p = Parser::new(l);
    let program = p.parse_program()?;

    eval(&program)
}

#[test]
fn test_integer_object() {
    let cases = vec![("5", 5), ("10", 10)];
    for (input, expect) in cases {
        let obj = test_eval(input).unwrap();
        if let ObjectWrapper::Integer(v) = obj {
            assert_eq!(v, expect);
        } else {
            assert!(false);
        }
    }
}

#[test]
fn test_return_statement() {
    let cases = [
        ("return 10", ObjectWrapper::Integer(10)),
        ("return 10; 9;", ObjectWrapper::Integer(10)),
        ("return 2 * 5; 9", ObjectWrapper::Integer(10)),
        ("9; return 2 * 5; 9;", ObjectWrapper::Integer(10)),
        ("if (10 > 1) { if (10 > 1) { return 10; } return 1; }", ObjectWrapper::Integer(10)),
    ];

    for (input, expect) in cases {
        let obj = test_eval(input).unwrap();
        assert_eq!(obj, expect);
    }
}