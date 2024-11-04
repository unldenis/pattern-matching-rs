use crate::utils::StringExt;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum ScannerError {
    #[error("no more characters to scan")]
    EndInput,

    #[error("unterminated string starting at column '{column}'")]
    UnterminatedString {
        column : usize
    },

    #[error("unexpected character '{character}' from '{from}' to '{to}'")]
    UnexpectedCharacter {
        character : char,
        from : usize,
        to : usize
    },
}

#[derive(Debug, PartialEq)]
pub enum TokenType {
  // Binary Op .
  PLUS, 
  EQUAL,

  // Literals.
  IDENTIFIER, 
  STRING(String),

  EOF
}

#[derive(Debug)]
pub struct Token {
    pub token_type : TokenType,
    pub lexeme : String,
    pub from : usize,
    pub to : usize
}

pub struct Scanner<'a> {
    source : &'a str,
    pub tokens : Vec<Token>,
    start : usize,
    current : usize,
}

impl<'a> Scanner<'a> {
    pub fn new(source : &'a str) -> Scanner<'a> {
        Scanner { source: source, tokens: Vec::new(), start: 0, current: 0 }
    }
    
    
    pub fn scan_tokens(&mut self) -> Result<(), ScannerError>  {

        while !self.is_at_end()  {
            self.start = self.current;
            self.scan_token()?;
        }
        
        Ok(())
    }
    
    fn scan_token(&mut self) -> Result<(), ScannerError> {
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
                return Err(ScannerError::UnexpectedCharacter { character: c, from: self.start, to: self.current });
            }

        }
        
        Ok(())
    }
    
    fn string(&mut self) -> Result<(), ScannerError> {
        let start_string_index = self.current - 1;
        while self.peek()? != '"' && !self.is_at_end() {
            if self.peek()? == '\n' {     
              return Err(ScannerError::UnterminatedString { column: start_string_index });                
            } 
            self.advance()?;
        }
         
        if self.is_at_end() {
            return Err(ScannerError::UnterminatedString { column: start_string_index });                
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
    
    
    
    fn peek(&self) -> Result<char, ScannerError> {
        self.source.chars().nth(self.current).ok_or(ScannerError::EndInput)
    }  
    
    fn advance(&mut self) -> Result<char, ScannerError> {
        let opt =self.source.chars().nth(self.current).ok_or(ScannerError::EndInput);
        self.current = self.current + 1;
        opt
    }  
    fn add_token(&mut self, token_type : TokenType) {
        self.tokens.push(Token { token_type: token_type, lexeme: self.source.to_owned().substring(self.start, self.current), from:self.start, to: self.current });
    }   
}