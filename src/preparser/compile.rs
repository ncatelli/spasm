use crate::preparser::ast;

pub trait Compile<I> {
    type Output;
    type Error;

    fn from_ast(input: Vec<ast::Node>) -> Result<Self::Output, Self::Error>;
}
