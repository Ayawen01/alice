use alice::{scanner::Scanner, parser::Parser, interpreter::Interpreter};

fn main() {
    let code = r#"
        let arr = [1,2,3];
        println(arr);
        let dyn_arr = [114514, "hello world", true, arr];
        println(dyn_arr);
        //println([1 > 2,[[[[[1],[1]],[4]]]]]);
        let v = 1;
        v = nil;
        println(["a" == "a", 114514.1919]);
        println(v == nil);
        println("------------------------------------------");
        
        for item in [1,2,3,4,5, nil] {
            println(item);
            println("------------------------------------------");
        }

        println("------------------------------------------");
        println("------------------------------------------");

        for item in dyn_arr {
            println(item);
            println("------------------------------------------");
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
