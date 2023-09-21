pub trait Symbol {
    fn get_symbol_func(&self) -> Option<fn(i32, i32) -> i32>;
}

impl Symbol for String {

    fn get_symbol_func(&self) -> Option<fn(i32, i32) -> i32> {
        
        match self.as_str() {
            "+" => Some(add_symbol),
            "-" => Some(sub_symbol),
            "*" => Some(mul_symbol),
            "/" => Some(div_symbol),
            _ => None,
        }

    }

}

fn add_symbol(a: i32, b: i32) -> i32 {
    a.wrapping_add(b)
}

fn sub_symbol(a: i32, b: i32) -> i32 {
    a.wrapping_sub(b)
}

fn mul_symbol(a: i32, b: i32) -> i32 {
    a.wrapping_mul(b)
}

fn div_symbol(a: i32, b: i32) -> i32 {
    a.wrapping_div(b)
}
