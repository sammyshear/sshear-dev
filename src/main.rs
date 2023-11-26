mod db;

use actix_web::{get, HttpServer, App, web, HttpResponse, Responder, http::header::ContentType, post, error};
use actix_files as fs;
use db::db::{query_posts, Post, insert_post, PostPayload};
use futures::StreamExt;

const MAX_SIZE: usize = 262_144;

#[get("/posts")]
async fn get_posts() -> impl Responder {
    let client = libsql_client::client::Client::from_env().await;
    let db_client = client.unwrap();
    let posts: Vec<Post> = query_posts(db_client).await;

    HttpResponse::Ok().content_type(ContentType::json()).json(posts)
}

#[post("/posts/new")]
async fn insert_new_post(mut payload: web::Payload) -> Result<HttpResponse, error::Error> {
    let client = libsql_client::client::Client::from_env().await;
    let db_client = client.unwrap();
    let mut body = web::BytesMut::new();
    while let Some(chunk) = payload.next().await {
        let chunk = chunk?;
        // limit max size of in-memory payload
        if (body.len() + chunk.len()) > MAX_SIZE {
            return Err(error::ErrorBadRequest("overflow"));
        }
        body.extend_from_slice(&chunk);
    }
    let post_obj = serde_json::from_slice::<PostPayload>(&body).unwrap();
    insert_post(post_obj, db_client).await;
    Ok(HttpResponse::Ok().content_type(ContentType::plaintext()).body("Inserted new post"))
}

#[actix_web::main]
async fn main() -> std::io::Result<()>{
    HttpServer::new(|| {
        App::new().service(fs::Files::new("/static", "static").index_file("index.html")).service(web::scope("/api").service(get_posts).service(insert_new_post))
    }).bind(("127.0.0.1", 8080))?.run().await
}
