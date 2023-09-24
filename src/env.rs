#![allow(dead_code)]

use std::collections::HashMap;

use crate::{lisp_type::LispType, result::LispResult};

pub struct Env<'a> {
    parent: Option<&'a Env<'a>>,
    children: Vec<Env<'a>>,
    data: HashMap<String, LispType>,
}

impl<'a> Env<'a> {
    
    pub fn new(parent: Option<&'a Env<'a>>) -> Self {
        Self {
            parent,
            children: Vec::new(),
            data: HashMap::new(),
        }
    }

    pub fn set(&mut self, key: &str, value: LispType) {
        self.data.insert(String::from(key), value);
    }

    pub fn find(&self, key: &str) -> Option<LispType> {
        if let Some(value) = self.data.get(&String::from(key)) {
            Some(value.clone())
        } else {
            if let Some(parent) = self.parent {
                parent.find(key)
            } else {
                None
            }
        }
    }

    pub fn get(&self, key: &str) -> LispResult<LispType> {
        match self.find(key) {
            Some(value) => Ok(value),
            None => Err(format!("Failed to find symbol \"{}\"", key)),
        }
    }

}
