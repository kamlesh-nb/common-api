use serde::de::DeserializeOwned;
use serde::Serialize;
use futures::StreamExt;
use std::{error::Error, marker::PhantomData};

use azure_data_cosmos::{
  clients::{CollectionClient, CosmosClient, CosmosOptions},
  prelude::{AuthorizationToken, Query},
  CosmosEntity
};

use async_trait::async_trait;

use super::Repository;


#[derive(Debug, Clone)]
pub struct Cosmos<V> {
    key: String,
    account: String,
    db: String,
    container: String,
    _marker: PhantomData<V>,
}

#[allow(unused)]
impl<T> Cosmos<T> {

  pub fn new(key: String, account: String, db: String, container: String) -> Self {
    Self{
      key, 
      account, 
      db, 
      container,
      _marker: PhantomData
    }
  }

  async fn create_client(&self) -> CollectionClient {
    let authorization_token = AuthorizationToken::primary_from_base64(&self.key).unwrap();
    let client = CosmosClient::new(
        self.account.clone(),
        authorization_token,
        CosmosOptions::default(),
    );

    let database = client.database_client(self.db.clone());
    database.collection_client(self.container.clone())
  }

}

#[async_trait]
impl<T> Repository<T> for Cosmos<T> 
where T: Serialize + DeserializeOwned + Clone + CosmosEntity + 'static + Send + Sync
{

    async fn create(&self, value: T) -> Result<T, Box<dyn Error + Send + Sync>> {
        let client = self.create_client().await;
      client
          .create_document(value.clone())
          .into_future()
          .await?;
      Ok(value)
    }

    async fn update(&self, value: T, id: String) -> Result<T, Box<dyn Error + Send + Sync>> {
      let client = self.create_client().await;
      client
          .document_client(id.clone(), &id)
          .unwrap()
          .replace_document(value.clone())
          .into_future()
          .await
          .unwrap();
  
      Ok(value)
    }

    async fn delete(&self, id: String) -> Result<String, Box<dyn Error + Send + Sync>> {
      let client = self.create_client().await;
      client
          .document_client(id.clone(), &id)
          .unwrap()
          .delete_document()
          .into_future()
          .await
          .unwrap();
  
      Ok(id)
    }

    async fn find_all(&self) -> Result<Vec<T>, Box<dyn Error + Send + Sync>> {
      let query = format!("SELECT * FROM c");
      let query_obj = Query::new(query);
      let client = self.create_client().await;
      let query_result = client
          .query_documents(query_obj)
          .query_cross_partition(true)
          .into_stream::<T>()
          .next()
          .await
          .unwrap()
          .unwrap()
          .into_documents()
          .unwrap()
          .results;

      let mut rows: Vec<T> = Vec::new();
      for document in query_result {
          rows.push(document.result)
      }
      Ok(rows)
    }

    async fn find_by_id(&self, id: String) -> Result<Vec<T>, Box<dyn Error + Send + Sync>> {
      let query = format!("SELECT * FROM c where c.id = '{}'", id);
      let query_obj = Query::new(query);
      let client = self.create_client().await;
      let query_result = client
          .query_documents(query_obj)
          .query_cross_partition(true)
          .into_stream::<T>()
          .next()
          .await
          .unwrap()
          .unwrap()
          .into_documents()
          .unwrap()
          .results;

      let mut rows: Vec<T> = Vec::new();
      for document in query_result {
          rows.push(document.result)
      }
      Ok(rows)
    }

    async fn find_by_query(&self, query: String) -> Result<Vec<T>, Box<dyn Error + Send + Sync>> {
      let query_obj = Query::new(query);
      let client = self.create_client().await;
      let query_result = client
          .query_documents(query_obj)
          .query_cross_partition(true)
          .into_stream::<T>()
          .next()
          .await
          .unwrap()
          .unwrap()
          .into_documents()
          .unwrap()
          .results;

      let mut rows: Vec<T> = Vec::new();
      for document in query_result {
          rows.push(document.result)
      }
      Ok(rows)
    }

}