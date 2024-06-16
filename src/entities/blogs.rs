use chrono::NaiveDate;
use mongodb::bson::{doc, oid::ObjectId};

use serde::{Deserialize, Serialize};

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
    pub name: String,
    pub email: String,
    pub bio: String,
    pub photo_url: String,
    pub intro: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PostEntity {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _id: Option<ObjectId>,
    pub url: String,
    pub title: String,
    pub subtitle: String,
    pub kicker: String,
    pub body: String,
    pub description: String,
    pub keywords: String,
    pub tldr: String,
    pub timestamp: NaiveDate,
    pub hero_image_url: String,
    pub authors: Vec<String>,
    pub tags: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TagQueryModel {
    pub id: String,
    pub name: String,
    pub description: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthorQueryModel {
    pub id: String,
    pub name: String,
    pub email: String,
    pub bio: String,
    pub photo_url: String,
    pub intro: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PostQueryModel {
    pub id: String,
    pub url: String,
    pub title: String,
    pub body: String,
    pub description: String,
    pub keywords: String,
    pub tldr: String,
    pub subtitle: String,
    pub hero_image_url: String,
    pub timestamp: NaiveDate,
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
            name: "not-set".to_string(),
            email: "not-set".to_string(),
            bio: "not-set".to_string(),
            photo_url: "not-set".to_string(),
            intro: "not-set".to_string(),
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
            url: "not-set".to_string(),
            title: "not-set".to_string(),
            body: "not-set".to_string(),
            description: "not-set".to_string(),
            keywords: "not-set".to_string(),
            tldr: "not-set".to_string(),
            timestamp: NaiveDate::from_ymd_opt(1970, 1, 1).unwrap(),
            hero_image_url: "not-set".to_string(),
            subtitle: "not-set".to_string(),
            _id: None,
            authors: vec![],
            tags: vec![],
            kicker: "not-set".to_string(),
        }
    }
}

impl Default for PostEntity {
    fn default() -> Self {
        Self::new()
    }
}
