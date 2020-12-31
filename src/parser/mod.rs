use crate::lexer::{Symbol, Token};
use anyhow::{Context, Result};
use std::iter::Peekable;

#[derive(Debug, PartialEq)]
pub enum Expression {
    Int(i32),
    BinaryExp(Box<Expression>, Operator, Box<Expression>),
}

#[derive(Debug, PartialEq)]
pub enum Operator {
    Plus,
    Minus,
    Mult,
    Div,
}

pub fn parse_expression<'a, It: Iterator<Item = &'a Token>>(
    token_iterator: &mut Peekable<It>,
) -> Result<Expression> {
    let mut expr = parse_prefix(token_iterator).context("Failed to parse prefix expression")?;

    while let Some(&next_token) = token_iterator.peek() {
        expr = parse_infix(expr, token_iterator).context("Failed to parse infix expression")?;
    }

    Ok(expr)
}

fn parse_prefix<'a, It: Iterator<Item = &'a Token>>(
    token_iterator: &mut Peekable<It>,
) -> Result<Expression> {
    match token_iterator.next() {
        Some(token) => match token {
            Token::Int(i) => Ok(Expression::Int(*i)),
            _ => anyhow::bail!("Unexpected token: {:?}", token),
        },
        None => anyhow::bail!("No token left"),
    }
}

fn parse_infix<'a, It: Iterator<Item = &'a Token>>(
    left: Expression,
    token_iterator: &mut Peekable<It>,
) -> Result<Expression> {
    match token_iterator.next() {
        Some(token) => match token {
            Token::Operator(op) => {
                let operator = match op {
                    Symbol::Plus => Operator::Plus,
                    Symbol::Minus => Operator::Minus,
                    Symbol::Mult => Operator::Mult,
                    Symbol::Div => Operator::Div,
                };

                let right =
                    parse_expression(token_iterator).context("Failed to parse right expression")?;

                Ok(Expression::BinaryExp(
                    Box::new(left),
                    operator,
                    Box::new(right),
                ))
            }

            _ => anyhow::bail!("Unexpected token: {:?}", token),
        },
        None => anyhow::bail!("No token left"),
    }
}
