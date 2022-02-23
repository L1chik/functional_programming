use super::tokenizer::{Tokenizer, Token};
use crate::errors::ParseError;

////// STRUCTURES //////
pub struct Parser<'a> {
    tokenizer: Tokenizer<'a>,
    current_token: Token,
}

////// IMPLEMENTATIONS //////

impl<'a> Parser<'a> {
    pub fn new(expr: &'a str) -> Result<Self, ParseError> {
        let mut lexer = Tokenizer::new(expr);
        let cur_token = match lexer.next() {
            Some(token) => token,
            None => return Err(ParseError::InvalidOperator())
        };

        Ok(Parser {
            tokenizer: lexer,
            current_token: cur_token,
        })
    }

    // pub fn parse(&mut self) -> Resul<Node, ParseError>
}
