use anyhow::Result;
use std::{
    env::args,
    io::{BufRead, Write},
};
mod scanner;
mod tokens;

use scanner::Scanner;

fn main() -> Result<()> {
    let args: Vec<String> = args().collect();
    if args.len() > 2 {
        println!("Usage: rlox [script]");
        return Ok(());
    }
    if args.len() == 2 {
        let script = &args[1];
        println!("Running file: {script}");
        run_file(script);
    } else {
        println!("Running prompt");
        run_prompt()?;
    }
    Ok(())
}

fn run_file(script: &str) {
    let source = std::fs::read_to_string(script).expect("Failed to read file");
    run(&source);
}

fn run_prompt() -> Result<()> {
    println!("Welcome to the Lox REPL");
    println!("Type 'exit' to quit");
    loop {
        print!("> ");
        std::io::stdout().flush()?;
        let Some(line) = std::io::stdin().lock().lines().next() else {
            break;
        };
        let line = line?;
        if line == "exit" {
            break;
        }
        let scanner = Scanner::new(&line);

        for token in scanner {
            println!("{token:#?}");
        }
    }
    Ok(())
}

fn run(source: &str) {
    println!("==== Source =====");
    println!("{source}");

    println!("==== Tokens =====");

    let scanner = Scanner::new(source);

    for token in scanner {
        println!("{token:#?}");
    }
}

#[cfg(test)]
mod tests {
    use tokens::Token;

    use super::*;

    #[test]
    fn test_simple_lexer() {
        let source = "> << / >= == <= // comment\n // comment\n, . ! !=\n=+*-\n;\n";
        let scanner = Scanner::new(source);
        let tokens: Vec<Token> = scanner.collect();
        assert_eq!(
            tokens,
            vec![
                Token::new(tokens::TokenType::Greater, ">", 1),
                Token::new(tokens::TokenType::Less, "<", 1),
                Token::new(tokens::TokenType::Less, "<", 1),
                Token::new(tokens::TokenType::Slash, "/", 1),
                Token::new(tokens::TokenType::GreaterEqual, ">=", 1),
                Token::new(tokens::TokenType::EqualEqual, "==", 1),
                Token::new(tokens::TokenType::LessEqual, "<=", 1),
                Token::new(tokens::TokenType::Comma, ",", 3),
                Token::new(tokens::TokenType::Dot, ".", 3),
                Token::new(tokens::TokenType::Bang, "!", 3),
                Token::new(tokens::TokenType::BangEqual, "!=", 3),
                Token::new(tokens::TokenType::Equal, "=", 4),
                Token::new(tokens::TokenType::Plus, "+", 4),
                Token::new(tokens::TokenType::Star, "*", 4),
                Token::new(tokens::TokenType::Minus, "-", 4),
                Token::new(tokens::TokenType::Semicolon, ";", 5),
            ]
        );
    }

    #[test]
    fn test_simple_lexer_with_string() {
        let source = "\n\n\n\n\n\n {}\n ()";
        let scanner = Scanner::new(source);
        let tokens: Vec<Token> = scanner.collect();
        assert_eq!(
            tokens,
            vec![
                Token::new(tokens::TokenType::LeftBrace, "{", 7),
                Token::new(tokens::TokenType::RightBrace, "}", 7),
                Token::new(tokens::TokenType::LeftParen, "(", 8),
                Token::new(tokens::TokenType::RightParen, ")", 8),
            ]
        );
    }

    #[test]
    fn test_string_literal() {
        let source = "\"Hello, World!\"";
        let scanner = Scanner::new(source);
        let tokens: Vec<Token> = scanner.collect();
        assert_eq!(
            tokens,
            vec![Token::new(
                tokens::TokenType::String("Hello, World!"),
                "Hello, World!",
                1
            )]
        );
    }

    #[test]
    fn test_string_literal_with_newline() {
        let source = " > \"Hello, \nWorld!\" < ";
        let scanner = Scanner::new(source);
        let tokens: Vec<Token> = scanner.collect();
        assert_eq!(
            tokens,
            vec![
                Token::new(tokens::TokenType::Greater, ">", 1),
                Token::new(
                    tokens::TokenType::String("Hello, \nWorld!"),
                    "Hello, \nWorld!",
                    2
                ),
                Token::new(tokens::TokenType::Less, "<", 2),
            ]
        );
    }
}
