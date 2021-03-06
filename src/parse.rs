use crate::ast::Expr;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::alpha1;
use nom::character::complete::char;
use nom::multi::separated_nonempty_list;
use nom::IResult;

/// Parses an string slice into an Expression
/// in form of an AST
pub fn expression(i: &str) -> Result<(String, Expr), ()> {
    // we cheat a little here by removing all whitespace beforehand
    let string: String = i
        .chars()
        .filter(|c: &char| match c {
            ' ' | '\n' | '\r' | '\t' => false,
            _ => true,
        })
        .collect();

    match and_tree(&string) {
        Err(_) => Err(()),
        // TODO handle possibility of rest
        Ok((_, expr)) => Ok((string, expr)),
    }
}

fn and_tree(i: &str) -> IResult<&str, Expr> {
    let (rest, ands) = separated_nonempty_list(char('^'), or_tree)(i)?;

    let mut iter = ands.into_iter();
    let first = iter.next().unwrap();

    Ok((
        rest,
        iter.fold(first, |acc, expr| Expr::And(Box::new(acc), Box::new(expr))),
    ))
}

fn or_tree(i: &str) -> IResult<&str, Expr> {
    let (rest, or_list) = separated_nonempty_list(char('|'), implication_tree)(i)?;

    let mut iter = or_list.into_iter();
    let first = iter.next().unwrap();

    Ok((
        rest,
        iter.fold(first, |acc, expr| Expr::Or(Box::new(acc), Box::new(expr))),
    ))
}

fn implication_tree(i: &str) -> IResult<&str, Expr> {
    let (rest, impl_list) = separated_nonempty_list(tag("->"), biimpl_tree)(i)?;

    let mut iter = impl_list.into_iter();
    let first = iter.next().unwrap();

    Ok((
        rest,
        iter.fold(first, |acc, expr| Expr::Impl(Box::new(acc), Box::new(expr))),
    ))
}

fn biimpl_tree(i: &str) -> IResult<&str, Expr> {
    let (rest, impl_list) = separated_nonempty_list(tag("<->"), literal_or_expression)(i)?;

    let mut iter = impl_list.into_iter();
    let first = iter.next().unwrap();

    Ok((
        rest,
        iter.fold(first, |acc, expr| {
            Expr::Biimpl(Box::new(acc), Box::new(expr))
        }),
    ))
}

fn literal_or_expression(i: &str) -> IResult<&str, Expr> {
    let (rest, neg) = nom::combinator::opt(char('-'))(i)?;

    let (rest, expr) = alt((
        nom::sequence::delimited(char('('), and_tree, char(')')),
        alt((
            nom::combinator::map(alpha1, |s: &str| Expr::Var(s.to_string())),
            nom::combinator::map(alt((char('1'), char('0'))), |c| Expr::Const(c == '1')),
        )),
    ))(rest)?;

    Ok((
        rest,
        if neg.is_some() {
            Expr::Not(Box::new(expr))
        } else {
            expr
        },
    ))
}
