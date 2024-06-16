use actix_multipart::form::{tempfile::TempFile, MultipartForm};
use actix_web::{post, HttpResponse, Responder};
use log::debug;
use serde::{Deserialize, Serialize};

use crate::utils::random_ops;

#[derive(Debug, MultipartForm)]
struct UploadForm {
    #[multipart(limit = "2MB")]
    photo: TempFile,
}

#[derive(Deserialize, Serialize)]
pub struct PhotoResponse {
    pub path: Option<String>,
    pub filename: Option<String>,
    pub error: Option<String>,
}

#[post("/api/photos")]
pub async fn post_photos(MultipartForm(form): MultipartForm<UploadForm>) -> impl Responder {
    debug!("uploading photos");
    let filename = format!(
        "{}-{}",
        random_ops::generate_guid(12),
        form.photo.file_name.unwrap()
    );
    let path = format!("./assets/images/uploads/{}", filename);
    debug!("saving to {path}");
    match form.photo.file.persist(&path) {
        Ok(_) => HttpResponse::Ok().json(PhotoResponse {
            path: Some(format!("/assets/images/uploads/{}", filename.clone())),
            filename: Some(filename.clone()),
            error: None,
        }),
        Err(e) => HttpResponse::InternalServerError().json(PhotoResponse {
            path: None,
            filename: None,
            error: Some(format!("Error saving photo: [{e}]")),
        }),
    }
}
