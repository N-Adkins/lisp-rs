#![allow(dead_code)]

use std::cell::RefCell;
use std::rc::Rc;

use crate::env::Env;
use crate::result::LispResult;
use crate::func::LispFunc;

#[derive(Clone)]
pub enum LispType {
    Func(LispFunc),
    List(Vec<LispType>),
    Int(i32),
    Symbol(String),
    String(String),
    Bool(bool),
    Nil,
}

impl LispType {

    pub fn evaluate(&self, env: Rc<RefCell<Env>>) -> LispResult<LispType> {

        match self {

            LispType::List(vec) => {

                if vec.is_empty() {
                    return Ok(LispType::Int(0));
                }

                let eval_vec = vec.clone();

                match eval_vec.first().unwrap() {

                    LispType::Symbol(symbol) => match symbol.as_str() {

                        "def!" => {

                            if eval_vec.len() != 3 {
                                return Err(String::from("Expected two arguments to \"def!\" declaration"));
                            }

                            if let LispType::Symbol(to_def) = eval_vec.get(1).unwrap() {
                                let value = eval_vec.get(2).unwrap().evaluate(Rc::clone(&env))?;
                                env.borrow_mut().set(to_def.as_str(), value.clone());
                                return Ok(value);
                            } else {
                                return Err(String::from("Failed to evaluate \"def!\" declaration"));
                            }
                            
                        }

                        "let*" => {

                            if eval_vec.len() != 3 {
                                return Err(String::from("Expected two arguments to \"let*\" declaration"));
                            }
                            
                            let new_env = Rc::new(RefCell::new(Env::new(Some(env))));

                            if let LispType::List(def_list) = eval_vec.get(1).unwrap() {
                                
                                let mut defs = def_list.clone();

                                if defs.len() % 2 != 0 {
                                    return Err(String::from("Found odd number of arguments for definitions in \"let*\" statement"));
                                }

                                while defs.len() > 0 {

                                    let symbol = if let LispType::Symbol(s) = defs.remove(0) {
                                        s
                                    } else {
                                        return Err(String::from("Expected symbol for \"let*\" definition"));
                                    };

                                    let value = defs.remove(0).evaluate(Rc::clone(&new_env))?;

                                    new_env.borrow_mut().set(symbol.as_str(), value);

                                }

                            } else {
                                return Err(String::from("Expected list of definitions as second argument to \"let*\" declaration"));
                            }

                            return eval_vec.get(2).unwrap().evaluate(new_env);

                        }

                        "fn*" => {

                            if eval_vec.len() != 3 {
                                return Err(String::from("Expected two arguments to \"fn*\" declaration"));
                            }

                            let arg_list = eval_vec.get(1).unwrap();
                            let body_list = eval_vec.get(2).unwrap();
                             
                            let mut func = LispFunc::new(Rc::clone(&env)); 
                            func.body = Box::new(body_list.clone());
                            func.args = if let LispType::List(vec) = arg_list.clone() { vec } else {
                                return Err(String::from("Arguments for function were not a list"));
                            };

                            Ok(LispType::Func(func))

                        }

                        "prn" => {

                            if eval_vec.len() != 2 {
                                return Err(String::from("Expected 1 argument to \"prn\" declaration"));
                            }

                            let eval = eval_vec.get(1).unwrap().evaluate(Rc::clone(&env))?;
                            eval.print();
                            
                            Ok(LispType::Nil)
                                                        
                        }

                        "list" => {

                            let mut list: Vec<LispType> = Vec::new();

                            for item in &eval_vec[1..] {
                                list.push(item.clone());
                            }

                            Ok(LispType::List(list))

                        }

                        "list?" => {

                            if eval_vec.len() != 2 {
                                return Err(String::from("Expected 1 argument to \"list?\" declaration"));
                            }

                            if let LispType::List(_) = eval_vec.get(1).unwrap() {
                                return Ok(LispType::Bool(true));
                            } else {
                                return Ok(LispType::Bool(false));
                            }
                            
                        }

                        "empty?" => {
                            
                            if eval_vec.len() != 2 {
                                return Err(String::from("Expected 1 argument to \"empty?\" declaration"));
                            }

                            let list = if let LispType::List(vec) = eval_vec.get(1).unwrap() {
                                vec
                            } else {
                                return Err(String::from("First argument to \"empty?\" declaration is not a list"));
                            };

                            Ok(LispType::Bool(list.is_empty()))

                        }

                        "count" => {
                            
                            if eval_vec.len() != 2 {
                                return Err(String::from("Expected 1 argument to \"count\" declaration"));
                            }

                            let list = if let LispType::List(vec) = eval_vec.get(1).unwrap() {
                                vec
                            } else {
                                return Err(String::from("First argument to \"count\" declaration is not a list"));
                            };

                            Ok(LispType::Int(list.len() as i32))

                        }

                        "do" => {

                            for item in &eval_vec[1..(eval_vec.len()-1)] {
                                item.clone().evaluate(Rc::clone(&env))?;
                            }

                            eval_vec.last().unwrap().evaluate(Rc::clone(&env))

                        }

                        _ => {
                            
                            let value = env.borrow_mut().get(symbol.as_str())?;

                            match value {
                                LispType::Func(func) => {
                                    return func.clone().call(&eval_vec[1..], env);
                                }
                                _ => return Ok(value),
                            }

                        }
                    }
                    _ => Err(String::from("Expected symbol at the start of list to be evaluated")),
                }
                
            }

            LispType::Symbol(s) => return env.borrow_mut().get(s.as_str()),
            LispType::String(s) => return Ok(LispType::String(s.clone())),
            LispType::Int(i) => return Ok(LispType::Int(*i)),

            _ => Err(String::from("Unhandled evaluation value")),

        }

    }

    pub fn print(&self) {
        match self {
            LispType::List(vec) => {
                let mut data = vec.clone();
                print!("(");
                for _ in 0..(data.len()) {
                    data.remove(0).print();
                    if data.len() != 0 {
                        print!(" ");
                    }
                }
                print!(")");
            },
            LispType::Int(i) => print!("{}", i),
            LispType::Symbol(s) => print!("{}", s),
            LispType::String(s) => print!("\"{}\"", s),
            LispType::Bool(b) => match *b {
                true => print!("true"),
                false => print!("false"),
            }
            LispType::Nil => print!("nil"),
            LispType::Func(_) => print!("#<function>"),
        }   
    }
    
    pub fn println(&self) {
        self.print();
        println!();
    }

}
