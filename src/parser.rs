use crate::scanner::{Token, TokenType};
use thiserror::Error;


#[derive(Error, Debug)]
pub enum ParserError<'ctx> {
    #[error("no token present on index {0}")]
    TokenNotFoundAt(usize),

    #[error("expected {expected} but found {found:?} at column {column}")]
    UnexpectedToken {
        expected : String,
        found : &'ctx TokenType,
        column : usize
    },

    #[error("missing match operator")]
    MatchOperatorNotFound,
}

#[derive(Debug)]
pub enum Expr<'c> {
    
    Binary(Box<Expr<'c>>,  &'c Token, Box<Expr<'c>>),
    Literal(String),
    
    Var(&'c Token)
}

pub struct Parser<'a> {
    tokens : &'a Vec<Token>
}

impl<'a> Parser<'a> {

    pub fn new(tokens : &'a Vec<Token>) -> Parser<'a> {
        Parser { tokens }
    }

    pub fn parse(&self) -> Result<Expr, ParserError> {
        let mut current : usize = 0;
        self.parse_match_expr(&mut current)
    }
    
    fn parse_match_expr(&self, current : &mut usize) -> Result<Expr, ParserError> {
        let mut expr = self.parse_sum_expr(current)?;
        
        if self.check(*current, &TokenType::EQUAL)? {
            let operator = self.peek(*current)?;
            *current = *current + 1;
            let right = self.parse_sum_expr(current)?;
            expr = Expr::Binary(Box::new(expr), operator, Box::new(right))
        } else {
            return Err(ParserError::MatchOperatorNotFound);
        }
        return Ok(expr);
    }


    fn parse_sum_expr(&self, current : &mut usize) -> Result<Expr, ParserError> {
        let mut expr = self.parse_literal_expr(current)?;
        
        while self.check(*current, &TokenType::PLUS)? {
            let operator = self.peek(*current)?;
            *current = *current + 1;
            let right = self.parse_literal_expr(current)?;
            expr = Expr::Binary(Box::new(expr), operator, Box::new(right))
        }
        return Ok(expr);
    }

    fn parse_literal_expr(&self, current : &mut usize) -> Result<Expr, ParserError>  {
        let token =  self.peek(*current)?;
        match &token.token_type {
            TokenType::STRING(str) => {
                *current = *current + 1;
                Ok(Expr::Literal(str.to_owned()))
            },
            TokenType::IDENTIFIER => {
                *current = *current + 1;                
                Ok(Expr::Var(&token))  
            }
            other => {
                Err(ParserError::UnexpectedToken { expected: "STRING / IDENTIFIER".into(), found: other, column: token.from })
            },
        }
    }

    fn check(&self, index : usize, token_type: &TokenType) -> Result<bool, ParserError> {
        if self.is_at_end(index)? {
            return Ok(false);
        }
        return Ok(&self.peek(index)?.token_type == token_type);
    }

    fn peek(&self, index : usize) -> Result<&Token, ParserError> {
        self.tokens.get(index).ok_or( ParserError::TokenNotFoundAt(index))
    }
    fn is_at_end(&self, index : usize) -> Result<bool, ParserError> {
        Ok(self.peek(index)?.token_type == TokenType::EOF)
    }

}