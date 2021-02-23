use ast::Expr;
use types::Primitive;

use crate::preparser::{ast, interpreter, types};

#[test]
fn should_interpret_known_primitive_literal_to_self_when_target_types_match() {
    let node: Expr<Primitive<u8>> =
        interpreter::WalkNode::walk_node(ast::Expr::Literal(types::Primitive::new(5u8))).unwrap();

    assert_eq!(types::Primitive::new(5u8), node.into())
}

#[test]
fn should_interpret_primitive_literal_to_coerced_type() {
    let node: Expr<Primitive<u16>> =
        interpreter::WalkNode::walk_node(ast::Expr::Literal(types::Primitive::new(5u8))).unwrap();

    assert_eq!(types::Primitive::new(5u16), node.into())
}
