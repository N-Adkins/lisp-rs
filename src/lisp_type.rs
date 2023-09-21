use crate::symbol::Symbol;
use crate::result::LispResult;

#[derive(Clone)]
pub enum LispType {
    List(Vec<LispType>),
    Int(i32),
    Symbol(String),
    String(String),
    Bool(bool),
    Nil,
}

impl LispType {

    pub fn evaluate(&self) -> LispResult<i32> {

        match self {

            LispType::List(vec) => {

                if vec.is_empty() {
                    return Ok(0);
                }

                let mut eval_vec = vec.clone();
                
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

                let a = eval_vec.remove(0).evaluate()?;
                let b = eval_vec.remove(0).evaluate()?;

                return Ok(eval_func(a, b));

            }

            LispType::Int(i) => return Ok(*i),

            _ => Err(String::from("Expected integer when evaluating")),

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
            LispType::Symbol(s) | LispType::String(s) => print!("{}", s),
            LispType::Bool(b) => match *b {
                true => print!("true"),
                false => print!("false"),
            }
            LispType::Nil => print!("nil"),
        }   
    }

    pub fn println(&self) {
        self.print();
        println!();
    }

}
