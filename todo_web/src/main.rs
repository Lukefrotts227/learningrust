use actix_web::{web, App, HttpServer, HttpResponse, Responder};
use rand::Rng;

async fn greet() -> impl Responder {
    let ppl = ["Ne", "An"];
    let greeting = random_from_array(&ppl);
    let refresh_interval = 5; // Auto-refresh every 5 seconds

    let html = format!(
        r#"
        <!DOCTYPE html>
        <html>
        <head>
            <meta http-equiv="refresh" content="{refresh_interval}">
            <title>Random Greeting</title>
        </head>
        <body>
            <h1>Choose: {}</h1>
        </body>
        </html>
        "#,
        greeting
    );

    // Use HttpResponse to explicitly return an HTML response
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}

fn random_from_array<'a>(a: &'a [&'a str]) -> &'a str {
    let mut rng = rand::thread_rng();
    a[rng.gen_range(0..a.len())]
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().route("/", web::get().to(greet))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
