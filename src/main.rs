use alice::{scanner::Scanner, parser::Parser, interpreter::Interpreter};

fn main() {
    let code = r#"
        // 变量
        // type: nil, boolean, i64, f64, string, array
        let v; // nil
        let str = "hello alice.";
        let num = 114514;
        let arr = [1, 2, 3];
        let dyn_arr = [114514, "hello alice", true];
        println(dyn_arr);
        
        // 判断
        let bool = true;
        if bool {
        println("^_^");
        } else {
        println("QWQ");
        }
        
        // 循环
        for item in [1,2, v, 3,4,5] {
            println(item);
        }

        println("------------------range-----------------");
        let start = 0;
        let end = 100;
        let x = [start..end];
        for item in [x] {
            println(item);
        }
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
    let ast = match parser.parse() {
        Ok(ast) => {
            println!("{:#?}", ast);
            ast
        }
        Err(e) => {
            println!("{e}");
            panic!()
        }
    };
    let mut interpreter = Interpreter::new();
    match interpreter.interpret(ast) {
        Ok(_) => (),
        Err(e) => println!("{e}")
    }
}
