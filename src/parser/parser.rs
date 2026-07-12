use std::iter::Peekable;

use super::{ast, Lexer, Token, Keyword};
use crate::errors::Result;

pub struct Parser<'a> {
    pub lexer: Peekable<Lexer<'a>>,
}   


impl Parser<'_> {
    pub fn parse() {

    }


    fn new(input: &str) -> Parser<'_> {
        return Parser { lexer: Lexer::new(input).peekable() }
    }

    fn next_if(&mut self, p: impl Fn(Token) -> bool) -> Option<Token> {
        self.lexer.peek().filter(|&&t| p(t))?;
        
    }


    // Everything related to an expression.

    fn parse_expression(&mut self) -> Result<ast::Expression> {
        self.logical_or()
    }

    fn logical_or(&mut self) -> Result<ast::Expression> {
        let left = self.logical_and();
        
        
    }

    fn logical_and(&mut self) -> Result<ast::Expression> {

    }

    fn equality(&mut self) -> Result<ast::Expression> {

    }

    fn comparison(&mut self) -> Result<ast::Expression> {

    }

    fn term(&mut self) -> Result<ast::Expression> {

    }

    fn factor(&mut self) -> Result<ast::Expression> {

    }

    fn unary(&mut self) -> Result<ast::Expression> {

    } 

    fn call(&mut self) -> Result<ast::Expression> {

    }

    fn arguments(&mut self) -> Result<ast::Expression> {

    }

    fn primary(&mut self) -> Result<ast::Expression> {

    }
}
