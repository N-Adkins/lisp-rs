mod reader;

use reader::{Reader, Print};

fn main() {
    
    let mut reader = Reader::tokenize(String::from(include_str!("../test.lisp")));

    //for token in reader.tokens {
        //print!("{}", token.to_string());
    //}
    
    reader.read_form().println();

}
