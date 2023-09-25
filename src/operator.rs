use std::cell::RefCell;
use std::rc::Rc;

use crate::env::Env;
use crate::lisp_type::LispType;
use crate::func::LispFunc;
use crate::result::LispResult;

pub fn init_operator_funcs(env: Rc<RefCell<Env>>) {
    env.borrow_mut().set("+", LispType::Func(LispFunc::new_operator(add_symbol, Rc::clone(&env))));
    env.borrow_mut().set("-", LispType::Func(LispFunc::new_operator(sub_symbol, Rc::clone(&env))));
    env.borrow_mut().set("*", LispType::Func(LispFunc::new_operator(mul_symbol, Rc::clone(&env))));
    env.borrow_mut().set("/", LispType::Func(LispFunc::new_operator(div_symbol, Rc::clone(&env))));
    env.borrow_mut().set("=", LispType::Func(LispFunc::new_operator(eq_symbol, Rc::clone(&env))));
}

fn add_symbol(a_in: LispType, b_in: LispType) -> LispResult<LispType> {

    let a = if let LispType::Int(i) = a_in {
        i
    } else {
        return Err(String::from("Attempted to add value that is not numeric"));
    };

    let b = if let LispType::Int(i) = b_in {
        i
    } else {
        return Err(String::from("Attempted to add value that is not numeric"));
    };

    return Ok(LispType::Int(a.wrapping_add(b)));
 
}


fn sub_symbol(a_in: LispType, b_in: LispType) -> LispResult<LispType> {

    let a = if let LispType::Int(i) = a_in {
        i
    } else {
        return Err(String::from("Attempted to subtract value that is not numeric"));
    };

    let b = if let LispType::Int(i) = b_in {
        i
    } else {
        return Err(String::from("Attempted to subtract value that is not numeric"));
    };

    return Ok(LispType::Int(a.wrapping_sub(b)));
 
}


fn mul_symbol(a_in: LispType, b_in: LispType) -> LispResult<LispType> {

    let a = if let LispType::Int(i) = a_in {
        i
    } else {
        return Err(String::from("Attempted to multiply value that is not numeric"));
    };

    let b = if let LispType::Int(i) = b_in {
        i
    } else {
        return Err(String::from("Attempted to multiply value that is not numeric"));
    };

    return Ok(LispType::Int(a.wrapping_mul(b)));
 
}


fn div_symbol(a_in: LispType, b_in: LispType) -> LispResult<LispType> {

    let a = if let LispType::Int(i) = a_in {
        i
    } else {
        return Err(String::from("Attempted to divide value that is not numeric"));
    };

    let b = if let LispType::Int(i) = b_in {
        i
    } else {
        return Err(String::from("Attempted to divide value that is not numeric"));
    };

    return Ok(LispType::Int(a.wrapping_div(b)));
 
}

fn eq_symbol(a_in: LispType, b_in: LispType) -> LispResult<LispType> {

    if std::mem::discriminant(&a_in) != std::mem::discriminant(&b_in) {
        return Err(String::from("Attempted to compare two values of different types"));
    }

    let comp = match (a_in, b_in) {
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
