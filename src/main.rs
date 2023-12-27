mod db;

use std::{ops::Deref, sync::Mutex};

use actix_web::{
    error, get, http::header::ContentType, post, web, App, HttpResponse, HttpServer, Responder,
};
use askama::Template;
use db::db::{insert_post, query_posts, Post, PostPayload};
use futures::StreamExt;

const MAX_SIZE: usize = 262_144;

#[get("/posts")]
async fn get_posts() -> impl Responder {
    let client = libsql_client::client::Client::from_env().await;
    let db_client = client.unwrap();
    let posts: Vec<Post> = query_posts(db_client).await;

    HttpResponse::Ok()
        .content_type(ContentType::json())
        .json(posts)
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
    Ok(HttpResponse::Ok()
        .content_type(ContentType::plaintext())
        .body("Sent request to server"))
}

#[derive(Template)]
#[template(path = "base.html")]
struct BaseTemplate<'a> {
    title: &'a str,
}

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate<'a> {
    _parent: &'a BaseTemplate<'a>,
}

#[derive(Template)]
#[template(path = "private.html")]
struct PrivateTemplate<'a> {
    _parent: &'a BaseTemplate<'a>,
}

impl<'a> Deref for IndexTemplate<'a> {
    type Target = BaseTemplate<'a>;

    fn deref(&self) -> &Self::Target {
        self._parent
    }
}

impl<'a> Deref for PrivateTemplate<'a> {
    type Target = BaseTemplate<'a>;

    fn deref(&self) -> &Self::Target {
        self._parent
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new()
            .service(
                web::scope("/api")
                    .service(get_posts)
                    .service(insert_new_post),
            )
            .service(web::resource("/").to(|| async {
                IndexTemplate {
                    _parent: &BaseTemplate {
                        title: "Sammy Shear",
                    },
                }
            }))
            .service(web::resource("/private").to(|| async {
                PrivateTemplate {
                    _parent: &BaseTemplate { title: "New Post" },
                }
            }))
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
