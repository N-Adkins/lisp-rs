use std::cell::RefCell;
use std::rc::Rc;

use crate::env::Env;
use crate::lisp_type::LispType;
use crate::func::LispFunc;
use crate::result::LispResult;

macro_rules! decl_operator {
    ( $symbol:expr, $func:expr, $env:expr ) => {
         $env.borrow_mut().set($symbol, LispType::Func(LispFunc::new_operator($func, Rc::clone(&$env))));
    }
}

pub fn init_operator_funcs(env: Rc<RefCell<Env>>) {
    decl_operator!("+", add_symbol, env);
    decl_operator!("-", sub_symbol, env);
    decl_operator!("*", mul_symbol, env);
    decl_operator!("/", div_symbol, env);
    decl_operator!("=", eq_symbol, env);
    decl_operator!("<", lt_symbol, env);
    decl_operator!("<=", lte_symbol, env);
    decl_operator!(">", gt_symbol, env);
    decl_operator!(">=", gte_symbol, env);
}

fn add_symbol(a_in: LispType, b_in: LispType) -> LispResult<LispType> {

    match (a_in, b_in) {
        (LispType::Int(a), LispType::Float(b)) => Ok(LispType::Float((a as f32) + b)),
        (LispType::Float(a), LispType::Int(b)) => Ok(LispType::Float(a + (b as f32))),
        (LispType::Int(a), LispType::Int(b)) => Ok(LispType::Int(a.wrapping_add(b))),
        (LispType::Float(a), LispType::Float(b)) => Ok(LispType::Float(a + b)),
        _ => Err(String::from("Invalid operands to add operation")),
    }
 
}


fn sub_symbol(a_in: LispType, b_in: LispType) -> LispResult<LispType> {

    match (a_in, b_in) {
        (LispType::Int(a), LispType::Float(b)) => Ok(LispType::Float((a as f32) - b)),
        (LispType::Float(a), LispType::Int(b)) => Ok(LispType::Float(a - (b as f32))),
        (LispType::Int(a), LispType::Int(b)) => Ok(LispType::Int(a.wrapping_sub(b))),
        (LispType::Float(a), LispType::Float(b)) => Ok(LispType::Float(a - b)),
        _ => Err(String::from("Invalid operands to sub operation")),
    }
 
}


fn mul_symbol(a_in: LispType, b_in: LispType) -> LispResult<LispType> {

    match (a_in, b_in) {
        (LispType::Int(a), LispType::Float(b)) => Ok(LispType::Float((a as f32) * b)),
        (LispType::Float(a), LispType::Int(b)) => Ok(LispType::Float(a * (b as f32))),
        (LispType::Int(a), LispType::Int(b)) => Ok(LispType::Int(a.wrapping_mul(b))),
        (LispType::Float(a), LispType::Float(b)) => Ok(LispType::Float(a * b)),
        _ => Err(String::from("Invalid operands to mul operation")),
    }
 
}


fn div_symbol(a_in: LispType, b_in: LispType) -> LispResult<LispType> {

    match (a_in, b_in) {
        (LispType::Int(a), LispType::Float(b)) => Ok(LispType::Float((a as f32) / b)),
        (LispType::Float(a), LispType::Int(b)) => Ok(LispType::Float(a / (b as f32))),
        (LispType::Int(a), LispType::Int(b)) => Ok(LispType::Int(a.wrapping_div(b))),
        (LispType::Float(a), LispType::Float(b)) => Ok(LispType::Float(a / b)),
        _ => Err(String::from("Invalid operands to div operation")),
    }
 
}

fn lt_symbol(a_in: LispType, b_in: LispType) -> LispResult<LispType> {

    if std::mem::discriminant(&a_in) != std::mem::discriminant(&b_in) {
        return Err(String::from("Attempted to compare two values of different types"));
    }

    let comp = match (a_in, b_in) {
        (LispType::Int(a), LispType::Float(b)) => (a as f32) < b,
        (LispType::Float(a), LispType::Int(b)) => a < (b as f32),
        (LispType::Float(a), LispType::Float(b)) => a < b,
        (LispType::Int(a), LispType::Int(b)) => a < b,
        (LispType::String(a), LispType::String(b)) => a.len() < b.len(),
        (LispType::Symbol(a), LispType::Symbol(b)) => a.len() < b.len(),
        (LispType::Bool(a), LispType::Bool(b)) => a < b,
        (LispType::Nil, LispType::Nil) => return Err(String::from("Can't compare nil values")),
        (LispType::List(_), LispType::List(_)) => return Err(String::from("Can't compare lists")),
        (LispType::Func(_), LispType::Func(_)) => return Err(String::from("Can't compare functions")),
        (_, _) => return Err(String::from("Failed to compare both elements in lt statement")),
    };

    Ok(LispType::Bool(comp))

}


