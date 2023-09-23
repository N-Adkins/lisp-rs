#![allow(dead_code)]

use std::collections::HashMap;

use crate::lisp_type::LispType;

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

}
