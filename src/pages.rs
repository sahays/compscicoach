use actix_web::{get, web, HttpResponse, Responder};
use handlebars::Handlebars;
use serde_json::json;
use sqlx::MySqlPool;

#[get("/")]
pub async fn index_page(
    handlebars: web::Data<Handlebars<'_>>,
    _pool: web::Data<MySqlPool>,
) -> impl Responder {
    render_template!(
        handlebars,
        "index",
        json!({"title": "Hello!", "model": "world"})
    )
}
