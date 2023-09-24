use crate::env::Env;
use crate::lisp_type::LispType;
use crate::result::LispResult;

#[derive(Clone)]
pub struct LispFunc {
    pub func: fn(&[LispType], &mut Env) -> LispResult<LispType>,
    pub arg_count: usize,
}

impl LispFunc {

    pub fn call(&self, args: &[LispType], env: &mut Env) -> LispResult<LispType> {
        if args.len() != self.arg_count {
            return Err(format!("Expected {} arguments for function, was called with {}", self.arg_count, args.len()));
        }
        return (self.func)(args, env);
    }

}
