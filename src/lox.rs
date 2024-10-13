use std::io;
use std::env;
use std::fs::read_to_string;

mod token;
use token::Token;

mod scanner;
use crate::lox::scanner::Scanner;

pub struct Lox {
    had_error: bool,
    file_name: Option<String>,
}


impl Lox{

    pub fn new() -> Self {
        let args: Vec<String> = env::args().collect();
        // Self::display_args(&args);
        if args.len() > 2 {
            panic!("Usage: rslox [script]");
        } else {
            if let Some(file_name) = args.get(1) {
                Self {
                    had_error: false,
                    file_name: Some(file_name.clone()),
                }
            } else {
                Self {
                    had_error: false,
                    file_name: None,
                }
            }
        }

    }

    fn display_args(args: &Vec<String>) {
        for arg in args {
            println!("{arg}");
        }
    }

    pub fn start(&mut self) {
        if let Some(file_name) = &self.file_name {
            self.run_file(&file_name);
        } else {
            self.run_prompt();
        }

    }

    pub fn run_file(&self, file_name: &str) {
        println!("Running file {file_name}");
        let result = read_to_string(file_name).unwrap();
        Self::run(result);
        if self.had_error {
            panic!("Exited on error.");
        }
    }

    pub fn run_prompt(&mut self) {
        let mut input = String::new();
        println!("Welcome to rsLox");
        loop {  
            println!(">");
            let _num_bytes = io::stdin().read_line(&mut input).unwrap();
            if input.is_empty() {
                println!("Quitting rsLox. Bye!");
                break;
            } else {
                Self::run(input.clone());
                self.had_error = false;
            }

            input.clear();  // stdin::readline "appends" to the buffer passed
        }
    }

    fn run(input: String) {
        println!("run received: {input}");
        let mut scanner = Scanner::new(input);
        let tokens: Vec<Token> = scanner.scan_tokens();

        println!("tokens:");
        for token in tokens {
            println!("{token}");
        }
    }

    fn error(&mut self, line: u32, msg: String) {
        self.report(line, "".to_string(), msg);
    }

    fn report(&mut self, line: u32, whr: String, msg: String) {
        println!("[line {line}] Error {whr} : {msg}");
        self.had_error = true;
    }
}
