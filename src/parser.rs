use std::ops::Index;

use crate::token::Keyword;
use crate::token::Token;

enum ListValue {
    List(Vec<ListValue>),
    Int(i32),
    Symbol(char),
    Keyword(Keyword),
    String(String),
}

fn actual_to_string(value: &ListValue, str: &mut String) {
    match value {

        ListValue::Int(v) => str.push_str(&*(v.to_string())),
        ListValue::Symbol(v) => str.push(*v),

        ListValue::String(v) => {
            str.push('\"');
            str.push_str(&*v);
            str.push('\"');
        }

        ListValue::Keyword(v) => match *v {
            Keyword::WriteLine => str.push_str("write-line"),
        }

        ListValue::List(list) => {
            str.push('(');
            for nested_val in list {
                actual_to_string(nested_val, str);
            }
            str.push(')');
        }
    }
    str.push(' '); 
}

impl ListValue {

    pub fn to_string(&self) -> String {
        let mut str: String = String::from("");
        actual_to_string(self.to_owned(), &mut str);
        str
    }

}

pub struct Parser {
    tokens: Vec<Token>,
    root_list: ListValue,
}

impl Parser {

    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            root_list: ListValue::List(Vec::new()), // Placeholder
        }
    }

    pub fn parse(&mut self) {
        self.root_list = self.parse_list();
        println!("{}", self.root_list.to_string());
    }

    fn parse_any(&mut self) -> ListValue {
        
        match self.peek_token() {

            Some(token) => match token {

                Token::ListBegin(_) => self.parse_list(),

                // Trivial conversions
                Token::Symbol(symbol) => { self.eat_token(); ListValue::Symbol(symbol) }
                Token::Int(int) =>  { self.eat_token(); ListValue::Int(int) }
                Token::Keyword(keyword) => { self.eat_token(); ListValue::Keyword(keyword) }
                Token::String(string) => { self.eat_token(); ListValue::String(string) }

                _ => panic!("Unhandled token of type {}", token.to_string()),

            }

            None => panic!("Failed to fetch token, expected any"),
        }

    }

    fn parse_list(&mut self) -> ListValue {
        
        self.eat_token(); // open paren

        let mut list: Vec<ListValue> = Vec::new();

        while match self.peek_token() {
            Some(token) => match token {
                Token::ListEnd(_) => false,
                _ => true,
            }
            None => false,
        } {
            list.push(self.parse_any());
        }

        ListValue::List(list)

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
