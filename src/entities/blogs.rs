use chrono::NaiveDate;
use mongodb::bson::{doc, oid::ObjectId};

use serde::{Deserialize, Serialize};

use crate::utils::date_ops;

use super::image::ImagePath;

#[derive(Debug, Serialize, Deserialize)]
pub struct TagEntity {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _id: Option<ObjectId>,
    pub name: String,
    pub description: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthorEntity {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _id: Option<ObjectId>,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub bio: String,
    pub photo_url: ImagePath,
    pub intro: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PostEntity {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _id: Option<ObjectId>,
    pub permalink: String,
    pub title: String,
    pub subtitle: String,
    pub kicker: String,
    pub body: String,
    pub description: String,
    pub keywords: String,
    pub tldr: String,
    pub publish_date: NaiveDate,
    pub modified_date: Option<NaiveDate>,
    pub hero_image: ImagePath,
    pub authors: Vec<String>,
    pub tags: Vec<String>,
}

impl TagEntity {
    pub fn new() -> Self {
        TagEntity {
            _id: None,
            name: "not-set".to_string(),
            description: "not-set".to_string(),
        }
    }
}

impl Default for TagEntity {
    fn default() -> Self {
        Self::new()
    }
}

impl AuthorEntity {
    fn new() -> Self {
        AuthorEntity {
            _id: None,
            first_name: "not-set".to_string(),
            email: "not-set".to_string(),
            bio: "not-set".to_string(),
            photo_url: ImagePath::new("not-set".to_string(), "not-set".to_string()),
            intro: "not-set".to_string(),
            last_name: "not-set".to_string(),
        }
    }
}

impl Default for AuthorEntity {
    fn default() -> Self {
        Self::new()
    }
}

impl PostEntity {
    fn new() -> Self {
        PostEntity {
            _id: None,
            permalink: "not-set".to_string(),
            title: "not-set".to_string(),
            subtitle: "not-set".to_string(),
            kicker: "not-set".to_string(),
            body: "not-set".to_string(),
            description: "not-set".to_string(),
            keywords: "not-set".to_string(),
            tldr: "not-set".to_string(),
            publish_date: date_ops::local_date(),
            modified_date: None,
            hero_image: ImagePath::new("not-set".to_string(), "not-set".to_string()),
            authors: vec!["not-set".to_string()],
            tags: vec!["not-set".to_string()],
        }
    }
}

impl Default for PostEntity {
    fn default() -> Self {
        Self::new()
    }
}
