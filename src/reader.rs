#![allow(dead_code)]

use crate::{lisp_type::LispType, result::LispResult};

use std::num::ParseIntError;
use regex::Regex;

pub struct Reader {
    pub tokens: Vec<String>,
    position: usize,
}

impl Reader {

    pub fn tokenize(input: String) -> Self {

        let regex = Regex::new(r#"[\s,]*(~@|[\[\]{}()'`~^@]|"(?:\\.|[^\\"])*"?|;.*|[^\s\[\]{}('"`,;)]*)"#).unwrap();
        let mut tokens: Vec<String> = Vec::new();
        
        tokens.push(String::from("("));
        tokens.push(String::from("do"));

        for (_, [raw_token]) in regex.captures_iter(input.as_str()).map(|c| c.extract()) {
            
            if raw_token.trim().is_empty() {
                continue; 
            }

            if raw_token.chars().nth(0).unwrap() == ';' {
                continue;
            }

            tokens.push(String::from(raw_token));

        }

        tokens.push(String::from(")"));

        Self {
            tokens,
            position: 0,
        }

    }

    fn next(&mut self) -> LispResult<String> {
        if self.position > self.tokens.len() - 1 {
            return Err(String::from("Attempted to access token out of bounds"));
        }
        let token = self.tokens[self.position].clone();
        self.position += 1;
        Ok(token)
    }

    fn peek(&self) -> LispResult<String> {
        if self.position > self.tokens.len() - 1 {
            return Err(String::from("Attempted to view token out of bounds"));
        }
        Ok(self.tokens[self.position].clone())
    }

    pub fn read_form(&mut self) -> LispResult<LispType> {

        let token = self.peek()?;
        let current_char = token.chars().nth(0).unwrap();

        match current_char {
            '(' => self.read_list(),
            ')' => Err(String::from("Found list closing token with no opener somehow")),
            _ => self.read_atom(),
        }

    }

    fn read_list(&mut self) -> LispResult<LispType> {

        self.next()?; // Should be left paren
        
        let mut list: Vec<LispType> = Vec::new();

        loop {

            let token = self.peek()?;
            let current_char = token.chars().nth(0).unwrap();

            if current_char == ')' {
                break;
            }
            
            let value = self.read_form()?;
            list.push(value);

        };

        self.next()?; // Should be right paren

        Ok(LispType::List(list))
        
    }

    fn read_atom(&mut self) -> LispResult<LispType> {
        
        let token = self.next()?;

        let first_char = token.chars().nth(0).unwrap();
        if first_char.is_numeric() || first_char == '-' {
            let num_result: Result<i32, ParseIntError> = token.parse();
            match num_result {
                Ok(i) => return Ok(LispType::Int(i)),
                Err(_) => {},
            }
        }
        
        // Unwrapping last char should be safe as it uses current length to get it
        // Does >= 2 to confirm that it isn't just a single quote being read so it can
        // be passed to symbol processing if it's just one
        if first_char == '"' && token.ends_with('"') && token.len() >= 2 {
            let mut str = token.clone();
            str.remove(0);
            str.remove(str.len() - 1);
            return Ok(LispType::String(str));
        }

        match token.as_str() {
            "true" => return Ok(LispType::Bool(true)),
            "false" => return Ok(LispType::Bool(false)),
            "nil" => return Ok(LispType::Nil),
            _ => {}
        }

        Ok(LispType::Symbol(token))

    }

}
