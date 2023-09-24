#![allow(dead_code)]

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

    pub fn evaluate(&self, env: &mut Env) -> LispResult<LispType> {

        match self {

            LispType::List(vec) => {

                if vec.is_empty() {
                    return Ok(LispType::Int(0));
                }

                let eval_vec = vec.clone();

                match eval_vec.first().unwrap() {

                    LispType::Symbol(symbol) => match symbol.as_str() {

                        "def!" => {

                            if vec.len() > 3 {
                                return Err(String::from("Expected two arguments to \"def!\" declaration"));
                            }

                            if let LispType::Symbol(to_def) = eval_vec.iter().nth(1).unwrap() {
                                let value = eval_vec.iter().nth(2).unwrap().evaluate(env)?;
                                env.set(to_def.as_str(), value.clone());
                                return Ok(value);
                            } else {
                                return Err(String::from("Failed to evaluate \"def!\" declaration"));
                            }
                            
                        }

                        "let*" => {

                            Err(String::from(""))

                        }

                        _ => {
                            
                            let value = env.get(symbol.as_str())?;

                            match value {
                                LispType::Func(func) => {
                                    return func.call(&eval_vec[1..], env);
                                }
                                _ => return Ok(value),
                            }

                        }
                    }
                    _ => Err(String::from("Expected symbol at the start of list to be evaluated")),
                }
                
            }

            LispType::Symbol(s) => return env.get(s.as_str()),

            LispType::Int(i) => return Ok(LispType::Int(*i)),

            _ => Err(String::from("Unhandled evaluation value")),

        }

    }

    /* fn evaluate_operator_list(&self, eval_vec: &mut Vec<LispType>) -> LispResult<LispType> {

        let func_result: LispResult<fn(i32, i32) -> i32> = match eval_vec.remove(0) {
            LispType::Symbol(symbol) => match symbol.get_symbol_func() {
                Some(func) => Ok(func),
                None => return Err(format!("Attempted to evaluate unhandled symbol \"{}\"", symbol)),
            },
            _ => Err(String::from("Expected symbol at the start of list to evaluate")),
        };

        let eval_func = func_result?;
        
        if eval_vec.len() != 2 {
            return Err(String::from("Expected 2 arguments for symbol evaluation"));
        }

        let a = match eval_vec.remove(0).evaluate()? {
            LispType::Int(i) => i,
            val @ _ => return Err(String::from("Expected integer when evaluating argument for operator"))
        };

        let b = match eval_vec.remove(0).evaluate()? {
            LispType::Int(i) => i,
            val @ _ => return Err(String::from("Expected integer when evaluating argument for operator"))
        };

        return Ok(LispType::Int(eval_func(a, b)));

    } */

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
            LispType::Func(_) => {},
        }   
    }
    
    pub fn println(&self) {
        self.print();
        println!();
    }

}
