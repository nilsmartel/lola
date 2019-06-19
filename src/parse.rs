use crate::ast::Expr;
use nom::character::complete::char;
use nom::multi::separated_nonempty_list;
use nom::IResult;

pub fn expression(i: &str) -> IResult<&str, Expr> {
    let (rest, ands) = separated_nonempty_list(char('^'), ors)(i)?;

    let mut iter = ands.into_iter();
    let first = iter.next().unwrap();

    Ok((
        rest,
        iter.fold(first, |acc, expr| Expr::And(Box::new(acc), Box::new(expr))),
    ))
}

fn ors(i: &str) -> IResult<&str, Expr> {
    unimplemented!();
}
