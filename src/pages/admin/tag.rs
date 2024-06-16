use actix_web::{get, post, web, HttpResponse, Responder};
use handlebars::Handlebars;
use mongodb::Client;

use serde_json::json;

use crate::models::TagRequestModel;

#[get("/admin/tag/add")]
pub async fn get_create_tag(
    handlebars: web::Data<Handlebars<'_>>,
    _mongoc: web::Data<Client>,
) -> impl Responder {
    render_template!(
        handlebars,
        "blog-home",
        json!({"title": "Hello!", "model": "world"})
    )
}

#[post("/admin/tag/add")]
pub async fn post_create_tag(
    _model: web::Json<TagRequestModel>,
    _mongoc: web::Data<Client>,
) -> impl Responder {
    HttpResponse::Ok().json("Hello!")
}
