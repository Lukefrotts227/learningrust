use actix_files as fs;
use actix_web::{App, HttpServer, Responder, web, HttpResponse};
use tera::Tera;
use std::env; 

async fn index(tera: web::Data<Tera>) -> impl Responder {
    let mut ctx = tera::Context::new();
    ctx.insert("title", "Testing Server Render");
    ctx.insert("heading", "Hello my friends!!");
    ctx.insert("content", "Here is some content of the page to test stuff");

    match tera.render("index.html", &ctx) {
        Ok(rendered) => HttpResponse::Ok().body(rendered),
        Err(e) => {
            println!("Error rendering template: {:?}", e);
            HttpResponse::InternalServerError().body("Internal Server Error")
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize Tera with a pattern that matches all HTML files in the templates directory
    let tera = Tera::new("./static/templates/**/*.html").unwrap();

    // Print the loaded templates
    println!("Loaded templates: {:?}", tera.get_template_names().collect::<Vec<_>>());

    HttpServer::new(move || {
        App::new()
            .data(tera.clone())  // Pass Tera to the application state
            .route("/", web::get().to(index))  // Route to your handler
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

