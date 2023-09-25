#![allow(dead_code)]

use std::cell::RefCell;
use std::rc::Rc;

use crate::env::Env;
use crate::result::LispResult;
use crate::func::LispFunc;

#[derive(Clone, Debug)]
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

                match eval_vec.first().unwrap().evaluate(Rc::clone(&env))? {

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

                        "type?" => {

                            if eval_vec.len() != 3 {
                                return Err(String::from("Expected 2 arguments to \"type?\" declaration"));
                            }
                             
                            return Ok(LispType::Bool(std::mem::discriminant(&eval_vec.get(1).unwrap().evaluate(Rc::clone(&env))?) == 
                                std::mem::discriminant(&eval_vec.get(2).unwrap().evaluate(Rc::clone(&env))?)));

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

                        "if" => {
                            
                            if eval_vec.len() != 3 && eval_vec.len() != 4 {
                                return Err(String::from("Expected 2 or 3 arguments to \"if\" declaration"));
                            }

                            let eval = eval_vec.get(1).unwrap().evaluate(Rc::clone(&env))?;
                            let result = match eval {
                                LispType::Nil => false,
                                LispType::Bool(false) => false,
                                _ => true,
                            };

                            if result {
                                return eval_vec.get(2).unwrap().evaluate(Rc::clone(&env));
                            } else {
                                if eval_vec.len() == 4 {
                                    return eval_vec.get(3).unwrap().evaluate(Rc::clone(&env));
                                } else {
                                    return Ok(LispType::Nil);
                                }
                            }

                        }

                        "and" | "&&" => {

                            if eval_vec.len() != 3 {
                                return Err(String::from("Expected 2 arguments to \"and\" declaration"));
                            }

                            let a = if let LispType::Bool(val) = eval_vec.get(1).unwrap().evaluate(Rc::clone(&env))? {
                                val
                            } else {
                                return Err(String::from("First argument to \"and\" declaration is not boolean"));
                            };

                            let b = if let LispType::Bool(val) = eval_vec.get(2).unwrap().evaluate(Rc::clone(&env))? {
                                val
                            } else {
                                return Err(String::from("Second argument to \"and\" declaration is not boolean"));
                            };

                            Ok(LispType::Bool(a && b))
                        
                        }

                        "or" | "||" => {

                            if eval_vec.len() != 3 {
                                return Err(String::from("Expected 2 arguments to \"or\" declaration"));
                            }

                            let a = if let LispType::Bool(val) = eval_vec.get(1).unwrap().evaluate(Rc::clone(&env))? {
                                val
                            } else {
                                return Err(String::from("First argument to \"or\" declaration is not boolean"));
                            };

                            let b = if let LispType::Bool(val) = eval_vec.get(2).unwrap().evaluate(Rc::clone(&env))? {
                                val
                            } else {
                                return Err(String::from("Second argument to \"or\" declaration is not boolean"));
                            };

                            Ok(LispType::Bool(a || b))
                        
                        }

                        "xor" | "^" => {

                            if eval_vec.len() != 3 {
                                return Err(String::from("Expected 2 arguments to \"xor\" declaration"));
                            }

                            let a = if let LispType::Bool(val) = eval_vec.get(1).unwrap().evaluate(Rc::clone(&env))? {
                                val
                            } else {
                                return Err(String::from("First argument to \"xor\" declaration is not boolean"));
                            };

                            let b = if let LispType::Bool(val) = eval_vec.get(2).unwrap().evaluate(Rc::clone(&env))? {
                                val
                            } else {
                                return Err(String::from("Second argument to \"xor\" declaration is not boolean"));
                            };

                            Ok(LispType::Bool(a ^ b))
                        
                        }


                        "not" | "!" => {

                            if eval_vec.len() != 2 {
                                return Err(String::from("Expected 1 argument to \"not\" declaration"));
                            }

                            let a = if let LispType::Bool(val) = eval_vec.get(1).unwrap().evaluate(Rc::clone(&env))? {
                                val
                            } else {
                                return Err(String::from("First argument to \"or\" declaration is not boolean"));
                            };

                            Ok(LispType::Bool(!a))
                        
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

                    LispType::Func(func) => {
                        return func.clone().call(&eval_vec[1..], env);
                    }

                    _ => Err(String::from("Expected symbol or function at the start of list to be evaluated")),

                }
                
            }

            LispType::Symbol(s) => return if let Some(found) = env.borrow_mut().find(s.as_str()) {
                Ok(found)
            } else {
                match s.as_str() {
                    "nil" => return Ok(LispType::Nil),
                    "true" => return Ok(LispType::Bool(true)),
                    "false" => return Ok(LispType::Bool(false)),
                    _ => Ok(LispType::Symbol(s.clone())),
                }
            },

            t @ LispType::String(_) => return Ok(t.clone()),
            t @ LispType::Bool(_) => return Ok(t.clone()),
            t @ LispType::Int(_) => return Ok(t.clone()),
            t @ LispType::Nil => return Ok(t.clone()),

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
