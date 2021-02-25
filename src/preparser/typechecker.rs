use crate::preparser::types;

/// a simple trait to define type checking behavior.
pub trait TypeQuery {
    fn matches(&self) -> bool {
        false
    }
}

/// TypeMatchQuery represents a type query that asserts rather two types match.
#[derive(Debug, Clone, Copy)]
pub struct TypeMatchQuery<T, U>
where
    T: Kinded,
    U: Kinded,
{
    t: T,
    u: U,
}

impl<T, U> TypeMatchQuery<T, U>
where
    T: Kinded,
    U: Kinded,
{
    fn new(t: T, u: U) -> Self {
        Self { t, u }
    }
}

impl<T, U> TypeQuery for TypeMatchQuery<T, U>
where
    T: Kinded,
    U: Kinded,
{
    fn matches(&self) -> bool {
        self.t.kind() == self.u.kind()
    }
}

/// Kinded implements the trait to return a types kind within the typesystem.
pub trait Kinded {
    fn kind(&self) -> Kind;
}

/// Kind represents a type to be used for type resolution.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Kind {
    Primitive(types::PrimitiveType),
}

impl Kinded for Kind {
    fn kind(&self) -> Kind {
        *self
    }
}

impl From<types::PrimitiveType> for Kind {
    fn from(src: types::PrimitiveType) -> Self {
        Kind::Primitive(src)
    }
}

#[cfg(test)]
mod tests {
    use crate::preparser::typechecker::*;
    use crate::preparser::types::PrimitiveType;

    #[test]
    fn should_assert_type_matches() {
        let lhs = Kind::Primitive(PrimitiveType::Uint8);
        let rhs = Kind::Primitive(PrimitiveType::Uint8);
        let no_match_rhs = Kind::Primitive(PrimitiveType::Uint16);
        assert!(TypeMatchQuery::new(lhs, rhs).matches());
        assert!(!TypeMatchQuery::new(lhs, no_match_rhs).matches());
    }
}
