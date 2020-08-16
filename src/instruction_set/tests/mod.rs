use crate::asm;

#[test]
fn test_statement_formatter_should_pretty_print_an_ast() {
    let expr = Stmt::Expression(Expr::Unary(UnaryExpr::Minus(Box::new(Expr::Primary(
        obj_number!(123.0),
    )))));

    assert_eq!("(Expression (- 123))".to_string(), format!("{}", expr))
}
