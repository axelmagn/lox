use std::sync::Mutex;
use std::{
    env, fs,
    io::{self, Write},
    process,
};

use crate::ast_printer::AstPrinter;
use crate::parser::Parser;
use crate::scanner::Scanner;
use crate::token::{Token, TokenType};

static mut HAD_ERROR: Mutex<bool> = Mutex::new(false);

pub struct Lox;

impl Lox {
    pub fn run_cli() {
        let mut args = env::args();
        if args.len() > 2 {
            println!("usage: rlox [script]");
            process::exit(64);
        } else if args.len() == 2 {
            let _program = args.next().unwrap();
            let path = args.next().unwrap();
            println!("RUNNING LOX FILE: {}", path);
            Self::run_file(&path);
        } else {
            println!("RUNNING LOX INTERPRETER");
            Self::run_prompt();
        }
    }

    fn run_file(path: &str) {
        let content = fs::read_to_string(path).unwrap();
        Self::run(&content);
        if Self::had_error() {
            process::exit(65);
        }
    }

    fn run_prompt() {
        let mut line = String::new();
        loop {
            print!("> ");
            io::stdout().flush().unwrap();
            line.clear();
            let n_bytes = io::stdin().read_line(&mut line).unwrap();
            if n_bytes == 0 {
                break;
            }
            Self::run(&line);
            Self::set_had_error(false);
        }
    }

    fn run(source: &str) {
        let mut scanner = Scanner::new(source);
        let tokens = scanner.scan_tokens();
        let mut parser = Parser::new(&tokens);
        let expression = parser.parse();
        if Self::had_error() {
            return;
        }
        println!("{}", AstPrinter::new().print(&expression.unwrap()));
    }

    pub fn error_on_line(line: usize, message: &str) {
        Self::report(line, "", message);
    }

    pub fn error_on_token(token: &Token, message: &str) {
        if token.ttype == TokenType::EOF {
            Self::report(token.line, "at end", message);
        } else {
            Self::report(token.line, &format!("at '{}'", token.lexeme), message);
        }
    }

    fn report(line: usize, loc: &str, message: &str) {
        write!(io::stderr(), "[line {}] Error{}: {}", line, loc, message).unwrap();
        Self::set_had_error(true);
    }

    fn had_error() -> bool {
        unsafe { *HAD_ERROR.lock().unwrap() }
    }

    fn set_had_error(val: bool) {
        unsafe {
            *HAD_ERROR.lock().unwrap() = val;
        }
    }
}
