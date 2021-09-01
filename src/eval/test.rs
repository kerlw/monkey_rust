use crate::eval::evaluator::Evaluator;
use crate::eval::ObjectWrapper;
use crate::lexer::lexer::Lexer;
use crate::parser::Parser;
use crate::parser::Result;

#[cfg(test)]
fn test_eval(input: &str) -> Result<ObjectWrapper> {
    let l = Lexer::new(input);
    let mut p = Parser::new(l);
    let program = p.parse_program()?;
    let mut evaluator = Evaluator::new(&program.statements);
    evaluator.eval()
}

#[test]
fn test_integer_object() {
    let cases = vec![("5", 5), ("10", 10)];
    for (input, expect) in cases {
        let obj = test_eval(input).unwrap();
        if let ObjectWrapper::Integer(v) = obj {
            assert_eq!(v, expect);
        } else {
            assert!(false, "{:?} is not an integer object.", obj);
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
        ("let a = 5; return 2 * a; 9;", ObjectWrapper::Integer(10)),
        (
            "if (10 > 1) { if (10 > 1) { return 10; } return 1; }",
            ObjectWrapper::Integer(10),
        ),
    ];

    for (input, expect) in cases {
        let obj = test_eval(input).unwrap();
        assert_eq!(obj, expect);
    }
}

#[test]
fn test_error_handle() {
    let cases = [("foobar", "identifier not found: foobar")];

    for (input, expect) in cases {
        let obj = test_eval(input).unwrap();
        if let ObjectWrapper::ErrorObject(v) = obj {
            assert_eq!(&v, expect);
        } else {
            assert!(false, "{:?} is not an error object.", obj);
        }
    }
}

#[test]
fn test_function_call() {
    let cases = [
        ("fn(x, y){ return x + y; }(5, 5);", 10i64),
        ("let add = fn(x, y) { return x + y; }; add(4, 6);", 10),
        ("let add = fn(x, y) { return x + y; }; add(add(1, 2), add(add(2, 2), add(1, 2)));", 10),
    ];

    for (input, expect) in cases {
        let obj = test_eval(input).unwrap();
        if let ObjectWrapper::Integer(v) = obj {
            assert_eq!(v, expect);
        } else {
            assert!(false, "expect integer: {}, got {:?}", expect, obj);
        }
    }
}
