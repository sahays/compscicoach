use std::fmt::Display;

use serde::{Deserialize, Serialize};

pub const UPLOADS_DIR: &str = "/assets/images/uploads";

pub enum ImageSize {
    Original,
    Thumbnail,
    Profile,
    Wide,
    Hero,
}

impl ImageSize {
    pub fn dimensions(&self) -> (u32, u32) {
        match self {
            ImageSize::Thumbnail => (150, 150),
            ImageSize::Profile => (300, 300),
            ImageSize::Wide => (800, 800 * 9 / 16),
            ImageSize::Hero => (1260, 1260 * 9 / 16),
            ImageSize::Original => (0, 0),
        }
    }

    pub fn all_sizes() -> Vec<ImageSize> {
        vec![
            ImageSize::Thumbnail,
            ImageSize::Profile,
            ImageSize::Wide,
            ImageSize::Hero,
        ]
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ImagePath {
    pub key: String,
    pub extension: String,
}

impl Display for ImagePath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.original_path())
    }
}

impl ImagePath {
    pub fn new(key: String, extension: String) -> Self {
        ImagePath { key, extension }
    }

    pub fn original_path(&self) -> String {
        format!("{}/{}.{}", UPLOADS_DIR, self.key, self.extension)
    }

    pub fn thumbnail_path(&self) -> String {
        format!("{}/{}-thumb.{}", UPLOADS_DIR, self.key, self.extension)
    }

    pub fn profile_path(&self) -> String {
        format!("{}/{}-profile.{}", UPLOADS_DIR, self.key, self.extension)
    }

    pub fn wide_path(&self) -> String {
        format!("{}/{}-wide.{}", UPLOADS_DIR, self.key, self.extension)
    }

    pub fn hero_path(&self) -> String {
        format!("{}/{}-hero.{}", UPLOADS_DIR, self.key, self.extension)
    }

    pub fn from(&self, size: &ImageSize) -> String {
        match size {
            ImageSize::Thumbnail => self.thumbnail_path(),
            ImageSize::Profile => self.profile_path(),
            ImageSize::Wide => self.wide_path(),
            ImageSize::Hero => self.hero_path(),
            ImageSize::Original => self.original_path(),
        }
    }

    pub fn from_string(path: &str) -> Self {
        let parts: Vec<&str> = path.split('/').collect();
        let file_name: Vec<&str> = parts[parts.len() - 1].split('.').collect();
        let key_parts: Vec<&str> = file_name[0].split('-').collect();

        ImagePath {
            key: key_parts[0].to_string(),
            extension: file_name[1].to_string(),
        }
    }
}
