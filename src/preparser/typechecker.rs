/// a simple trait to define type checking behavior.
pub trait TypeCheck {
    fn matches() -> bool {
        false
    }
}
