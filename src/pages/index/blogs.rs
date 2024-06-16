use actix_web::{get, web, HttpResponse, Responder};
use handlebars::Handlebars;
use mongodb::Client;

use serde_json::json;

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
