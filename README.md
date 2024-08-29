# Crafting Interpreters - Lox Programming Language - Rust Implementation

This repository contains the Rust implementation of the Lox programming language as described in the book [Crafting Interpreters](https://craftinginterpreters.com/).

## Project Structure

The project is structured as follows:

- `src/`: Contains the source code of the Lox interpreter.
- `src/lox/`: Contains the source code of the Lox interpreter.
- `src/lox/ast.rs`: Contains the abstract syntax tree (AST) of the Lox interpreter.
- `src/lox/interpreter.rs`: Contains the interpreter of the Lox interpreter.
- `src/lox/lexer.rs`: Contains the lexer of the Lox interpreter.
- `src/lox/parser.rs`: Contains the parser of the Lox interpreter.
- `src/lox/token.rs`: Contains the token of the Lox interpreter.
- `src/main.rs`: Contains the main entry point of the Lox interpreter.
- `tests/`: Contains the tests of the Lox interpreter.
- `tests/lox/`: Contains the tests of the Lox interpreter.
- `tests/lox/interpreter.rs`: Contains the tests of the interpreter of the Lox interpreter.
- `tests/lox/lexer.rs`: Contains the tests of the lexer of the Lox interpreter.
- `tests/lox/parser.rs`: Contains the tests of the parser of the Lox interpreter.
- `tests/lox/token.rs`: Contains the tests of the token of the Lox interpreter.
- `Cargo.toml`: Contains the dependencies of the Lox interpreter.

## Installation

To install the Lox interpreter, you can use the following command:

```bash
git clone

cd lox-rust
```


## Usage

To run the Lox interpreter, you can use the following command:

```bash
cargo run
```

This will start the REPL (Read-Eval-Print Loop) where you can type in Lox code and see the output.

## Development

To run the tests, you can use the following command:

```bash
cargo test
```

To run the tests with logging, you can use the following command:

```bash

RUST_LOG=debug cargo test
```

To Lint the code, you can use the following command:

```bash
cargo clippy
```

To format the code, you can use the following command:

```bash

cargo fmt
```




## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
```

### Output

```bash
$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.01s
     Running `target/debug/lox`

Welcome to Lox!

> var a = 10;

> print a;

10
```
