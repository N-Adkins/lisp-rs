mod token;
mod symbol;
mod ast;
mod lexer;
mod parser;

use lexer::Lexer;
use parser::Parser;

fn main() {
    
    let test_input = include_str!("../test.lisp");

    let mut lexer = Lexer::new(String::from(test_input));
    lexer.tokenize();

    let mut parser = Parser::new(lexer.get_tokens());
    parser.parse();

    println!("{}", parser.get_list().eval_list());

}
