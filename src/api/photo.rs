use std::{ffi::OsStr, fs::File, io::Write, path::Path};

use actix_multipart::Multipart;
use actix_web::{post, HttpResponse, Responder};
use futures::StreamExt;
use image::{imageops::FilterType, GenericImageView};
use serde::{Deserialize, Serialize};

use crate::{
    entities::image::{ImagePath, ImageSize, UPLOADS_DIR},
    utils::random_ops,
};

#[derive(Deserialize, Serialize)]
pub struct PhotoResponse {
    pub path: Option<String>,
    pub filename: Option<String>,
    pub error: Option<String>,
}

#[post("/api/photo")]
pub async fn post_photo(mut payload: Multipart) -> impl Responder {
    if let Some(item) = payload.next().await {
        let mut field = item.unwrap();
        let content_disposition = field.content_disposition().clone();
        let filename = content_disposition.get_filename().unwrap();
        let extension = Path::new(filename)
            .extension()
            .and_then(OsStr::to_str)
            .unwrap_or("");

        // Generate a unique key
        let unique_key = random_ops::generate_guid(16);

        // Create the file path with the unique key and original extension
        let file_path = format!("{}/{}.{}", UPLOADS_DIR, unique_key, extension);

        // Create a new file and write the contents
        let mut f = File::create(&file_path).unwrap();

        // Use a while-let loop to read chunks from the field
        while let Some(chunk) = field.next().await {
            let data = chunk.unwrap();
            f.write_all(&data).unwrap();
        }

        generate_photo_sizes(&file_path);

        // Return the unique key and extension as JSON
        return HttpResponse::Ok().json((unique_key, extension.to_string()));
    }

    HttpResponse::BadRequest().body("No file uploaded")
}

#[post("/api/photos")]
pub async fn post_photos(mut payload: Multipart) -> impl Responder {
    let mut response = Vec::new();

    while let Some(item) = payload.next().await {
        let mut field = item.unwrap();
        let content_disposition = field.content_disposition().clone();
        let filename = content_disposition.get_filename().unwrap();
        let extension = Path::new(filename)
            .extension()
            .and_then(OsStr::to_str)
            .unwrap_or("");

        // Generate a unique key
        let unique_key = random_ops::generate_guid(16);

        // Create the file path with the unique key and original extension
        let file_path = format!("{}/{}.{}", UPLOADS_DIR, unique_key, extension);

        // Create a new file and write the contents
        let mut f = File::create(&file_path).unwrap();
        while let Some(chunk) = field.next().await {
            let data = chunk.unwrap();
            f.write_all(&data).unwrap();
        }

        generate_photo_sizes(&file_path);

        // Append the unique key and extension to the response
        response.push((unique_key, extension.to_string()));
    }

    // Return the response as JSON
    HttpResponse::Ok().json(response)
}

pub fn generate_photo_sizes(file_path: &str) {
    // Extract key and extension from the file path
    let path = Path::new(file_path);
    let key = path.file_stem().unwrap().to_str().unwrap();
    let extension = path.extension().unwrap().to_str().unwrap();

    // Iterate over all image sizes and create resized photos
    for size in ImageSize::all_sizes() {
        resize_photo(size, key, extension);
    }
}

pub fn resize_photo(size: ImageSize, key: &str, extension: &str) {
    let path = ImagePath::new(key.to_string(), extension.to_string());
    let original_path = path.original_path();
    let resized_path = path.from(&size);
    let (width, height) = size.dimensions();

    // do not proceed if the resized image already exists
    if Path::new(&resized_path).exists() {
        return;
    }

    // Open the original image
    let img = image::open(original_path).unwrap();

    // Calculate the new dimensions while maintaining the aspect ratio
    let (orig_width, orig_height) = img.dimensions();
    let aspect_ratio = orig_width as f32 / orig_height as f32;
    let (new_width, new_height) = if width as f32 / height as f32 > aspect_ratio {
        ((height as f32 * aspect_ratio) as u32, height)
    } else {
        (width, (width as f32 / aspect_ratio) as u32)
    };

    // Resize the image
    let resized_img = img.resize(new_width, new_height, FilterType::Lanczos3);

    // Save the resized image to disk
    let mut output = File::create(&resized_path).unwrap();
    resized_img
        .write_to(
            &mut output,
            image::ImageFormat::from_extension(extension).unwrap(),
        )
        .unwrap();
}
