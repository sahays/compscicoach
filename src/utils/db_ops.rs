use futures::stream::TryStreamExt;
use mongodb::{bson::doc, Client};

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
}
