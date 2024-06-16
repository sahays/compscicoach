use actix_web::{get, post, web, HttpResponse, Responder};
use handlebars::Handlebars;
use log::{debug, error, info};
use mongodb::Client;

use serde_json::json;

use crate::{
    entities::{blogs::AuthorEntity, result_types::EntityResult},
    models::{AuthorRequestModel, AuthorResponseModel},
    utils::{
        db_ops, file_ops,
        json_ops::{self, JsonOpsResult},
    },
};

#[get("/admin/author")]
pub async fn get_create_author(handlebars: web::Data<Handlebars<'_>>) -> impl Responder {
    render_template!(
        handlebars,
        "author-create",
        json!({
            "title": "Add a new Author",
            "schema": file_ops::read_file("./assets/schema/author-schema.json").unwrap()
        })
    )
}

#[get("/admin/authors")]
pub async fn get_author_list(
    handlebars: web::Data<Handlebars<'_>>,
    mongoc: web::Data<Client>,
) -> impl Responder {
    match db_ops::Database
        .get_all::<AuthorEntity>(&mongoc, "authors")
        .await
    {
        EntityResult::Success(r) => {
            debug!("{:?}", r);
            render_template!(
                handlebars,
                "author-list",
                json!({
                    "title": "All Authors",
                    "authors": AuthorResponseModel::from_vec(r)
                })
            )
        }
        EntityResult::Error(e) => {
            error!("Failed to create author: {:?}", e);
            HttpResponse::InternalServerError().body("Error listing authors")
        }
    }
}

#[post("/admin/author")]
pub async fn post_create_author(
    model: web::Json<AuthorRequestModel>,
    mongoc: web::Data<Client>,
) -> impl Responder {
    debug!("{:?}", model);

    match json_ops::validate_json_text(
        "./assets/schema/author-schema.json",
        serde_json::to_string(&model).unwrap().as_str(),
    ) {
        JsonOpsResult::Success(_) => {
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
        JsonOpsResult::Error(e) => {
            error!("Failed to validate author: {:?}", e);
            HttpResponse::InternalServerError().body("Error validating author")
        }
    }
}
