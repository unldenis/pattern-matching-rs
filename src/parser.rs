use crate::scanner::{Token, TokenType};

#[derive(Debug)]
pub enum Expr<'c> {

    Binary(Box<Expr<'c>>,  &'c Token, Box<Expr<'c>>),
    Literal(String)
}

pub struct Parser<'a> {
    tokens : &'a Vec<Token>
}

impl<'a> Parser<'a> {

    pub fn new(tokens : &'a Vec<Token>) -> Parser<'a> {
        Parser { tokens }
    }

    pub fn parse(&self) -> Result<Expr, String> {
        let mut current : usize = 0;
        self.parse_sum_expr(&mut current)
    }

    fn parse_sum_expr(&self, current : &mut usize) -> Result<Expr, String> {
        let mut expr = self.parse_literal_expr(current)?;
        while self.check(*current, &TokenType::PLUS)? {
            let operator = self.peek(*current)?;
            *current = *current + 1;
            let right = self.parse_literal_expr(current)?;
            expr = Expr::Binary(Box::new(expr), operator, Box::new(right))
        }
        return Ok(expr);
    }

    fn parse_literal_expr(&self, current : &mut usize) -> Result<Expr, String>  {
        match &self.peek(*current)?.token_type {
            TokenType::STRING(str) => {
                *current = *current + 1;
                Ok(Expr::Literal(str.to_owned()))
            },
            other => {
                Err(format!("Expected STRING token, but found {:?}", other))
            },
        }
    }

    fn check(&self, index : usize, token_type: &TokenType) -> Result<bool, String> {
        if self.is_at_end(index)? {
            return Ok(false);
        }
        return Ok(&self.peek(index)?.token_type == token_type);
    }

    fn peek(&self, index : usize) -> Result<&Token, String> {
        self.tokens.get(index).ok_or(format!("no token present on index {}", index))
    }
    fn is_at_end(&self, index : usize) -> Result<bool, String> {
        Ok(self.peek(index)?.token_type == TokenType::EOF)
    }


}