mod scanner;
mod utils;

use std::io::stdin;
use std::io::Write;
use std::io::stdout;

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
        
        let trimmed = input.trim_end();
        

     
        let mut scanner = scanner::Scanner::new(trimmed);
        
        match scanner.scan_tokens() {
            Ok(_) => {
                println!("scan OK");
                for ele in scanner.tokens.iter() {
                    println!("{:?}", ele);
                }
            },
            Err(error) => {
                println!("failed to scan tokens: {}", error);
            },
        };
            
        // clear input string        
        input.clear();
        
    }

}
