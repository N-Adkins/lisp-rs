mod result;
mod symbol;
mod lisp_type;
mod reader;

use reader::Reader; 

use rustyline::error::ReadlineError;
use rustyline::DefaultEditor;

fn main() {

    let mut rl = DefaultEditor::new().expect("Failed to load input / output");

    loop {

        let read_line = rl.readline("Lisp> ");
        let input = match read_line {
            Ok(line) => {
                rl.add_history_entry(line.as_str()).expect("");
                line.as_str().to_owned()
            }
            Err(ReadlineError::Interrupted) => { println!("Exiting"); break },
            Err(_) => panic!("Failed to read input line"),
        };

        let mut reader = Reader::tokenize(input.trim_end().to_owned());
        
        if reader.tokens.is_empty() {
            continue;
        }
        
        let form = match reader.read_form() {
            Ok(v) => v,
            Err(msg) => { println!("{}", msg); continue },
        };

        form.println();
        
        let eval = match form.evaluate() {
            Ok(v) => v,
            Err(msg) => { println!("{}", msg); continue },
        };

        println!("{}", eval);

    }
}
