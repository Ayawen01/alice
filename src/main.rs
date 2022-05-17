use alice::{scanner::Scanner, parser::Parser};

fn main() {
    let code = r#"
    let v; // nil
let str = "hello alice.";
let num = 114514;
//let arr = [1, 2, 3];
//let dyn_arr = [114514, "hello alice", true];
let x = ([1,2,3,]);
    "#;
    let mut scanner = Scanner::new(code.to_string().into_bytes());
    let r = scanner.scan_tokens();
    let tokens = match r {
        Ok(tokens) => {
            tokens.iter().for_each(|token| {
                println!("{:?}", token);
            });
            tokens
        },
        Err(errors) => {
            errors.iter().for_each(|e| println!("{e}"));
            panic!()
        }
    };
    let mut parser = Parser::new(tokens);
    match parser.parse() {
        Ok(ast) => println!("{:#?}", ast),
        Err(e) => println!("{e}")
    };
}
