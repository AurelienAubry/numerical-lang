use anyhow::{Context, Result};
use std::option::Option::Some;

#[derive(Debug, PartialEq)]
pub enum Symbol {
    Plus,
    Minus,
    Mult,
    Div,
}

#[derive(Debug, PartialEq)]
pub enum Token {
    Int(i32),
    Operator(Symbol),
}

pub trait Tokenizer {
    fn tokenize(&self) -> Result<Vec<Token>>;
}
impl Tokenizer for String {
    fn tokenize(&self) -> Result<Vec<Token>> {
        let mut chars_it = self.chars().peekable();
        let mut tokens = vec![];

        while let Some(&ch) = chars_it.peek() {
            match ch {
                '0'..='9' => {
                    let mut numbers_chars_vec = vec![];
                    while let Some(&ch) = chars_it.peek() {
                        if ch.is_numeric() {
                            chars_it
                                .next()
                                .context("Failed to iter to next character")?;
                            numbers_chars_vec.push(ch);
                        } else {
                            break;
                        }
                    }
                    let integer_str: String = numbers_chars_vec.into_iter().collect();
                    let integer = integer_str.parse::<i32>().with_context(|| {
                        format!("Failed to parse integer from string: {}", integer_str)
                    })?;

                    tokens.push(Token::Int(integer));
                }
                '+' => {
                    chars_it
                        .next()
                        .context("Failed to iter to next character")?;
                    tokens.push(Token::Operator(Symbol::Plus))
                }
                '-' => {
                    chars_it
                        .next()
                        .context("Failed to iter to next character")?;
                    tokens.push(Token::Operator(Symbol::Minus))
                }
                '*' => {
                    chars_it
                        .next()
                        .context("Failed to iter to next character")?;
                    tokens.push(Token::Operator(Symbol::Mult))
                }
                '/' => {
                    chars_it
                        .next()
                        .context("Failed to iter to next character")?;
                    tokens.push(Token::Operator(Symbol::Div))
                }
                _ => anyhow::bail!("Invalid character: {}", ch),
            }
        }
        Ok(tokens)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenize() {
        let a = "1*234+3/4-7".to_string();
        let a_tokens = a.tokenize().unwrap();
        let expected_a_tokens = vec![
            Token::Int(1),
            Token::Operator(Symbol::Mult),
            Token::Int(234),
            Token::Operator(Symbol::Plus),
            Token::Int(3),
            Token::Operator(Symbol::Div),
            Token::Int(4),
            Token::Operator(Symbol::Minus),
            Token::Int(7),
        ];

        assert_eq!(a_tokens, expected_a_tokens);

        let b = "".to_string();
        let b_tokens = b.tokenize().unwrap();
        assert_eq!(b_tokens, vec![]);
    }

    #[test]
    fn test_tokenize_error() {
        let a = "1.0*234.7+3/4-7".to_string();
        let a_tokens = std::panic::catch_unwind(|| a.tokenize().unwrap());
        assert!(a_tokens.is_err());

        let b = "ejaa".to_string();
        let b_tokens = std::panic::catch_unwind(|| b.tokenize().unwrap());
        assert!(b_tokens.is_err());
    }
}
