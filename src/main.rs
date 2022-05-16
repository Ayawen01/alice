use alice::scanner::Scanner;

fn main() {
    let code = r#"
    // 变量
    // type: nil, boolean, i64, f64, string, array
    let v; // nil
    let str = "hello alice.";
    let num = 114514;
    let arr = [1, 2, 3];
    let dyn_arr = [114514, "hello alice", true];

    // 判断
    let bool = true;
    if bool {
    println("^_^");
    } else {`
    println("QWQ");
    }

    // 循环
    for item in [1,2,3,4,5] {
    println(item);
    }

    for index in [0..7] {
    println(index); // 0 -- 6
    }

    // 函数/lambda
    fn add(a, b) {
    return a + b;
    }

    let f = (str) => {
    println(str);
    };
    f("madoka");
    "#;
    let mut scanner = Scanner::new(code.to_string().into_bytes());
    let r = scanner.scan_tokens();
    match r {
        Ok(tokens) => tokens.iter().for_each(|token| {
            println!("{:?}", token);
        }),
        Err(errors) => errors.iter().for_each(|e| println!("{e}"))
    }
}
