use actix_web::{get, post, web, HttpResponse, Responder};
use handlebars::Handlebars;
use mongodb::Client;

use serde_json::json;

use crate::models::PostRequestModel;

#[get("/admin/post/add")]
pub async fn get_create_blog(
    handlebars: web::Data<Handlebars<'_>>,
    _mongoc: web::Data<Client>,
) -> impl Responder {
    render_template!(
        handlebars,
        "admin/post/add.hbs",
        json!({"title": "Hello!", "model": "world"})
    )
}

#[post("/admin/post/add")]
pub async fn post_create_post(
    _model: web::Json<PostRequestModel>,
    _mongoc: web::Data<Client>,
) -> impl Responder {
    HttpResponse::Ok().json("Hello!")
}
