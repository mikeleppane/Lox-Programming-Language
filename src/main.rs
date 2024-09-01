use anyhow::Result;
use rlox::Scanner;
use std::{
    env::args,
    io::{stdout, BufRead, Write},
};
fn main() {
    let args: Vec<String> = args().collect();

    match args.len() {
        1 => run_prompt(),
        2 => run_file(&args[1]).expect("Error running file"),
        _ => {
            eprintln!("Usage: lox-ast [script]");
            std::process::exit(64);
        }
    }
}

fn run_file(path: &str) -> Result<()> {
    let buf = std::fs::read_to_string(path)?;
    if run(&buf).is_err() {
        std::process::exit(65);
    }
    Ok(())
}

fn run_prompt() {
    let stdin = std::io::stdin();
    let stdout = stdout();
    println!("Welcome to Lox!");
    println!("Press Ctrl-C to exit.");
    println!("Type in your code below:");
    println!();

    print!("> ");
    stdout.lock().flush().unwrap();
    for line in stdin.lock().lines() {
        if let Ok(line) = line {
            if line.trim() == "exit" {
                break;
            }
            if run(&line).is_err() {
                //  Ignore: error already printed
            }
        } else {
            break;
        }
        print!("> ");
        stdout.lock().flush().unwrap();
    }
}

fn run(source: &str) -> Result<()> {
    let mut scanner = Scanner::new(source);
    let tokens = scanner.scan_tokens()?;
    for token in tokens {
        println!("{token:#?}");
    }
    Ok(())
}
