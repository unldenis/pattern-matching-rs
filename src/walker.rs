
use crate::{parser::Expr, scanner::{self, TokenType}};

pub fn check_ast(ast : &Expr) -> Result<(), String>{
    println!("AST: {:?}", ast);

    match ast {
        Expr::Binary(left, operator, right) => {
            if(operator.token_type != scanner::TokenType::EQUAL) {
                return Err("ast must start with match expr".into());
            } 
    
            let hasVarLeft = hasVar(left);
            let hasVarRight = hasVar(right);
            
            if hasVarLeft && hasVarRight {
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

pub fn hasVar(binaryExpr : &Expr) -> bool { 
    match binaryExpr {
        Expr::Binary(left, _, right) => {
            hasVar(&left) || hasVar(&right)
        },
        Expr::Literal(_) => {
            false
        },
        Expr::Var(_) => {
            true
        },
    }
}