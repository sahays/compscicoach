use actix_web::{get, web, HttpResponse, Responder};
use handlebars::Handlebars;
use log::error;
use mongodb::Client;

use pulldown_cmark::{html, Parser};
use serde_json::json;

use crate::{
    entities::{
        blogs::{AuthorEntity, PostEntity, TagEntity},
        result_types::EntityResult,
    },
    models::PostResponseModel,
    utils::db_ops::Database,
};

#[get("/")]
pub async fn get_posts(
    handlebars: web::Data<Handlebars<'_>>,
    _mongoc: web::Data<Client>,
) -> impl Responder {
    render_template!(
        handlebars,
        "blog-home",
        json!({"title": "Hello!", "model": "world"})
    )
}

#[get("/blogs/{permalink}")]
pub async fn get_post(
    handlebars: web::Data<Handlebars<'_>>,
    mongoc: web::Data<Client>,
    path: web::Path<String>,
) -> impl Responder {
    let permalink = path.into_inner();

    let authors_collection = Database::new(&mongoc, "authors");
    let authors = match Database::find_all::<AuthorEntity>(authors_collection).await {
        EntityResult::Success(r) => r,
        EntityResult::Error(e) => {
            error!("Failed to find authors: {:?}", e);
            return HttpResponse::InternalServerError().body("Error finding authors");
        }
    };

    let tags_collection = Database::new(&mongoc, "tags");
    let tags = match Database::find_all::<TagEntity>(tags_collection).await {
        EntityResult::Success(r) => r,
        EntityResult::Error(e) => {
            error!("Failed to find tags: {:?}", e);
            return HttpResponse::InternalServerError().body("Error finding tags");
        }
    };

    let collection = Database::new(&mongoc, "posts");
    let post = match Database::find_by::<PostEntity, String>(
        collection,
        String::from("permalink"),
        permalink,
    )
    .await
    {
        EntityResult::Success(r) => r,
        EntityResult::Error(e) => {
            error!("Failed to find posts: {:?}", e);
            return HttpResponse::InternalServerError().body("Error finding posts");
        }
    };

    let model = PostResponseModel::combine(post, authors, tags);
    let parser = Parser::new(&model.body);
    // Allocate a string for the HTML output
    let mut html_output = String::new();
    // Convert markdown to HTML
    html::push_html(&mut html_output, parser);
    render_template!(
        handlebars,
        "blog-post",
        json!({
            "title": model.title,
            "description":  model.description,
            "body": html_output,
            "model": model
        })
    )
}
