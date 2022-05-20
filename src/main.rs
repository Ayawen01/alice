use std::io::Write;
use alice::{scanner::Scanner, parser::Parser, interpreter::Interpreter};

fn main() -> std::io::Result<()> {
    let args = std::env::args().collect::<Vec<String>>();

    if args.len() == 2 {
        run_file(&args[1])?;
    } else {
        run_prompt()?;
    }

    Ok(())
}

fn run_prompt() -> std::io::Result<()> {
    loop {
        let mut code = String::new();
        print!("> ");
        std::io::stdout().flush()?;

        std::io::stdin().read_line(&mut code).unwrap();

        let source = code.into_bytes();
        run(source);
    }

    Ok(())
}

fn run_file(path: &str) -> std::io::Result<()> {
    let code = String::new();

    let source = std::fs::read_to_string(path)?.into_bytes();

    run(source);

    Ok(())
}

fn run(source: Vec<u8>) {
    let mut scanner = Scanner::new(source);
    let tokens = match scanner.scan_tokens() {
        Ok(tokens) => {
            tokens.iter().for_each(|token| println!("{:?}", token));
            tokens
        },
        Err(errors) => {
            errors.iter().for_each(|e| println!("{e}"));
            return;
        }
    };

    let mut parser = Parser::new(tokens);
    let ast = match parser.parse() {
        Ok(ast) => {
            ast.iter().for_each(|node| println!("{:?}", node));
            ast
        }
        Err(e) => {
            println!("{e}");
            return;
        }
    };

    let mut interpreter = Interpreter::new();
    match interpreter.interpret(ast) {
        Ok(_) => (),
        Err(e) => println!("{e}")
    }
}