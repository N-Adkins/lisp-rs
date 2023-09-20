use crate::token::Keyword;
use crate::symbol::get_symbol_func;

#[derive(Clone)]
pub enum AST {
    List(Vec<AST>),
    Int(i32),
    Symbol(char),
    Keyword(Keyword),
    String(String),
}

fn actual_to_string(value: &AST, str: &mut String) {
    match value {

        AST::Int(v) => str.push_str(&*(v.to_string())),
        AST::Symbol(v) => str.push(*v),

        AST::String(v) => {
            str.push('\"');
            str.push_str(&*v);
            str.push('\"');
        }

        AST::Keyword(v) => match *v {
            Keyword::WriteLine => str.push_str("write-line"),
        }

        AST::List(list) => {
            str.push('(');
            for nested_val in list {
                actual_to_string(nested_val, str);
            }
            str.push(')');
        }
    }
    str.push(' '); 
}

impl AST {

    pub fn to_string(&self) -> String {
        let mut str: String = String::from("");
        actual_to_string(&self.to_owned(), &mut str);
        str
    }

    pub fn eval_list(&self) -> i32 {
        
        let mut children = match self {
            AST::List(children) => children.clone(),
            _ => panic!("Attempted to evaulate non-list as a list"),
        };
        
        let symbol = match children.remove(0) {
            AST::Symbol(v) => v,
            _ => panic!("Expected symbol for evaluation"),
        };

        let symbol_func = match get_symbol_func(symbol) {
            Some(func) => func,
            None => panic!("Unexpected symbol, found no associated function"),
        };
        
        let a_child = children.remove(0);
        let a: i32 = match a_child {
            AST::List(_) => a_child.eval_list(),
            AST::Int(v) => v,
            _ => panic!("Unexpected symbol, expected list or int, found {}", a_child.to_string()),
        };

        let b_child = children.remove(0);
        let b: i32 = match b_child {
            AST::List(_) => b_child.eval_list(), 
            AST::Int(v) => v,
            _ => panic!("Unexpected symbol, expected list or int, found {}", b_child.to_string()),
        };

        symbol_func(a, b)

    }

}
