mod db;

use actix_web::{get, HttpServer, App, web, HttpResponse, Responder, http::header::ContentType};
use actix_files as fs;
use db::db::query_posts;


#[get("/posts")]
async fn get_posts() -> impl Responder {
    let client = libsql_client::client::Client::from_env().await;
    let db_client = client.unwrap();
    let posts = query_posts(db_client).await;
    HttpResponse::Ok().content_type(ContentType::json()).body(format!("{{ {:?} }}", posts))
}

#[actix_web::main]
async fn main() -> std::io::Result<()>{
    HttpServer::new(|| {
        App::new().service(fs::Files::new("/static", "static").index_file("index.html")).service(web::scope("/api").service(get_posts))
    }).bind(("127.0.0.1", 8080))?.run().await
}
