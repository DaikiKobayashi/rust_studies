use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct NewScore {
    name: Option<String>,
    score: u64,
}

#[post("/saveScore")]
async fn post_new_score(new_score: web::Json<NewScore>) -> impl Responder {
    println!("new score");
    println!("{:?}", new_score);
    HttpResponse::Ok().body("ok")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new()
        .service(post_new_score))
            .bind("127.0.0.1:8080")?
            .run()
            .await
}