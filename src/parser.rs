use crate::scanner::{Token, TokenType};


pub enum Expr {

    Binary(Box<Expr>,  Token, Box<Expr>),
    Literal(String)
}

pub struct Parser<'a> {
    tokens : &'a Vec<Token>,
    current : usize
}

impl<'a> Parser<'a> {

    pub fn new(tokens : &'a Vec<Token>) -> Parser<'a> {
        Parser { tokens, current: 0 }
    }

    fn expression(&mut self) -> Result<Expr, String> {
        self.parse_binary_expr()
    }

    fn parse_binary_expr(&mut self) -> Result<Expr, String> {
        let mut expr = self.parse_primary_expr()?;
        if self._match(&[TokenType::PLUS])? {
            let operator = self.previous()?;
            let right = self.parse_binary_expr()?;
            expr = Expr::Binary(Box::new(expr), operator.clone(), Box::new(right))
        }
        return Ok(expr);
    }

    fn parse_primary_expr(&self) -> Result<Expr, String>  {
        match &self.peek()?.token_type {
            TokenType::STRING(str) => {
                Ok(Expr::Literal(str.clone()))
            },
            other => {
                Err(format!("Expected STRING token, but found {:?}", other))
            },
        }
    }

    fn _match(&mut self, token_types : &[TokenType]) -> Result<bool, String> {
       for ele in token_types {
            if self.check(ele)? {
                self.advance()?;
                return Ok(true);
            }   
       }
      return Ok(false);
    }
    fn check(&self, token_type: &TokenType) -> Result<bool, String> {
        if self.is_at_end()? {
            return Ok(false);
        }
        return Ok(&self.peek()?.token_type == token_type);
    }


    fn previous(&self) -> Result<&Token, String> {
        self.tokens.get(self.current - 1).ok_or(format!("no token present on index {}", self.current - 1))
    }
    fn peek(&self) -> Result<&Token, String> {
        self.tokens.get(self.current).ok_or(format!("no token present on index {}", self.current))
    }
    fn is_at_end(&self) -> Result<bool, String> {
        Ok(self.peek()?.token_type == TokenType::EOF)
    }

    fn advance(&mut self) -> Result<&Token, String> {
        if !self.is_at_end()? {
            self.current += 1;
        }
        return self.previous();
    }


}