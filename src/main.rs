#![allow(clippy::needless_return)] 

mod result;
mod operator;
mod func;
mod lisp_type;
mod env;
mod reader;

use std::cell::RefCell;
use std::process::exit;
use std::{env as stdenv, fs};
use std::rc::Rc;

use reader::Reader; 
use env::Env;

use rustyline::error::ReadlineError;
use rustyline::DefaultEditor;
use colored::Colorize;

fn jit() {

    let mut rl = DefaultEditor::new().expect("Failed to load input / output");

    let global_env = Rc::new(RefCell::new(Env::new(None)));
    operator::init_operator_funcs(Rc::clone(&global_env));

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
        
        #[cfg(debug_assertions)]
        {
            print!("{} \n{:#?}\n", "Parsed tree: ".green().bold(), form);
        }

        let eval = match form.evaluate(Rc::clone(&global_env)) {
            Ok(v) => v,
            Err(msg) => { println!("{} {}", "Evaluation error:".red().bold(), msg.as_str().red()); continue },
        };

        print!("{} ", "Evaluation:".green().bold());
        eval.println();
    
        #[cfg(debug_assertions)]
        {
            print!("{} \n{:#?}\n", "Evaluated tree: ".green().bold(), eval);
        }

    }


}

fn runtime(input: String) {
    
    let global_env = Rc::new(RefCell::new(Env::new(None)));
    operator::init_operator_funcs(Rc::clone(&global_env));

    let mut reader = Reader::tokenize(input.trim_end().to_owned());
        
    if reader.tokens.is_empty() {
        exit(0);
    }
    
    let form = match reader.read_form() {
        Ok(v) => v,
        Err(msg) => { println!("{} {}", "Reading error:".red().bold(), msg.as_str().red()); exit(1); },
    };

    #[cfg(debug_assertions)]
    {
        print!("{} \n{:#?}\n", "Parsed tree: ".green().bold(), form);
    }

    let _eval = match form.evaluate(Rc::clone(&global_env)) {
        Ok(v) => v,
        Err(msg) => { println!("{} {}", "Evaluation error:".red().bold(), msg.as_str().red()); exit(1); },
    };

    #[cfg(debug_assertions)]
    {
        print!("{} \n{:#?}\n", "Evaluated tree: ".green().bold(), _eval);
    }
    
    exit(0);

}

fn main() {

    let args: Vec<String> = stdenv::args().collect();

    if args.len() ==  1 {
        jit();
        exit(0);
    } else if args.len() == 2 {

        let input = match fs::read_to_string(args.get(1).unwrap()) {
            Ok(raw) => match raw.parse::<String>() {
                Ok(s) => s,
                Err(_) => { println!("Failed to parse filename"); exit(1); }
            }
            Err(_) => { println!("Failed to read file"); exit(1); }
        };

        runtime(input);

        exit(0);

    }

    println!("Invalid number of arguments passed to interpreter"); 

}
