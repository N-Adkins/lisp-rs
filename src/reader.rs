#![allow(dead_code)]

use std::num::ParseIntError;
use regex::Regex;

pub trait Print {
     fn print(&self);
     fn println(&self) {
        self.print();
        println!();
     }
}

#[derive(Clone)]
pub enum LispType {
    List(LispList),
    Int(i32),
    Symbol(String),
    String(String),
    Bool(bool),
    Nil,
}

impl Print for LispType {

    fn print(&self) {
        match self {
            LispType::List(list) => list.print(),
            LispType::Int(i) => print!("{}", i),
            LispType::Symbol(s) | LispType::String(s) => print!("{}", s),
            LispType::Bool(b) => match *b {
                true => print!("true"),
                false => print!("false"),
            }
            LispType::Nil => print!("nil"),
        }   
    }

}

#[derive(Clone)]
pub struct LispList {
    data: Vec<LispType>,
}

impl Print for LispList {
    
    fn print(&self) {
        let mut data = self.data.clone();
        print!("(");
        for _ in 0..(data.len()) {
            data.remove(0).print();
            if data.len() != 0 {
                print!(" ");
            }
        }
        print!(")");
    }

}

pub struct Reader {
    pub tokens: Vec<String>,
    position: usize,
}

impl Reader {

    pub fn tokenize(input: String) -> Self {

        let regex = Regex::new(r#"[\s,]*(~@|[\[\]{}()'`~^@]|"(?:\\.|[^\\"])*"?|;.*|[^\s\[\]{}('"`,;)]*)"#).unwrap();
        let mut tokens: Vec<String> = Vec::new();

        for (_, [raw_token]) in regex.captures_iter(input.as_str()).map(|c| c.extract()) {
            
            if raw_token.trim().is_empty() {
                continue;
            }

            tokens.push(String::from(raw_token));

        }

        Self {
            tokens,
            position: 0,
        }

    }

    fn next(&mut self) -> Option<String> {
        if self.position > self.tokens.len() - 1 {
            return None;
        }
        let token = self.tokens[self.position].clone();
        self.position += 1;
        Some(token)
    }

    fn peek(&self) -> Option<String> {
        if self.position > self.tokens.len() - 1 {
            return None;
        }
        Some(self.tokens[self.position].clone())
    }

    pub fn read_form(&mut self) -> LispType {

        let current_char = match self.peek() {
            Some(token) => token.chars().nth(0).unwrap(), // Unwrap should be safe here
            None => panic!("How?"),
        };

        match current_char {
            '(' => self.read_list(),
            ')' => panic!("Found list closing token with no opener somehow"),
            _ => self.read_atom(),
        }

    }

    fn read_list(&mut self) -> LispType {

        self.next().unwrap(); // Should be left paren
        
        let mut list = LispList {
            data: Vec::new(),
        };

        loop {

            let current_char = match self.peek() {
                Some(token) => token.chars().nth(0).unwrap(), // Unwrap should be safe here
                None => panic!("Could not find end of list"),
            };

            if current_char == ')' {
                break;
            }

            list.data.push(self.read_form());

        };

        self.next().unwrap(); // Should be right paren

        LispType::List(list)
        
    }

    fn read_atom(&mut self) -> LispType {
        
        let token = match self.next() {
            Some(token) => token, // Unwrap should be safe here
            None => panic!("What?"),
        };

        let first_char = token.chars().nth(0).unwrap();
        if first_char.is_numeric() {
            let num_result: Result<i32, ParseIntError> = token.parse();
            match num_result {
                Ok(i) => return LispType::Int(i),
                Err(_) => {},
            }
        }

        if first_char == '"' && token.chars().nth(token.len() - 1).unwrap() == '"'{
            let mut str = token.clone();
            str.remove(0);
            str.remove(str.len() - 1);
            return LispType::String(str);
        }

        match token.as_str() {
            "true" => return LispType::Bool(true),
            "false" => return LispType::Bool(false),
            "nil" => return LispType::Nil,
            _ => {}
        }

        return LispType::Symbol(token)

    }

}
