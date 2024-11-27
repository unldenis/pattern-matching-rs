use std::error::Error;

use crate::utils::StringExt;

#[derive(Debug)]
pub enum TokenType {
  // Single-character tokens.
  PLUS, 

  // One or two character tokens.
  EQUAL,
  
  // Literals.
  IDENTIFIER, 
  STRING(String),

  EOF


}

#[derive(Debug)]
pub struct Token {
    token_type : TokenType,
    lexeme : String,
    from : usize,
    to : usize
}

pub struct Scanner<'a> {
    source : &'a str,
    pub tokens : Vec<Token>,
    start : usize,
    current : usize,
    line : usize
}

impl<'a> Scanner<'a> {
    pub fn new(source : &'a str) -> Scanner<'a> {
        Scanner { source: source, tokens: Vec::new(), start: 0, current: 0, line: 0 }
    }
    
    
    pub fn scan_tokens(&mut self) -> Result<(), String>  {

        while !self.is_at_end()  {
            self.start = self.current;
            self.scan_token()?;
        }
        
        self.tokens.push(Token { token_type: TokenType::EOF, lexeme: String::from("\0"), from: self.current, to: self.current });    
        Ok(())
    }
    
    fn scan_token(&mut self) -> Result<(), String> {
        let c = self.advance().ok_or("no more characters to scan")?;
        
        match c {
            // '\n' | '\r' => {
            //     self.add_token(TokenType::EOF);
            // }
            '+' => {
                self.add_token(TokenType::PLUS);
            }
            '=' => {
                self.add_token(TokenType::EQUAL);
            }
            _ => {
                self.add_token(TokenType::IDENTIFIER);
            }
        }
        
        Ok(())
    }
    
    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }   
    
    fn advance(&mut self) -> Option<char> {
        let opt = self.source.chars().nth(self.current);
        self.current = self.current + 1;
        opt
    }  
    fn add_token(&mut self, token_type : TokenType) {
        self.tokens.push(Token { token_type: token_type, lexeme: self.source.to_owned().substring(self.start, self.current), from:self.start, to: self.current });
    }   
}