use crate::grammar::ast::{eq::AstEq, Expression, UnaryExpression, UnaryOp};
use crate::grammar::model::{HasSourceReference, WrightInput};
use nom::IResult;

impl UnaryOp {}

impl<T: Clone + std::fmt::Debug> UnaryExpression<T> {
    /// Name used in parser tracing.
    pub const TRACE_NAME: &'static str = "UnaryExpr";
}

impl<I: WrightInput> UnaryExpression<I> {
    /// Parse a unary expression in source code.
    pub fn parse(input: I) -> IResult<I, Self> {
        unimplemented!()
    }
}

impl<I: std::fmt::Debug + Clone> Into<Expression<I>> for UnaryExpression<I> {
    fn into(self) -> Expression<I> {
        Expression::UnaryExpression(self)
    }
}

impl<I: std::fmt::Debug + Clone> HasSourceReference<I> for UnaryExpression<I> {
    fn get_source_ref(&self) -> &I {
        &self.frag
    }
}

impl<I: Clone + std::fmt::Debug + PartialEq> AstEq for UnaryExpression<I> {
    fn ast_eq(fst: &Self, snd: &Self) -> bool {
        fst.op == snd.op && AstEq::ast_eq(&*fst.inner, &*snd.inner)
    }
}
