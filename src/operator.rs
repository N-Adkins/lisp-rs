use crate::env::Env;
use crate::lisp_type::LispType;
use crate::func::LispFunc;
use crate::result::LispResult;

pub fn init_symbol_funcs(env: &mut Env) {
    env.set("+", LispType::Func(LispFunc { func: add_symbol, arg_count: 2 }));
    env.set("-", LispType::Func(LispFunc { func: sub_symbol, arg_count: 2 }));
    env.set("*", LispType::Func(LispFunc { func: mul_symbol, arg_count: 2 }));
    env.set("/", LispType::Func(LispFunc { func: div_symbol, arg_count: 2 }));
}

fn add_symbol(args: &[LispType], env: &mut Env) -> LispResult<LispType> {
    
    let a = if let LispType::Int(i) = args.iter().nth(0).unwrap_or(&LispType::Int(0)).evaluate(env)? {
        i
    } else {
        return Err(String::from("Attempted to add value that is not numeric"));
    };

    let b = if let LispType::Int(i) = args.iter().nth(1).unwrap_or(&LispType::Int(0)).evaluate(env)? {
        i
    } else {
        return Err(String::from("Attempted to add value that is not numeric"));
    };

    return Ok(LispType::Int(a.wrapping_add(b)));
    
}

fn sub_symbol(args: &[LispType], env: &mut Env) -> LispResult<LispType> {
    
    let a = if let LispType::Int(i) = args.iter().nth(0).unwrap_or(&LispType::Int(0)).evaluate(env)? {
        i
    } else {
        return Err(String::from("Attempted to subtract value that is not numeric"));
    };

    let b = if let LispType::Int(i) = args.iter().nth(1).unwrap_or(&LispType::Int(0)).evaluate(env)? {
        i
    } else {
        return Err(String::from("Attempted to subtract value that is not numeric"));
    };

    return Ok(LispType::Int(a.wrapping_sub(b)));
    
}

fn mul_symbol(args: &[LispType], env: &mut Env) -> LispResult<LispType> {
    
    let a = if let LispType::Int(i) = args.iter().nth(0).unwrap_or(&LispType::Int(0)).evaluate(env)? {
        i
    } else {
        return Err(String::from("Attempted to multiply value that is not numeric"));
    };

    let b = if let LispType::Int(i) = args.iter().nth(1).unwrap_or(&LispType::Int(0)).evaluate(env)? {
        i
    } else {
        return Err(String::from("Attempted to multiply value that is not numeric"));
    };

    return Ok(LispType::Int(a.wrapping_mul(b)));
    
}

fn div_symbol(args: &[LispType], env: &mut Env) -> LispResult<LispType> {
    
    let a = if let LispType::Int(i) = args.iter().nth(0).unwrap_or(&LispType::Int(0)).evaluate(env)? {
        i
    } else {
        return Err(String::from("Attempted to divide value that is not numeric"));
    };

    let b = if let LispType::Int(i) = args.iter().nth(1).unwrap_or(&LispType::Int(0)).evaluate(env)? {
        i
    } else {
        return Err(String::from("Attempted to divide value that is not numeric"));
    };

    return Ok(LispType::Int(a.wrapping_div(b)));
    
}
