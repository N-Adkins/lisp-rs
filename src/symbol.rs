type SymbolImpl = fn(i32, i32) -> i32;

fn symbol_add(a: i32, b: i32) -> i32 {
    a.wrapping_add(b)
}

fn symbol_subtract(a: i32, b: i32) -> i32 {
    a.wrapping_sub(b)
}

fn symbol_multiply(a: i32, b: i32) -> i32 {
    a.wrapping_mul(b)
}

fn symbol_divide(a: i32, b: i32) -> i32 {
    a.wrapping_div(b)
}

pub fn get_symbol_func(symbol: char) -> Option<SymbolImpl> {
    match symbol {
        '+' => Some(symbol_add),
        '-' => Some(symbol_subtract),
        '*' => Some(symbol_multiply),
        '/' => Some(symbol_divide),
        _ => None,
    }
}
