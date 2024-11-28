mod scanner;
mod utils;
mod parser;

use std::io::stdin;
use std::io::Write;
use std::io::stdout;

use parser::Expr;
use parser::Parser;
use scanner::Token;

fn main() {

    let mut input = String::new();    
    
    loop {
        print!("> ");
        
        if let Err(error) = Write::flush(&mut stdout()) {
            println!("failed to flush stdout: {}", error);
            continue;         
        }  
        
        if let Err(error) = stdin().read_line(&mut input) {
            println!("failed to read line: {}", error);
            continue;         
        } 
        
        // push in the end

        
        let mut trimmed = input.trim_end().to_owned();
        trimmed.push('\n');
     
        let mut scanner = scanner::Scanner::new(&trimmed);
        
        match scanner.scan_tokens() {
            Ok(_) => {
                for ele in scanner.tokens.iter() {
                    println!("{:?}", ele);
                }

                parse(&scanner.tokens);
            },
            Err(error) => {
                println!("failed to scan tokens: {}", error);
            },
        };
            
        // clear input string        
        input.clear();
        
    }

}

fn parse<'a>(tokens : &'a Vec<Token>) {
    
    let parser = Parser::new(tokens);


   match parser.parse() {
        Ok(ast) => {
        
            println!("{:?}", ast);
        },
        Err(err) => {
            println!("failed to parse: {}", err);
        },
    }
} 