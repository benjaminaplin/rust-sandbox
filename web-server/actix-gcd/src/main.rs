use actix_web::{web, App, HttpResponse, HttpServer};
use serde::Deserialize;

#[actix_web::main]
async fn main() {
    let server = HttpServer::new(|| {
        App::new()
            .route("/triangle", web::get().to(get_triangle))
    });

    println!("Serving on http://localhost:5001...");
    server
        .bind("127.0.0.1:5001").expect("error binding server to address")
        .run()
        .await
        .expect("error running server");
}

fn fold_tri(n: i32) -> i32 {
    (1..=n).fold(0, |sum, item| sum + item)
}

#[derive(Deserialize)]
struct TriangleQuery {
    n: i32,
}

async fn get_triangle(query: web::Query<TriangleQuery>) -> HttpResponse {
    let n = query.n;
    let sum = fold_tri(n);
    HttpResponse::Ok()
        .content_type("application/json")
        .body(
            sum.to_string()
        )
}
