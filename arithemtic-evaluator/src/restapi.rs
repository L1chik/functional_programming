mod parsemath;
mod errors;

use parsemath::ast;
use parsemath::parser::Parser;
use crate::errors::ParseError;
use actix_web::{App, HttpRequest, HttpServer, Responder, web};

async fn evaluate(expr: String) -> Result<f64, ParseError> {
    let expr = expr.split_whitespace().collect::<String>();

    let mut math_parser = Parser::new(&expr)?;
    let ast = math_parser.parse()?;
    println!("The generated node tree is: {:?}", ast);

    Ok(ast::eval(ast)?)
}

async fn calc(req: HttpRequest) -> impl Responder {
    let input = String::from(req.match_info().get("expr").unwrap_or("Invalid expression"));

    match evaluate(input).await {
        Ok(val) => format!("Result: {}\n", val),
        Err(_) => {
            format!("Error. Try to enter valid expression\n")
        }
    }
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/{expr}", web::get().to(calc))
    })
        .bind("127.0.0.1:8999")?
        .run()
        .await
}