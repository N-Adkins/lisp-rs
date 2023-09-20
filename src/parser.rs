use std::ops::Index;

use crate::ast::AST;
use crate::token::Token;

pub struct Parser {
    tokens: Vec<Token>,
    root_list: AST,
}

impl Parser {

    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            root_list: AST::List(Vec::new()), // Placeholder
        }
    }

    pub fn get_list(&self) -> AST {
        self.root_list.clone()
    }

    pub fn parse(&mut self) {
        self.root_list = self.parse_list();
        println!("{}", self.root_list.to_string());
    }

    fn parse_any(&mut self) -> AST {
        
        match self.peek_token() {

            Some(token) => match token {

                Token::ListBegin(_) => self.parse_list(),

                // Trivial conversions
                Token::Symbol(symbol) => { self.eat_token(); AST::Symbol(symbol) }
                Token::Int(int) =>  { self.eat_token(); AST::Int(int) }
                Token::Keyword(keyword) => { self.eat_token(); AST::Keyword(keyword) }
                Token::String(string) => { self.eat_token(); AST::String(string) }

                _ => panic!("Unhandled token of type {}", token.to_string()),

            }

            None => panic!("Failed to fetch token, expected any"),
        }

    }

    fn parse_list(&mut self) -> AST {
        
        self.eat_token(); // open paren

        let mut list: Vec<AST> = Vec::new();
        
        loop {

            list.push(self.parse_any());

            match self.peek_token() {
                Some(token) => match token {
                    Token::ListEnd(_) => break,
                    _ => {},
                }
                None => break,
            }

        };

        self.eat_token();

        AST::List(list)

    }

    fn eat_token(&mut self) -> Option<Token> {
        if self.tokens.is_empty() {
            return None;
        } 
        Some(self.tokens.remove(0))
    }

    fn peek_token(&mut self) -> Option<Token> {
        if self.tokens.len() <= 1 {
            return None;
        }
        Some(self.tokens.index(1).clone())
    }

}
