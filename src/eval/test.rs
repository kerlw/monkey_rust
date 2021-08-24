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

    Ok(eval(&program))
}

#[test]
fn test_integer_object() {
    let cases = vec![("5", 5), ("10", 10)];
    for (input, expect) in cases {
        let obj = test_eval(input).unwrap();
        if let ObjectWrapper::Integer(v) = obj {
            assert_eq!(v, expect);
        } else {

        }
    }
}