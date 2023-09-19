#[derive(Clone)]
pub enum Keyword {
    WriteLine,
}

#[derive(Clone)]
pub enum Token {

    Symbol(char),
    String(String),
    Keyword(Keyword),
    Int(i32),

    // Value is the paren depth
    ListBegin(u32),
    ListEnd(u32),

}

impl ToString for Token {
    
    fn to_string(&self) -> String {
        match *self {
            Token::Symbol(_) => String::from("Symbol"),
            Token::Keyword(_) => String::from("Keyword"),
            Token::String(_) => String::from("String"),
            Token::Int(_) => String::from("Int"),
            Token::ListBegin(_) => String::from("ListBegin"),
            Token::ListEnd(_) => String::from("ListEnd"),
        }
    }

}
