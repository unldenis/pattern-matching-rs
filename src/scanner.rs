use std::error::Error;

use crate::utils::StringExt;

#[derive(Debug, PartialEq)]
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
    pub token_type : TokenType,
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
        
        Ok(())
    }
    
    fn scan_token(&mut self) -> Result<(), String> {
        let c = self.advance()?;
        
        match c {
            // '\n' | '\r' => {
            //     self.add_token(TokenType::EOF);
            // }
            ' ' => {}
            '+' => {
                self.add_token(TokenType::PLUS);
            }
            '=' => {
                self.add_token(TokenType::EQUAL);
            }
            '"' => {         
                self.string()?;       
            }
            '\n' => {
                self.add_token(TokenType::EOF);  
            }
            _ => {
                if c.is_alphabetic() {
                    while self.peek()?.is_alphanumeric()  {
                        self.advance()?;
                    }
                    self.add_token(TokenType::IDENTIFIER);
                    return Ok(());
                }
                return Err(format!("unexpected character '{}' at column '{}'", c, self.start));
            }

        }
        
        Ok(())
    }
    
    fn string(&mut self) -> Result<(), String> {
        while self.peek()? != '"' && !self.is_at_end() {
            if self.peek()? == '\n' {     
              return Err(String::from("unterminated string"));                
            } 
            self.advance()?;
        }
         
        if self.is_at_end() {
            return Err(String::from("unterminated string"));                
        }
         
        // The closing ".
        self.advance()?;
        
        let value = self.source.to_owned().substring(self.start + 1, self.current - 1);
        self.add_token(TokenType::STRING(value));
        Ok(())
    }   
    
    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }   
    
    
    
    fn peek(&self) -> Result<char, String> {
        self.source.chars().nth(self.current).ok_or(String::from("no more characters to scan"))
    }  
    
    fn advance(&mut self) -> Result<char, String> {
        let opt =self.source.chars().nth(self.current).ok_or(String::from("no more characters to scan"));
        self.current = self.current + 1;
        opt
    }  
    fn add_token(&mut self, token_type : TokenType) {
        self.tokens.push(Token { token_type: token_type, lexeme: self.source.to_owned().substring(self.start, self.current), from:self.start, to: self.current });
    }   
}