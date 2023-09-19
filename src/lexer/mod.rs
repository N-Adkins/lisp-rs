#![allow(dead_code, unused_variables)]

use crate::token::Keyword;
use crate::token::Token;

pub struct Lexer { 
    current_position: usize,
    current_char: char,
    current_depth: u32,
    tokens: Vec<Token>,
    input: String,
}

impl Lexer {

    pub fn new(input: String) -> Self {
        Self {
            current_position: 0,
            current_char: ' ',
            current_depth: 0,
            tokens: Vec::new(),
            input,
        } 
    }

    pub fn tokenize(&mut self) {

        self.tokens.push(Token::ListBegin(0));

        while self.current_position < self.input.len() - 1 {
            
            self.next_char();
            
            if self.current_char == '(' {
                self.current_depth += 1;
                self.tokens.push(Token::ListBegin(self.current_depth));
                continue;
            }

            if self.current_char == ')' {
                self.tokens.push(Token::ListEnd(self.current_depth));
                if self.current_depth == 0 {
                     panic!("Unexpected ')' token");
                }
                self.current_depth -= 1;
                continue;
            }
            
            let mut slice = String::new();
            while !self.current_char.is_whitespace() {

                slice.push(self.current_char);

                match self.peek_next_char() {
                    '(' | ')' => break,
                    _ => {}
                }

                self.next_char();

            }
            
            match slice.parse() {
                Ok(v) => { self.tokens.push(Token::Int(v)); continue; }
                Err(_) => {}
            };

            if slice.len() == 1 {
                self.tokens.push(Token::Symbol(slice.chars().nth(0).unwrap()));
                continue;
            }

            if slice.chars().nth(0).unwrap() == '\"' && slice.chars().nth(slice.len() - 1).unwrap() == '\"' {
                slice.remove(0);
                slice.remove(slice.len() - 1);
                self.tokens.push(Token::String(slice));
                continue;
            }

            // is symbol
            let keyword = match &*slice.to_lowercase() {
                 "write-line" => Keyword::WriteLine,
                 _ => panic!("There is no such keyword \"{}\"", slice),
            };

            self.tokens.push(Token::Keyword(keyword));

        }

        if self.current_depth > 0 {
            panic!("Lists were not closed - missing at least {} closing tokens", self.current_depth);
        }

        self.tokens.push(Token::ListEnd(0));

    }

    pub fn get_tokens(&self) -> Vec<Token> {
        return self.tokens.to_vec();
    }

    fn next_char(&mut self) {

        self.current_char = match self.input.chars().nth(self.current_position) {
            Some(v) => v,
            None => panic!("Unexpected character at position {}", self.current_position),
        };

        self.current_position += 1;

    }

    fn peek_next_char(&self) -> char {
        match self.input.chars().nth(self.current_position) {
            Some(v) => v,
            None => panic!("Unexpected character at position {}", self.current_position),
        }
    }

}
