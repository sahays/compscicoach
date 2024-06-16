use actix_web::{get, post, web, HttpResponse, Responder};
use handlebars::Handlebars;
use log::{debug, error, info};
use mongodb::Client;

use serde_json::json;

use crate::{
    entities::result_types::EntityResult,
    models::AuthorRequestModel,
    utils::{db_ops, file_ops},
};

#[get("/admin/author")]
pub async fn get_create_author(
    handlebars: web::Data<Handlebars<'_>>,
    _mongoc: web::Data<Client>,
) -> impl Responder {
    render_template!(
        handlebars,
        "author-create",
        json!({"title": "Add a new Author", "schema": file_ops::read_file("./assets/scripts/author-schema.json").unwrap()})
    )
}

#[post("/admin/author")]
pub async fn post_create_author(
    model: web::Json<AuthorRequestModel>,
    mongoc: web::Data<Client>,
) -> impl Responder {
    debug!("{:?}", model);

    match db_ops::Database
        .create(&mongoc, "authors", model.to())
        .await
    {
        EntityResult::Success(r) => {
            info!("Author created {:?}", r);

            HttpResponse::Ok().body("Author created")
        }
        EntityResult::Error(e) => {
            error!("Failed to create author: {:?}", e);

            HttpResponse::InternalServerError().body("Error creating author")
        }
    }
}