fn lte_symbol(a_in: LispType, b_in: LispType) -> LispResult<LispType> {

    if std::mem::discriminant(&a_in) != std::mem::discriminant(&b_in) {
        return Err(String::from("Attempted to compare two values of different types"));
    }

    let comp = match (a_in, b_in) {
        (LispType::Int(a), LispType::Float(b)) => a as f32 <= b,
        (LispType::Float(a), LispType::Int(b)) => a <= b as f32,
        (LispType::Float(a), LispType::Float(b)) => a <= b,
        (LispType::Int(a), LispType::Int(b)) => a <= b,
        (LispType::String(a), LispType::String(b)) => a.len() <= b.len(),
        (LispType::Symbol(a), LispType::Symbol(b)) => a.len() <= b.len(),
        (LispType::Bool(a), LispType::Bool(b)) => a <= b,
        (LispType::Nil, LispType::Nil) => return Err(String::from("Can't compare nil values")),
        (LispType::List(_), LispType::List(_)) => return Err(String::from("Can't compare lists")),
        (LispType::Func(_), LispType::Func(_)) => return Err(String::from("Can't compare functions")),
        (_, _) => return Err(String::from("Failed to compare both elements in lte statement")),
    };

    Ok(LispType::Bool(comp))

}

fn gt_symbol(a_in: LispType, b_in: LispType) -> LispResult<LispType> {

    if std::mem::discriminant(&a_in) != std::mem::discriminant(&b_in) {
        return Err(String::from("Attempted to compare two values of different types"));
    }

    let comp = match (a_in, b_in) { 
        (LispType::Int(a), LispType::Float(b)) => a as f32 > b,
        (LispType::Float(a), LispType::Int(b)) => a > b as f32,
        (LispType::Float(a), LispType::Float(b)) => a > b,
        (LispType::Int(a), LispType::Int(b)) => a > b,
        (LispType::String(a), LispType::String(b)) => a.len() > b.len(),
        (LispType::Symbol(a), LispType::Symbol(b)) => a.len() > b.len(),
        (LispType::Bool(a), LispType::Bool(b)) => a > b,
        (LispType::Nil, LispType::Nil) => return Err(String::from("Can't compare nil values")),
        (LispType::List(_), LispType::List(_)) => return Err(String::from("Can't compare lists")),
        (LispType::Func(_), LispType::Func(_)) => return Err(String::from("Can't compare functions")),
        (_, _) => return Err(String::from("Failed to compare both elements in gt statement")),
    };

    Ok(LispType::Bool(comp))

}

fn gte_symbol(a_in: LispType, b_in: LispType) -> LispResult<LispType> {

    if std::mem::discriminant(&a_in) != std::mem::discriminant(&b_in) {
        return Err(String::from("Attempted to compare two values of different types"));
    }

    let comp = match (a_in, b_in) { 
        (LispType::Int(a), LispType::Float(b)) => a as f32 >= b,
        (LispType::Float(a), LispType::Int(b)) => a >= b as f32,
        (LispType::Float(a), LispType::Float(b)) => a >= b,
        (LispType::Int(a), LispType::Int(b)) => a >= b,
        (LispType::String(a), LispType::String(b)) => a.len() >= b.len(),
        (LispType::Symbol(a), LispType::Symbol(b)) => a.len() >= b.len(),
        (LispType::Bool(a), LispType::Bool(b)) => a >= b,
        (LispType::Nil, LispType::Nil) => return Err(String::from("Can't compare nil values")),
        (LispType::List(_), LispType::List(_)) => return Err(String::from("Can't compare lists")),
        (LispType::Func(_), LispType::Func(_)) => return Err(String::from("Can't compare functions")),
        (_, _) => return Err(String::from("Failed to compare both elements in gte statement")),
    };

    Ok(LispType::Bool(comp))

}

fn eq_symbol(a_in: LispType, b_in: LispType) -> LispResult<LispType> {

    if std::mem::discriminant(&a_in) != std::mem::discriminant(&b_in) {
        return Err(String::from("Attempted to compare two values of different types"));
    }

    let comp = match (a_in, b_in) {
        (LispType::Int(a), LispType::Float(b)) => a as f32 == b,
        (LispType::Float(a), LispType::Int(b)) => a == b as f32,
        (LispType::Float(a), LispType::Float(b)) => a == b,
        (LispType::Int(a), LispType::Int(b)) => a == b,
        (LispType::String(a), LispType::String(b)) => a == b,
        (LispType::Bool(a), LispType::Bool(b)) => a == b,
        (LispType::Nil, LispType::Nil) => true,
        (LispType::Symbol(a), LispType::Symbol(b)) => a == b,
        (LispType::List(_), LispType::List(_)) => return Err(String::from("Can't compare lists")),
        (LispType::Func(_), LispType::Func(_)) => return Err(String::from("Can't compare functions")),
        (_, _) => return Err(String::from("Failed to compare both elements in eq statement")),
    };

    Ok(LispType::Bool(comp))

}
