mod scanner;
mod utils;
mod parser;
mod walker;

use std::io::stdin;
use std::io::Write;
use std::io::stdout;

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
/* 
                for ele in scanner.tokens.iter() {
                    println!("{:?}", ele);
                }
*/
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

            
            if let Err(err) = walker::check_ast(&ast) {
                println!("ast invalid: {}", err);
                return;
            }

            match walker::evaluate(&ast) {
                Ok(str) => {
                    println!("{}", str);

                },
                Err(err) => {
                    println!("failed to evaluate the ast: {}", err);
                },
            }
        },
        Err(err) => {
            println!("failed to parse: {}", err);
        },
    }
} 

