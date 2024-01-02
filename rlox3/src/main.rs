use clap::Parser;

mod ast;
mod lox;
mod parser;

#[derive(Parser)]
#[command(author, version, about)]
struct Args {
    /// optional script file to run
    file: Option<String>,
}

fn main() {
    let args = Args::parse();
}
