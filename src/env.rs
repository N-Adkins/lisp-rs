#![allow(dead_code)]

use std::{collections::HashMap, rc::Rc, cell::RefCell};

use crate::{lisp_type::LispType, result::LispResult};

#[derive(Clone)]
pub struct Env {
    parent: Option<Rc<RefCell<Env>>>,
    data: HashMap<String, LispType>,
}

impl Env {
    
   pub fn new(parent: Option<Rc<RefCell<Env>>>) -> Self {
        Self {
            parent,
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
            if let Some(parent) = &self.parent {
                parent.borrow().find(key)
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

impl std::fmt::Debug for Env {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Env")
            .finish()
    } 
}
