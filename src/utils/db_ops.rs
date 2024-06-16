use std::str::FromStr;

use futures::stream::TryStreamExt;
use mongodb::{
    bson::{self, doc, oid::ObjectId},
    Client,
};

use serde::{de::DeserializeOwned, Serialize};

use crate::{
    entities::result_types::{DatabaseErrorType, EntityResult, SuccessResultType},
    utils::environ::Environ,
};

pub struct Database;

impl Database {
    pub async fn create<T>(
        &self,
        client: &Client,
        collection_name: &str,
        entity: T,
    ) -> EntityResult<SuccessResultType>
    where
        T: Serialize + Unpin + Send + Sync,
    {
        let collection = client
            .database(Environ::default().db_name.as_str())
            .collection::<T>(collection_name);

        match collection.insert_one(entity, None).await {
            Ok(result) => {
                EntityResult::Success(SuccessResultType::Created(result.inserted_id.to_string()))
            }
            Err(e) => EntityResult::Error(DatabaseErrorType::MutationError(
                format!("Error creating document in {}", collection_name),
                e.to_string(),
            )),
        }
    }

    pub async fn get_all<T>(&self, client: &Client, collection_name: &str) -> EntityResult<Vec<T>>
    where
        T: DeserializeOwned + Unpin + Send + Sync,
    {
        let collection = client
            .database(Environ::default().db_name.as_str())
            .collection::<T>(collection_name);

        match collection.find(doc! {}, None).await {
            Ok(mut cursor) => {
                let mut entities = vec![];
                loop {
                    match cursor.try_next().await {
                        Ok(Some(entity)) => entities.push(entity),
                        Ok(None) => break,
                        Err(e) => {
                            return EntityResult::Error(DatabaseErrorType::QueryError(
                                format!("Error getting documents from {}", collection_name),
                                e.to_string(),
                            ))
                        }
                    }
                }
                EntityResult::Success(entities)
            }
            Err(e) => EntityResult::Error(DatabaseErrorType::QueryError(
                format!("Error getting documents from {}", collection_name),
                e.to_string(),
            )),
        }
    }

    pub async fn find<T>(
        &self,
        client: &Client,
        collection_name: &str,
        id: String,
    ) -> EntityResult<T>
    where
        T: DeserializeOwned + Unpin + Send + Sync,
    {
        let collection = client
            .database(Environ::default().db_name.as_str())
            .collection::<T>(collection_name);

        let object_id = ObjectId::from_str(id.as_str()).unwrap();

        match collection.find_one(doc! {"_id": object_id}, None).await {
            Ok(cursor) => match cursor {
                Some(r) => EntityResult::Success(r),
                None => EntityResult::Error(DatabaseErrorType::NotFound(
                    format!("Error finding document {} in {}", id, collection_name),
                    "Document not found".to_string(),
                )),
            },
            Err(e) => EntityResult::Error(DatabaseErrorType::QueryError(
                format!("Error getting documents from {}", collection_name),
                e.to_string(),
            )),
        }
    }

    pub async fn update<T>(
        &self,
        client: &Client,
        collection_name: &str,
        entity: T,
        id: String,
    ) -> EntityResult<SuccessResultType>
    where
        T: Serialize + Unpin + Send + Sync,
    {
        let collection = client
            .database(Environ::default().db_name.as_str())
            .collection::<T>(collection_name);

        let object_id = ObjectId::from_str(id.as_str()).unwrap();

        let filter = doc! { "_id": object_id };
        let update = doc! { "$set": bson::to_bson(&entity).unwrap() };

        match collection.update_one(filter, update, None).await {
            Ok(result) => {
                if result.matched_count > 0 {
                    EntityResult::Success(SuccessResultType::Updated(id.clone()))
                } else {
                    EntityResult::Error(DatabaseErrorType::MutationError(
                        format!("No document found to update in {}", collection_name),
                        "No matching document found".to_string(),
                    ))
                }
            }
            Err(e) => EntityResult::Error(DatabaseErrorType::MutationError(
                format!("Error updating document in {}", collection_name),
                e.to_string(),
            )),
        }
    }
}
