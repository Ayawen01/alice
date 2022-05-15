use alice::scanner::Scanner;

fn main() {
    let code = r#"
        let str = "hello alice.";
    "#;
    let mut scanner = Scanner::new(code.to_string().into_bytes());
    let r = scanner.scan_tokens();
    match r {
        Ok(tokens) => tokens.iter().for_each(|token| {
            println!("{:?}", token);
        }),
        Err(error) => todo!(),
    }
}
