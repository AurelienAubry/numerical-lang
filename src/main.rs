mod lexer;
mod parser;

use lexer::Tokenizer;
use parser::parse_expression;

fn main() {
    let input = "1*234+3/4-7".to_string();
    let input_tokens = input.tokenize().unwrap();
    println!("Input tokens: {:?}", input_tokens);

    let exp = parse_expression(&mut input_tokens.iter().peekable());
    println!("{:?}", exp);
}
