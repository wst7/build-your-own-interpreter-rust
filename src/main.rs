use std::env;
use std::fs;
use std::io::{self, Write};

mod parser;
mod scanner;
mod evaluator;


fn read_file_contents(filename: &str) -> String {
    fs::read_to_string(filename).unwrap_or_else(|_| {
        writeln!(io::stderr(), "Failed to read file {}", filename).unwrap();
        String::new()
    })
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        writeln!(io::stderr(), "Usage: {} tokenize <filename>", args[0]).unwrap();
        return;
    }

    let command = &args[1];
    let filename = &args[2];

    match command.as_str() {
        "tokenize" => {
            let file_contents = read_file_contents(&filename);

            if !file_contents.is_empty() {
                let mut s = scanner::Scanner::new(&file_contents);
                let (tokens, errors) = s.scan_tokens();
                for err in errors {
                    eprintln!("{}", err);
                }
                for token in tokens {
                    println!("{}", token.to_string());
                }
                if !errors.is_empty() {
                    std::process::exit(65);
                }
            } else {
                println!("EOF  null");
            }
        }
        "parse" => {
            let file_contents = read_file_contents(&filename);
            let mut s = scanner::Scanner::new(&file_contents);
            let (tokens, errors) = s.scan_tokens();
            if !errors.is_empty() {
                std::process::exit(65);
            }
            let mut parser = parser::Parser::new(tokens);
            let expr = match parser.parse() {
                Ok(expr) => expr,
                Err(error) => {
                    std::process::exit(65);
                }
            };
            println!("{}", expr);
        }
        "evaluate" => {
            let file_contents = read_file_contents(&filename);
            let mut s = scanner::Scanner::new(&file_contents);
            let (tokens, errors) = s.scan_tokens();
            let mut parser = parser::Parser::new(tokens);
            let ast = match parser.parse() {
                Ok(expr) => expr,
                Err(error) => {
                    std::process::exit(65);
                }
            };
            let evaluator = evaluator::Evaluator::new(&ast);
            let result = evaluator.evaluate();
            println!("{}", result);
        }
        _ => {
            writeln!(io::stderr(), "Unknown command: {}", command).unwrap();
            return;
        }
    }
}
