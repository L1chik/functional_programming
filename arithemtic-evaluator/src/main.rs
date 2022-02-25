use std::io;

mod parsemath;
mod errors;

use parsemath::ast;
use parsemath::parser::Parser;
use crate::errors::ParseError;


fn evaluate(expr: String) -> Result<f64, ParseError> {
    let expr = expr.split_whitespace().collect::<String>();

    let mut math_parser = Parser::new(&expr)?;
    let ast = math_parser.parse()?;
    println!("The generated node tree is: {:?}", ast);

    Ok(ast::eval(ast)?)
}

fn main() {
    println!("Supported operations: Add, Subtract, Multiply, Divide, Power(^)!");
    println!("Enter expression: ");

    loop {
        let mut input = String::new();

        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                match evaluate(input) {
                    Ok(val) => print!("Result: {}\n", val),
                    Err(_) => {
                        print!("Error. Try to enter valid expression\n");
                    }
                };
            }

            Err(e) => println!("Error: {}", e),
        }
    }
}
