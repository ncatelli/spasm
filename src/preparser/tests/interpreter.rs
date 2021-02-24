use crate::preparser::ast::{Expr, Interpreter, Literal, Node};
use crate::preparser::types::Primitive;

#[test]
fn should_interpret_known_primitive_literal_to_self_when_target_types_match() {
    let node = Node::Expr(Expr::Literal(Literal::U8(Primitive::new(5u8))));

    assert_eq!(Ok(Primitive::new(5u8)), node.interpret())
}
