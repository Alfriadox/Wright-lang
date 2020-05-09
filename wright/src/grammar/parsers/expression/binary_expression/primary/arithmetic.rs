use crate::grammar::ast::{
    Expression,
    BinaryOp
};
use crate::grammar::parsers::expression::binary_expression::primary::{base_primary, parser_left};
use nom::branch::alt;
use nom::IResult;
use crate::grammar::tracing::input::OptionallyTraceable;
use crate::grammar::tracing::trace_result;

/// Parse child of a lower precedence arithmetic operator.
pub fn arithmetic1_primary<I: OptionallyTraceable>(input: I) -> IResult<I, Expression<I>> {
    let trace = "BinaryExpr::arithmetic1_primary";
    trace_result(
        trace,
        alt((
            arithmetic2,
            base_primary
        ))(input.trace_start_clone(trace))
    )
}

/// Parse lower precedence arithmetic expression.
pub fn arithmetic1<I: OptionallyTraceable>(input: I) -> IResult<I, Expression<I>> {
    let trace = "BinaryExpr::arithmetic1";
    trace_result(
        trace,
        parser_left(
            arithmetic1_primary,
            BinaryOp::parse_arithmetic_operator1
        )(input.trace_start_clone(trace))
    )
}

/// Parse higher precedence arithmetic expression.
pub fn arithmetic2<I: OptionallyTraceable>(input: I) -> IResult<I, Expression<I>> {
    let trace = "BinaryExpr::arithmetic2";
    trace_result(
        trace,
        parser_left(
            base_primary,
            BinaryOp::parse_arithmetic_operator2
        )(input.trace_start_clone(trace))
    )
}
