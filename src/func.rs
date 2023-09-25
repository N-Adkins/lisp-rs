use std::cell::RefCell;
use std::rc::Rc;

use crate::env::Env;
use crate::lisp_type::LispType;
use crate::result::LispResult;

#[derive(Clone, Debug)]
pub struct LispFunc {
    pub body: Box<LispType>, // Expected to be a list that can be evaluated
    pub args: Vec<LispType>, // Expected to be symbols
    pub operator_func: Option<fn(LispType, LispType) -> LispResult<LispType>>,
    pub internal_env: Rc<RefCell<Env>>,
}

impl LispFunc {

    pub fn new(outer_env: Rc<RefCell<Env>>) -> Self {
        Self {
            body: Box::new(LispType::List(Vec::new())), // temp
            args: Vec::new(),
            operator_func: None,
            internal_env: Rc::new(RefCell::new(Env::new(Some(outer_env)))),
        }
    }

    pub fn new_operator(operator_func: fn(LispType, LispType) -> LispResult<LispType>, outer_env: Rc<RefCell<Env>>) -> Self {
        let mut func = Self::new(Rc::clone(&outer_env));
        func.args = vec![LispType::Nil, LispType::Nil]; // Placeholders for length requirements
        func.operator_func = Some(operator_func);
        func
    }

    pub fn call(&mut self, args: &[LispType], env: Rc<RefCell<Env>>) -> LispResult<LispType> {

        if self.args.len() != args.len() {
            return Err(format!("Expected {} arguments to function, found {}", self.args.len(), args.len()));
        }

        if let Some(func) = self.operator_func {
            return func(args.first().unwrap().evaluate(Rc::clone(&env))?, args.last().unwrap().evaluate(Rc::clone(&env))?);
        }

        let mut env_cpy = self.internal_env.borrow().clone();

        for i in 0..args.len() {
            if let LispType::Symbol(s) = self.args.get(i).unwrap() {
                env_cpy.set(s.as_str(), args.get(i).unwrap().evaluate(Rc::clone(&env))?);
            } else {
                return Err(String::from("Expected symbols as arguments in a function"));
            }
        }
        
        return self.body.evaluate(Rc::new(RefCell::new(env_cpy)));

    }

}
