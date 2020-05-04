use crate::grammar::ast::{eq::AstEq, Expression, Parens};
use crate::grammar::model::{Fragment, HasFragment};
use crate::grammar::parsers::expression::ToExpression;
use crate::grammar::parsers::whitespace::token_delimiter;
use crate::grammar::parsers::with_input;
use nom::character::complete::{char as ch, multispace0};
use nom::combinator::map;
use nom::sequence::{delimited, terminated};
use nom::IResult;

impl<'s> Parens<'s> {
    fn inner(frag: Fragment<'s>) -> IResult<Fragment<'s>, Expression<'s>> {
        delimited(
            token_delimiter,
            delimited(
                terminated(ch('('), token_delimiter),
                Expression::parse,
                terminated(ch(')'), token_delimiter),
            ),
            token_delimiter,
        )(frag)
    }

    /// Parse parentheses and the expression between them in source code. Will
    /// ignore any whitespace before and after.
    pub fn parse(input: Fragment<'s>) -> IResult<Fragment<'s>, Self> {
        map(with_input(Self::inner), |(consumed, expr)| Parens {
            frag: consumed,
            inner: Box::new(expr),
        })(input)
    }
}

impl<'s> HasFragment<'s> for Parens<'s> {
    fn get_fragment(&self) -> Fragment<'s> {
        self.frag
    }
}

impl<'s> ToExpression<'s> for Parens<'s> {
    fn create_expr(self) -> Expression<'s> {
        Expression::Parens(self)
    }
}

impl<'s> AstEq for Parens<'s> {
    #[inline]
    fn ast_eq(fst: &Self, snd: &Self) -> bool {
        AstEq::ast_eq(&*fst.inner, &*snd.inner)
    }
}
