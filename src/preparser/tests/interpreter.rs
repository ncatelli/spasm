use std::convert::TryInto;

use crate::preparser::ast::{Expr, Interpreter, Node};
use crate::preparser::types::{Primitive, PrimitiveVariant};

#[test]
fn should_interpret_known_primitive_literal_to_self_when_target_types_match() {
    let node = Node::Expr(Expr::Literal(PrimitiveVariant::Uint8(Primitive::new(5u8))));

    assert_eq!(
        Ok(PrimitiveVariant::Uint8(Primitive::new(5u8))),
        node.interpret()
    )
}
