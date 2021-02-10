mod lexer;
mod parser;

use lexer::lexer::Lexer;

use parser::parser::Parser;
use parser::node::Node;

use rustyline::error::ReadlineError;
use rustyline::Editor;

use std::collections::HashMap;

fn main() {
    let mut readline = Editor::<()>::new();
    let mut env: HashMap<String, Node> = HashMap::new();
    loop {
        let input = match readline.readline("\x1b[34m>\x1b[0m ") {
            Err(ReadlineError::Interrupted) => continue,
            Err(ReadlineError::Eof) => break,
            Err(error) => {
                println!("{:?}", error);
                break;
            },
            Ok(string) => string
        };
        let line = input.trim();
        if line.len() == 0 {
            continue;
        }
        let lexer = Lexer::new(line);
        let tokens: Result<Vec<_>, _> = lexer.collect();
        let tokens = match tokens {
            Err(message) => {
                println!("\x1b[31m{}\x1b[0m", message);
                continue;
            },
            Ok(tokens) => tokens
        };
        let mut parser = Parser::new(tokens);
        let node = parser.parse_all();
        match node {
            Ok(result) => {
                match parser.status() {
                    Ok(_) => println!("{}", result.eval(&mut env).to_string()),
                    Err(message) => {
                        println!("\x1b[31m{}\x1b[0m", message);
                        continue
                    }
                };
            },
            Err(message) => println!("\x1b[31m{}\x1b[0m", message),
        };
    }
}
