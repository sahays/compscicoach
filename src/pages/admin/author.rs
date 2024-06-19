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
        .find_all::<AuthorEntity>(&mongoc, "authors")
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
            error!("Failed to list author: {:?}", e);
            HttpResponse::InternalServerError().body("Error listing authors")
        }
    }
}

#[get("/admin/author/{id}")]
pub async fn get_edit_author(
    handlebars: web::Data<Handlebars<'_>>,
    mongoc: web::Data<Client>,
    path: web::Path<String>,
) -> impl Responder {
    let author_id = path.into_inner();
    match db_ops::Database
        .find::<AuthorEntity>(&mongoc, "authors", author_id)
        .await
    {
        EntityResult::Success(r) => {
            debug!("{:?}", r);
            render_template!(
                handlebars,
                "author-edit",
                json!({
                    "title": "Edit Author",
                    "author": AuthorResponseModel::from(r),
                    "schema": file_ops::read_file("./assets/schema/author-schema.json").unwrap()
                })
            )
        }
        EntityResult::Error(e) => {
            error!("Failed to find author: {:?}", e);
            HttpResponse::BadRequest().body("Error finding author")
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
                    HttpResponse::BadRequest().body("Error creating author")
                }
            }
        }
        JsonOpsResult::Error(e) => {
            error!("Failed to validate author: {:?}", e);
            HttpResponse::BadRequest().body("Error validating author")
        }
    }
}

#[post("/admin/author/{id}")]
pub async fn post_edit_author(
    mongoc: web::Data<Client>,
    path: web::Path<String>,
    model: web::Json<AuthorRequestModel>,
) -> impl Responder {
    let author_id = path.into_inner();
    debug!("{:?}", model);

    match json_ops::validate_json_text(
        "./assets/schema/author-schema.json",
        serde_json::to_string(&model).unwrap().as_str(),
    ) {
        JsonOpsResult::Success(_) => {
            match db_ops::Database
                .update(&mongoc, "authors", model.to(), author_id)
                .await
            {
                EntityResult::Success(r) => {
                    info!("Author updated {:?}", r);
                    HttpResponse::Ok().body("Author updated")
                }
                EntityResult::Error(e) => {
                    error!("Failed to update author: {:?}", e);
                    HttpResponse::BadRequest().body("Error updating author")
                }
            }
        }
        JsonOpsResult::Error(e) => {
            error!("Failed to validate author: {:?}", e);
            HttpResponse::BadRequest().body("Error validating author")
        }
    }
}
