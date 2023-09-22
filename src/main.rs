mod result;
mod symbol;
mod lisp_type;
mod reader;

use reader::Reader; 

use rustyline::error::ReadlineError;
use rustyline::DefaultEditor;

use colored::Colorize;

fn main() {
    
    let mut rl = DefaultEditor::new().expect("Failed to load input / output");

    loop {

        let read_line = rl.readline("Lisp> ".bold().to_string().as_str());
        let input = match read_line {
            Ok(line) => {
                rl.add_history_entry(line.as_str()).expect("");
                line.as_str().to_owned()
            }
            Err(ReadlineError::Interrupted) | Err(ReadlineError::Eof)=> { println!("{}", "Exiting".bold()); break },
            Err(_) => panic!("Failed to read input line"),
        };

        if input.trim_end().is_empty() {
            continue;
        }
        
        let mut reader = Reader::tokenize(input.trim_end().to_owned());
        
        if reader.tokens.is_empty() {
            continue;
        }
        
        let form = match reader.read_form() {
            Ok(v) => v,
            Err(msg) => { println!("{} {}", "Reading error:".red().bold(), msg.as_str().red()); continue },
        };
        
        print!("{} ", "Parsed form:".green().bold());
        form.println();
        
        let eval = match form.evaluate() {
            Ok(v) => v,
            Err(msg) => { println!("{} {}", "Evaluation error:".red().bold(), msg.as_str().red()); continue },
        };

        println!("{} {}", "Evaluation:".green().bold(), eval);

    }
}
