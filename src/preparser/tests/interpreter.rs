use crate::preparser::ast::*;
use crate::preparser::types::{Primitive, PrimitiveVariant};

#[test]
fn should_interpret_known_primitive_literal_to_self_when_target_types_match() {
    let node = Node::Expr(Expr::Literal(PrimitiveVariant::Uint8(Primitive::new(5u8))));

    assert_eq!(
        Ok(PrimitiveVariant::Uint8(Primitive::new(5u8))),
        node.interpret()
    )
}

#[test]
fn should_interpret_typed_add() {
    let node = Node::Expr(Expr::Binary(
        Operator::Plus,
        Box::new(Expr::Literal(PrimitiveVariant::Uint8(Primitive::new(5u8)))),
        Box::new(Expr::Literal(PrimitiveVariant::Uint8(Primitive::new(5u8)))),
    ));

    assert_eq!(
        Ok(PrimitiveVariant::Uint8(Primitive::new(10u8))),
        node.interpret()
    );

    let type_error_node = Node::Expr(Expr::Binary(
        Operator::Plus,
        Box::new(Expr::Literal(PrimitiveVariant::Uint16(Primitive::new(
            5u16,
        )))),
        Box::new(Expr::Literal(PrimitiveVariant::Uint8(Primitive::new(5u8)))),
    ));

    assert!(type_error_node.interpret().is_err());
}
