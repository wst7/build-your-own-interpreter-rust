use std::env;
use std::fs;
use std::io::{self, Write};

mod evaluator;
mod parser;
mod scanner;
mod environment;

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
            let expr = match parser.parse_expr() {
                Ok(expr) => expr,
                Err(error) => {
                    eprintln!("{}", error);
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
            let ast = match parser.parse_expr() {
                Ok(expr) => expr,
                Err(error) => {
                    eprintln!("{}", error);
                    std::process::exit(65);
                }
            };
            let value = match evaluator::evaluate_expr(&ast){
                Ok(result) => result,
                Err(error) => {
                    eprintln!("{}", error);
                    std::process::exit(70);
                }
            };
            println!("{}", value);
        },
        "run" => {
            let file_contents = read_file_contents(&filename);
            let mut s = scanner::Scanner::new(&file_contents);
            let (tokens, errors) = s.scan_tokens();
           
            let mut parser = parser::Parser::new(tokens);
            let stmts = match parser.parse() {
                Ok(expr) => expr,
                Err(error) => {
                    eprintln!("{}", error);
                    std::process::exit(65);
                }
            };
            for stmt in stmts {
                let _ = match evaluator::evaluate_stmt(&stmt) {
                    Ok(_) => {}
                    Err(err) => {
                        eprintln!("{}", err);
                        std::process::exit(70);
                    }
                };
            }
        }
        _ => {
            writeln!(io::stderr(), "Unknown command: {}", command).unwrap();
            return;
        }
    }
}
