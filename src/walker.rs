
use crate::{parser::Expr, scanner::{self}};

pub fn check_ast(ast : &Expr) -> Result<(), String>{
    println!("AST: {:?}", ast);

    match ast {
        Expr::Binary(left, operator, right) => {
            if operator.token_type != scanner::TokenType::EQUAL {
                return Err("ast must start with match expr".into());
            } 
    
            let has_var_left = has_var(left);
            let has_var_right = has_var(right);
            
            if has_var_left && has_var_right {
                return Err("vars can not be on both sides of the equality".into());
            }
        }
        _ => {
            return Err("ast must start with binary expr".into());
        },
    }
    Ok(())

}

pub fn evaluate(ast : &Expr) -> Result<String, String> {
    Ok(match ast {
        Expr::Binary(left, operator , right) => {
           format!("{} {} {}", evaluate(left)?, operator.lexeme, evaluate(right)?)
        }
        Expr::Literal(str) => {
            format!("\"{}\"", str)
        }
        Expr::Var(token) => {
            token.lexeme.to_owned()
        }
    })
}

pub fn has_var(expr : &Expr) -> bool { 
    match expr {
        Expr::Binary(left, _, right) => {
            has_var(&left) || has_var(&right)
        },
        Expr::Literal(_) => {
            false
        },
        Expr::Var(_) => {
            true
        },
    }
}