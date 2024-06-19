use std::vec;

use chrono::NaiveDate;
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

use crate::entities::{
    blogs::{AuthorEntity, PostEntity, TagEntity},
    image::ImagePath,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthorRequestModel {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub bio: String,
    pub photo_url: String,
    pub intro: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthorResponseModel {
    pub id: String,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub bio: String,
    pub photo_url: String,
    pub intro: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TagRequestModel {
    pub name: String,
    pub description: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TagResponseModel {
    pub id: String,
    pub name: String,
    pub description: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PostResponseModel {
    pub title: String,
    pub subtitle: String,
    pub body: String,
    pub description: String,
    pub keywords: String,
    pub tldr: String,
    pub hero_image: String,
    pub publish_date: NaiveDate,
    pub modified_date: Option<NaiveDate>,
    pub authors: Vec<AuthorResponseModel>,
    pub tags: Vec<TagResponseModel>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PostRequestModel {
    pub kicker: String,
    pub title: String,
    pub subtitle: String,
    pub body: String,
    pub description: String,
    pub keywords: String,
    pub tldr: String,
    pub hero_image: String,
    pub publish_date: NaiveDate,
    pub author: Vec<String>,
    pub tags: Vec<String>,
}

impl AuthorRequestModel {
    pub fn to(&self) -> AuthorEntity {
        AuthorEntity {
            _id: None,
            first_name: self.first_name.clone(),
            email: self.email.to_string(),
            bio: self.bio.to_string(),
            photo_url: ImagePath::from_string(self.photo_url.as_str()),
            intro: self.intro.to_string(),
            last_name: self.last_name.clone(),
        }
    }
}

impl AuthorResponseModel {
    pub fn from(entity: AuthorEntity) -> Self {
        AuthorResponseModel {
            id: entity._id.unwrap().to_string(),
            first_name: entity.first_name,
            last_name: entity.last_name,
            email: entity.email,
            bio: entity.bio,
            photo_url: entity.photo_url.profile_r_path(),
            intro: entity.intro,
        }
    }

    pub fn from_vec(entities: Vec<AuthorEntity>) -> Vec<Self> {
        let mut authors = vec![];

        for entity in entities {
            authors.push(AuthorResponseModel::from(entity));
        }

        authors
    }
}

impl TagRequestModel {
    pub fn to(&self) -> TagEntity {
        TagEntity {
            _id: None,
            name: self.name.to_string(),
            description: self.description.to_string(),
        }
    }
}

impl TagResponseModel {
    pub fn from(entity: TagEntity) -> Self {
        TagResponseModel {
            id: entity._id.unwrap().to_string(),
            name: entity.name.to_string(),
            description: entity.description.to_string(),
        }
    }

    pub fn from_vec(entities: Vec<TagEntity>) -> Vec<Self> {
        let mut tags = vec![];

        for entity in entities {
            tags.push(TagResponseModel::from(entity));
        }

        tags
    }
}

impl PostResponseModel {
    pub fn from(entity: PostEntity) -> Self {
        PostResponseModel {
            title: entity.title.to_string(),
            subtitle: entity.subtitle.to_string(),
            body: entity.body.to_string(),
            description: entity.description.to_string(),
            keywords: entity.keywords.to_string(),
            tldr: entity.tldr.to_string(),
            hero_image: entity.hero_image.to_string(),
            publish_date: entity.publish_date,
            modified_date: entity.modified_date,
            authors: vec![],
            tags: vec![],
        }
    }

    pub fn from_vec(entities: Vec<PostEntity>) -> Vec<Self> {
        let mut posts = vec![];

        for entity in entities {
            posts.push(PostResponseModel::from(entity));
        }

        posts
    }

    pub fn all(
        posts: Vec<PostEntity>,
        authors: Vec<AuthorEntity>,
        tags: Vec<TagEntity>,
    ) -> Vec<PostResponseModel> {
        let mut post_responses = Vec::new();

        for post in posts {
            let post_authors: Vec<AuthorResponseModel> = post
                .authors
                .iter()
                .filter_map(|author_id| {
                    authors
                        .iter()
                        .find(|author| {
                            author
                                ._id
                                .unwrap()
                                .eq(&ObjectId::parse_str(author_id).unwrap())
                        })
                        .map(|author| AuthorResponseModel {
                            first_name: author.first_name.clone(),
                            last_name: author.last_name.clone(),
                            email: author.email.clone(),
                            bio: author.bio.clone(),
                            photo_url: author.photo_url.to_string(),
                            intro: author.intro.clone(),
                            id: author._id.unwrap().to_string(),
                        })
                })
                .collect();

            let post_tags: Vec<TagResponseModel> = post
                .tags
                .iter()
                .filter_map(|tag_id| {
                    tags.iter()
                        .find(|tag| tag._id.unwrap().eq(&ObjectId::parse_str(tag_id).unwrap()))
                        .map(|tag| TagResponseModel {
                            name: tag.name.clone(),
                            description: tag.description.clone(),
                            id: tag._id.unwrap().to_string(),
                        })
                })
                .collect();

            let post_response = PostResponseModel {
                title: post.title.clone(),
                subtitle: post.subtitle.clone(),
                body: post.body.clone(),
                description: post.description.clone(),
                keywords: post.keywords.clone(),
                tldr: post.tldr.clone(),
                hero_image: post.hero_image.to_string(),
                publish_date: post.publish_date,
                modified_date: post.modified_date,
                authors: post_authors,
                tags: post_tags,
            };

            post_responses.push(post_response);
        }

        post_responses
    }
}

impl PostRequestModel {
    pub fn to(&self) -> PostEntity {
        PostEntity {
            permalink: "not-set".to_string(),
            title: self.title.to_string(),
            body: self.body.to_string(),
            description: self.description.to_string(),
            keywords: self.keywords.to_string(),
            tldr: self.tldr.to_string(),
            hero_image: ImagePath::from_string(self.hero_image.as_str()),
            subtitle: self.subtitle.to_string(),
            _id: None,
            authors: self.author.clone(),
            tags: self.tags.clone(),
            kicker: self.kicker.to_string(),
            publish_date: self.publish_date,
            modified_date: None,
        }
    }
}
